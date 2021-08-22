use std::collections::HashMap;

use rand::{thread_rng, Rng};
use tag_game::Agent;

use crate::world::{Board, TagWorld};

/// The state, if an agent is tagged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    /// The agent is currently "It"
    It,
    /// The agent recently was "It"
    Recent,
    /// The agent can be tagged by "It"
    None,
}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AgentState {
    pub tag: Tag,
    pub position: [f32; 2],
}

/// The implementation for the Tag Agent
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TagAgent;

impl Agent for TagAgent {
    type State = AgentState;
    type World = TagWorld;

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_lossless
    )]
    fn on_update(
        &self,
        id: u64,
        state: &mut Self::State,
        world: &Self::World,
        population: &HashMap<u64, (Self, Self::State)>,
    ) {
        fn run(state: &mut AgentState, board: Board, dx: f32, dy: f32) {
            state.position[0] = (state.position[0] + dx).clamp(0., board.width as f32 - 1.);
            state.position[1] = (state.position[1] + dy).clamp(0., board.height as f32 - 1.);
        }

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

        match state.tag {
            // Search an agent to tag
            Tag::It => {
                let mut nearest = (id, f32::MAX);
                // Find the nearest agent
                for (&ag_id, (_, agent)) in population {
                    if id == ag_id || agent.tag == Tag::Recent {
                        continue;
                    }
                    let [x, y] = state.position;
                    let [ag_x, ag_y] = population[&world.current_it].1.position;
                    let d = (ag_x - x).hypot(ag_y - y);
                    if d < nearest.1 {
                        nearest = (ag_id, d);
                    }
                }
                let [ag_x, ag_y] = population[&nearest.0].1.position;
                let [x, y] = state.position;
                let dx = if ag_x > x && rng.gen_bool(0.9) {
                    1.
                } else {
                    -1.
                };
                let dy = if ag_y > y && rng.gen_bool(0.9) {
                    1.
                } else {
                    -1.
                };

                run(state, world.board, dx, dy);
            }
            // Run around randomly
            Tag::Recent => {
                let dx = if rng.gen_bool(0.5) { 1. } else { -1. };
                let dy = if rng.gen_bool(0.5) { 1. } else { -1. };
                run(state, world.board, dx, dy);
            }
            // Flee from "It"
            Tag::None => {
                let [it_x, it_y] = population[&world.current_it].1.position;
                let [x, y] = state.position;
                let mut dx = if it_x < x && rng.gen_bool(0.6) {
                    1.
                } else {
                    -1.
                };
                let mut dy = if it_y < y && rng.gen_bool(0.6) {
                    1.
                } else {
                    -1.
                };

                if (it_x as f32 - x as f32).hypot(it_y as f32 - y as f32) > 20_f32 {
                    dx *= -1.;
                    dy *= -1.;
                }
                run(state, world.board, dx, dy);
            }
        }
    }
}
