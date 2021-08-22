#![allow(clippy::module_name_repetitions)]

mod agent;
mod config;
mod output;
mod world;

use std::{
    fs::File,
    io::{stdin, stdout, BufReader, Write},
    time::Instant,
};

use rand::Rng;
use termion::{event::Key, input::TermRead};

use tag_game::Simulation;

use crate::{
    agent::{AgentState, Tag, TagAgent},
    config::Config,
    output::Output,
    world::{Board, World},
};

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
    let config = Config::load()?;

    // Initialize random generator
    let mut rng = rand::thread_rng();

    // Initialize world
    let world = World {
        board: config.board,
        current_it: rng.gen_range(0..config.num_players),
        recent_it: None,
    };

    // create the simulation with the created world
    let mut simulation = Simulation::new(world);

    // create the agents
    // the world already has the information, which agent is "It" at startup
    // The agent will update the state as soon as the simulation begins
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
    check_tag(&mut simulation);
    simulation.update();
    viewer.draw_players(simulation.iter());
    stdout().flush()?;

    for c in stdin().keys() {
        match c? {
            Key::Char('q') | Key::Esc | Key::Ctrl('c' | 'd') => break,
            Key::Char('t') => {
                let start = Instant::now();
                check_tag(&mut simulation);
                simulation.update();
                let calc_time = start.elapsed();
                let start = Instant::now();
                viewer.draw_players(simulation.iter());
                let draw_time = start.elapsed();
                stdout().flush()?;
                viewer.draw_time(calc_time, draw_time)?;
            }
            Key::Left | Key::Char('h') => viewer.scroll_left(simulation.iter()),
            Key::Down | Key::Char('j') => viewer.scroll_down(simulation.iter()),
            Key::Up | Key::Char('k') => viewer.scroll_up(simulation.iter()),
            Key::Right | Key::Char('l') => viewer.scroll_right(simulation.iter()),
            _ => {}
        }
        let current_it_id = simulation.world().current_it;
        if let Some(current_it) = simulation.agent(current_it_id) {
            print!(
                " - current \"It\": {} at position ({},{})    ",
                current_it_id,
                current_it.position[0] + 1,
                current_it.position[1] + 1
            );
        }
        stdout().flush()?;
    }
    Ok(())
}
