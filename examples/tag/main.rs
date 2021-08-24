#![allow(clippy::module_name_repetitions, clippy::cast_lossless)]

mod agent;
mod config;
mod output;
mod world;

use std::{
    fs,
    io::{stdin, stdout, Write},
    time::Instant,
};

use agent::{Position, Properties, Tag};
use rand::Rng;
use termion::{event::Key, input::TermRead};

use tag_game::{LuaScriptHost, Simulation};

use crate::{
    agent::AgentState,
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
        current_it: rng.gen_range(0..config.num_players),
        recent_it: None,
    };

    // create the simulation with the created world
    let mut simulation = Simulation::<AgentState, TagWorld, LuaScriptHost>::new(world)
        .expect("Failed to create script host");

    let agent_script = fs::read_to_string("examples/tag/scripts/agent.lua")?;
    let default_agent_behavior = simulation
        .add_agent_behavior(&agent_script)
        .expect("Could not add agent behavior");

    let world_script = fs::read_to_string("examples/tag/scripts/world.lua")?;
    let default_world_behavior = simulation
        .add_world_behavior(&world_script)
        .expect("Could not add world behavior");

    // create the agents
    // the world already has the information, which agent is "It" at startup
    // The agent will update the state as soon as the simulation begins
    for _ in 0..config.num_players {
        simulation
            .add_agent(
                default_agent_behavior,
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
            )
            .expect("Could not add agent");
    }

    simulation
        .update_agent_behavior(default_agent_behavior, &agent_script)
        .expect("Could not update agent behavior");

    simulation
        .update_world_behavior(default_world_behavior, &world_script)
        .expect("Could not update world behavior");

    // create the viewer to spectate the game
    let mut viewer = Output::new(config.board)?;
    simulation.update().expect("Unable to upgrade");
    let agents = simulation.agents().expect("Could not get agents");
    viewer.draw_players(&agents);

    for c in stdin().keys() {
        match c? {
            Key::Char('q') | Key::Esc | Key::Ctrl('c' | 'd') => break,
            Key::F(5) | Key::Char('r') => {
                let agent_script = fs::read_to_string("examples/tag/scripts/agent.lua")?;
                simulation
                    .update_agent_behavior(default_agent_behavior, &agent_script)
                    .expect("Could not update agent behavior");
                let world_script = fs::read_to_string("examples/tag/scripts/world.lua")?;
                simulation
                    .update_world_behavior(default_world_behavior, &world_script)
                    .expect("Could not update world behavior");
            }
            Key::Char('t') => {
                let start = Instant::now();

                // We may skip some frames being shown
                // as terminals tend to be slow
                for _ in 0..config.step {
                    // Advance simulation by one tick
                    simulation.update().expect("Unable to upgrade");
                }

                let calc_time = start.elapsed();
                let start = Instant::now();

                // Draw players on board
                viewer.draw_players(&simulation.agents().expect("Could not get agents"));

                let draw_time = start.elapsed();
                viewer.draw_time(calc_time, draw_time, config.step);
                stdout().flush()?;
            }
            Key::Left | Key::Char('h') => viewer.scroll_left(&agents),
            Key::Down | Key::Char('j') => viewer.scroll_down(&agents),
            Key::Up | Key::Char('k') => viewer.scroll_up(&agents),
            Key::Right | Key::Char('l') => viewer.scroll_right(&agents),
            _ => {}
        }

        // Inspect some values
        let world = simulation.world().expect("Could not get world");
        let current_it_id = world.current_it;
        let current_it = &agents[current_it_id];
        print!(
            " - current \"It\": {:?} at position ({},{})    ",
            current_it_id,
            current_it.position.x + 1.,
            current_it.position.y + 1.
        );

        stdout().flush()?;
    }
    Ok(())
}
