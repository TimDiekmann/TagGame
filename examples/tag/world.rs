use serde::{Deserialize, Serialize};

/// Properties of the board of the game.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Board {
    /// The width of the board.
    #[serde(default = "Board::default_width")]
    pub width: u16,
    /// The height of the board.
    #[serde(default = "Board::default_height")]
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

impl Board {
    fn default_width() -> u16 {
        Self::default().width
    }

    fn default_height() -> u16 {
        Self::default().height
    }
}

/// Global state for the game.
pub struct World {
    /// The board used in the game
    pub board: Board,
    /// The current agent id, which is tagged as "It"
    pub current_it: u64,
    /// The current agent id, which was recently tagged as "It"
    pub recent_it: Option<u64>,
}
