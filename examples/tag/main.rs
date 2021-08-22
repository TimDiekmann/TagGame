//#![allow(unused)]

mod config;
mod output;
mod state;
mod world;

use rand::{thread_rng, Rng};
use std::{
    fs::File,
    io::{stdin, BufReader, Write},
};
use tag_game::{Agent, Simulation};
use termion::{event::Key, input::TermRead};

use config::Config;
use state::{AgentState, Tag};
use world::Board;

use crate::{output::Output, world::World};

/// Prints to the console as soon as an event occurs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct TagAgent;

impl Agent for TagAgent {
    type State = AgentState;
    type World = World;

    fn on_update<'sim>(
        &'sim self,
        id: u64,
        state: &'sim mut Self::State,
        world: &'sim Self::World,
        _population: impl Iterator<Item = (u64, &'sim Self::State)>,
    ) {
        if world.current_it == id {
            state.tag = Tag::It;
        } else if let Some(recent_it) = world.recent_it {
            if recent_it == id {
                state.tag = Tag::Recent;
            } else {
                state.tag = Tag::None;
            }
        }
        let mut rng = thread_rng();

        let dx = rng.gen_range(0..=1);
        let dx = 1;
        if rng.gen_bool(0.5) && state.position[0] < world.board.width - 1 {
            state.position[0] += dx;
        } else if state.position[0] > 0 {
            state.position[0] -= dx;
        }

        let dy = rng.gen_range(0..=1);
        let dy = 1;
        if rng.gen_bool(0.5) && state.position[1] < world.board.height - 1 {
            state.position[1] += dy;
        } else if state.position[1] > 0 {
            state.position[1] -= dy;
        }
        // state.position[0] = state.position[0].clamp(0, world.board.width);

        // let dy = rng.gen_range(0..=1);
        // if rng.gen_bool(0.5) {
        //     state.position[1] += dy;
        // } else {
        //     state.position[1] -= dy;
        // }
        // state.position[1] = state.position[0].clamp(0, world.board.height);
    }
}

fn distance(p: [u16; 2], q: [u16; 2]) -> f32 {
    let p1 = f32::from(p[0]);
    let p2 = f32::from(p[1]);
    let q1 = f32::from(q[0]);
    let q2 = f32::from(q[1]);
    (q1 - p1).hypot(q2 - p2)
}

fn check_tag(simulation: &mut Simulation<TagAgent>) {
    let current_it_id = simulation.world().current_it;
    let mut next_id = None;
    if let Some(current_it) = simulation.agent(current_it_id) {
        for (id, agent) in simulation.iter() {
            if id == current_it_id {
                // One can't tag themself
                continue;
            }
            if agent.tag == Tag::Recent {
                // No retag
                continue;
            }
            if distance(current_it.position, agent.position) < 3_f32 {
                next_id.replace(id);
                break;
            }
        }
    }
    if let Some(next_id) = next_id {
        let world = simulation.world_mut();
        world.recent_it = Some(current_it_id);
        world.current_it = next_id;
    }
}

fn main() -> Result<(), std::io::Error> {
    let config_file_path = std::env::current_dir()?
        .join("examples")
        .join("tag")
        .join("config.json");
    let config_file = BufReader::new(File::open(config_file_path)?);
    let config: Config = serde_json::from_reader(config_file)?;

    // Initialize random generator
    let mut rng = rand::thread_rng();

    let world = World {
        board: config.board,
        current_it: rng.gen_range(0..config.num_players),
        recent_it: None,
        rng: rng.clone(),
    };

    let mut simulation = Simulation::new(world);

    for _ in 0..config.num_players {
        let position = [
            rng.gen_range(0..config.board.width),
            rng.gen_range(0..config.board.height),
        ];

        simulation.add_agent(
            TagAgent,
            AgentState {
                tag: Tag::None,
                position,
            },
        );
    }

    let mut viewer = Output::new(config.board)?;

    viewer.screen().flush()?;

    for c in stdin().keys() {
        match c? {
            Key::Char('q') | Key::Esc | Key::Ctrl('c' | 'd') => break,
            Key::Char('t') => {
                check_tag(&mut simulation);
                simulation.update();
                viewer.draw_players(simulation.iter())?;
            }
            _ => {}
        }
        viewer.screen().flush()?;
    }
    Ok(())
}
