use std::collections::HashMap;

use rand::{thread_rng, Rng};
use tag_game::Agent;

use crate::world::World;

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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AgentState {
    pub tag: Tag,
    pub position: [u16; 2],
}

/// The implementation for the Tag Agent
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TagAgent;

impl Agent for TagAgent {
    type State = AgentState;
    type World = World;

    fn on_update(
        &self,
        id: u64,
        state: &mut Self::State,
        world: &Self::World,
        _population: &HashMap<u64, (Self, Self::State)>,
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

        let dx = 1;
        if rng.gen_bool(0.5) && state.position[0] < world.board.width - 1 {
            state.position[0] += dx;
        } else if state.position[0] > 0 {
            state.position[0] -= dx;
        }

        let dy = 1;
        if rng.gen_bool(0.5) && state.position[1] < world.board.height - 1 {
            state.position[1] += dy;
        } else if state.position[1] > 0 {
            state.position[1] -= dy;
        }
    }
}
