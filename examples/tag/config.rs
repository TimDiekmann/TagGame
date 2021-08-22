use serde::{Deserialize, Serialize};

use crate::Board;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub board: Board,
    #[serde(default = "Config::default_num_players")]
    pub num_players: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            board: Board::default(),
            num_players: 10,
        }
    }
}

impl Config {
    fn default_num_players() -> u64 {
        Self::default().num_players
    }
}
