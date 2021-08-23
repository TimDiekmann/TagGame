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
    pub current_it: usize,
    /// The current agent id, which was recently tagged as "It"
    pub recent_it: Option<usize>,
}

impl World<TagAgent> for TagWorld {
    fn update(&mut self, agents: &mut [(TagAgent, AgentState)]) {
        // Check, if the current "It" has tagged someone in the latest tick
        if let Tag::It(Some(next)) = agents[self.current_it].1.tag {
            self.recent_it = Some(self.current_it);
            self.current_it = next;
        }
    }
}
