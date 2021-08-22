use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use crate::Board;

/// Configuration for the Tag game
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub board: Board,
    #[serde(default = "Config::default_num_players")]
    pub num_players: u64,
    #[serde(default = "Config::default_step")]
    pub step: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            board: Board::default(),
            num_players: 10,
            step: 1,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        let config_file_path = std::env::current_dir()?
            .join("examples")
            .join("tag")
            .join("config.json");
        if let Ok(file) = File::open(&config_file_path) {
            Ok(serde_json::from_reader(BufReader::new(file))?)
        } else {
            let writer = BufWriter::new(OpenOptions::new().write(true).open(config_file_path)?);
            let config = Self::default();
            serde_json::to_writer_pretty(writer, &Self::default())?;
            Ok(config)
        }
    }

    fn default_num_players() -> u64 {
        Self::default().num_players
    }

    fn default_step() -> u32 {
        Self::default().step
    }
}
