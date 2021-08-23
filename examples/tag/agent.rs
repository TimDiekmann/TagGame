use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use tag_game::Agent;

use crate::world::{Board, TagWorld};

/// The state, if an agent is tagged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    /// The agent is currently "It". If the id is set, "It" has tagged
    /// a new agent, which will become "It" next tick.
    It(Option<usize>),
    /// The agent recently was "It".
    Recent,
    /// The agent can be tagged by "It".
    None,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self.x - rhs.x).mul_add(self.x - rhs.x, (self.y - rhs.y) * (self.y - rhs.y))
    }
}

/// Configuration for player properties and behaviors
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Properties {
    pub untagged_deciding: f64,
    pub tagged_deciding: f64,
    pub untagged_speed_multiplied: f32,
    pub tagged_speed_multiplied: f32,
}

/// The current State an agent.
#[derive(Clone, PartialEq, Debug)]
pub struct AgentState {
    pub tag: Tag,
    pub position: Position,
    pub properties: Properties,
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
        id: usize,
        state: &mut Self::State,
        world: &Self::World,
        population: &[(Self, Self::State)],
    ) {
        fn run(state: &mut AgentState, board: Board, dx: f32, dy: f32) {
            state.position.x = (state.position.x + dx).clamp(0., board.width as f32 - 1.);
            state.position.y = (state.position.y + dy).clamp(0., board.height as f32 - 1.);
        }

        let mut rng = thread_rng();

        // chosen by fair dice roll.
        // guaranteed to be random.
        let mut random_bool = |probability| -> bool { probability > rng.gen_range(0.0..1.0) };

        if world.current_it == id {
            state.tag = Tag::It(None);
        } else if let Some(recent_it) = world.recent_it {
            if recent_it == id {
                state.tag = Tag::Recent;
            } else {
                state.tag = Tag::None;
            }
        }

        match &mut state.tag {
            // Search an agent to tag
            Tag::It(next) => {
                let mut nearest = (id, f32::MAX);
                // Find the nearest agent
                for (ag_id, (_, agent)) in population.iter().enumerate() {
                    if id == ag_id {
                        continue;
                    }
                    if let Some(recent_it) = world.recent_it {
                        if ag_id == recent_it {
                            continue;
                        }
                    }
                    let d = state.position.distance_squared(agent.position);
                    if d < nearest.1 {
                        nearest = (ag_id, d);
                    }
                }

                if nearest.0 != id && nearest.1 < 3. {
                    next.replace(nearest.0);
                    return;
                }

                let Position { x: ag_x, y: ag_y } = population[nearest.0].1.position;
                let Position { x, y } = state.position;

                let mut dx = if ag_x > x { 1. } else { -1. };
                let mut dy = if ag_y > y { 1. } else { -1. };
                dx *= if random_bool(state.properties.tagged_deciding) {
                    1.
                } else {
                    -1.
                } * state.properties.tagged_speed_multiplied;
                dy *= if random_bool(state.properties.tagged_deciding) {
                    1.
                } else {
                    -1.
                } * state.properties.tagged_speed_multiplied;

                run(state, world.board, dx, dy);
            }
            // Run around randomly
            Tag::Recent => {
                let dx = if random_bool(0.5) { 1. } else { -1. };
                let dy = if random_bool(0.5) { 1. } else { -1. };
                run(state, world.board, dx, dy);
            }
            // Flee from "It"
            Tag::None => {
                let Position { x: it_x, y: it_y } = population[world.current_it].1.position;
                let Position { x, y } = state.position;

                let mut dx = if it_x < x { 1. } else { -1. };
                let mut dy = if it_y < y { 1. } else { -1. };
                dx *= if random_bool(state.properties.untagged_deciding) {
                    1.
                } else {
                    -1.
                } * state.properties.untagged_speed_multiplied;
                dy *= if random_bool(state.properties.untagged_deciding) {
                    1.
                } else {
                    -1.
                } * state.properties.untagged_speed_multiplied;

                if (Position { x, y }).distance_squared(Position { x: it_x, y: it_y }) > 400_f32 {
                    dx *= -1.;
                    dy *= -1.;
                }
                run(state, world.board, dx, dy);
            }
        }
    }
}
