use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tag_game::World;

use crate::agent::{AgentState, Tag, TagAgent};

/// Properties of the board of the game.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Board {
    /// The width of the board.
    pub width: u16,
    /// The height of the board.
    pub height: u16,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            width: 50,
            height: 50,
        }
    }
}

/// Global state for the game.
pub struct TagWorld {
    /// The board used in the game
    pub board: Board,
    /// The current agent id, which is tagged as "It"
    pub current_it: u64,
    /// The current agent id, which was recently tagged as "It"
    pub recent_it: Option<u64>,
}

impl World<TagAgent> for TagWorld {
    fn update(&mut self, agents: &mut HashMap<u64, (TagAgent, AgentState)>) {
        let current_it_id = self.current_it;
        let mut next_id = None;
        if let Some((_, current_it)) = agents.get(&current_it_id).copied() {
            for (&id, &mut (_, state)) in agents {
                if id == current_it_id {
                    // One can't tag themself
                    continue;
                }
                if state.tag == Tag::Recent {
                    // No retag
                    continue;
                }
                if current_it.position.distance(state.position) < 3_f32 {
                    next_id.replace(id);
                    break;
                }
            }
        }
        if let Some(next_id) = next_id {
            self.recent_it = Some(current_it_id);
            self.current_it = next_id;
        }
    }
}
