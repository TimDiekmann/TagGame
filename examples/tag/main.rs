#![allow(clippy::module_name_repetitions, clippy::cast_lossless)]

mod agent;
mod config;
mod output;
mod world;

use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};

use agent::{Position, Properties};
use rand::Rng;
use termion::{event::Key, input::TermRead};

use tag_game::{Id, Simulation};

use crate::{
    agent::{AgentState, Tag, TagAgent},
    config::Config,
    output::Output,
    world::{Board, TagWorld},
};

fn main() -> Result<(), std::io::Error> {
    let config = Config::load()?;

    // Initialize random generator
    let mut rng = rand::thread_rng();

    // Initialize world
    let world = TagWorld {
        board: config.board,
        current_it: Id(rng.gen_range(0..config.num_players)),
        recent_it: None,
    };

    // create the simulation with the created world
    let mut simulation = Simulation::new(world);

    // create the agents
    // the world already has the information, which agent is "It" at startup
    // The agent will update the state as soon as the simulation begins
    for _ in 0..config.num_players {
        simulation.add_agent(
            TagAgent,
            AgentState {
                tag: Tag::None,
                position: Position {
                    x: rng.gen_range(0. ..config.board.width as f32),
                    y: rng.gen_range(0. ..config.board.height as f32),
                },
                properties: Properties {
                    untagged_deciding: rng.gen_range(config.agents.untagged_deciding.clone()),
                    tagged_deciding: rng.gen_range(config.agents.tagged_deciding.clone()),
                    untagged_speed_multiplied: rng
                        .gen_range(config.agents.untagged_speed_multiplied.clone()),
                    tagged_speed_multiplied: rng
                        .gen_range(config.agents.tagged_speed_multiplied.clone()),
                },
            },
        );
    }

    // create the viewer to spectate the game
    let mut viewer = Output::new(config.board)?;
    simulation.update();
    viewer.draw_players(simulation.iter());
    stdout().flush()?;

    for c in stdin().keys() {
        match c? {
            Key::Char('q') | Key::Esc | Key::Ctrl('c' | 'd') => break,
            Key::Char('t') => {
                let start = Instant::now();
                for _ in 0..config.step {
                    simulation.update();
                }
                let calc_time = start.elapsed();
                let start = Instant::now();
                viewer.draw_players(simulation.iter());
                let draw_time = start.elapsed();
                stdout().flush()?;
                viewer.draw_time(calc_time, draw_time, config.step)?;
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
                current_it_id.0,
                current_it.position.x + 1.,
                current_it.position.y + 1.
            );
        }
        stdout().flush()?;
    }
    Ok(())
}
