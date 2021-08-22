use std::{
    fs::{self, File},
    io::{self, BufReader},
};

use serde::{Deserialize, Serialize};

use crate::Board;

/// Configuration for the Tag game
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
    pub fn load() -> Result<Self, io::Error> {
        let example_dir = std::env::current_dir()?.join("examples").join("tag");
        let config_file_path = example_dir.join("config.json");
        let config_file = if let Ok(file) = File::open(&config_file_path) {
            BufReader::new(file)
        } else {
            let template_path = example_dir.join("config.template.json");
            fs::copy(template_path, &config_file_path)?;
            BufReader::new(File::open(config_file_path)?)
        };
        Ok(serde_json::from_reader(config_file)?)
    }

    fn default_num_players() -> u64 {
        Self::default().num_players
    }
}
