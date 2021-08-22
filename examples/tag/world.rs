use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Board {
    #[serde(default = "Board::default_width")]
    pub width: u16,
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

pub struct World {
    pub board: Board,
    pub rng: ThreadRng,
    pub current_it: u64,
    pub recent_it: Option<u64>,
}
