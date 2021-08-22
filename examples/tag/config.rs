use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use crate::{agent::Properties, Board};

/// Configuration for the Tag game
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub board: Board,
    pub num_players: u64,
    pub step: u32,
    pub player_properties: Properties,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            board: Board::default(),
            num_players: 10,
            step: 1,
            player_properties: Properties::default(),
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
            let writer = BufWriter::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(config_file_path)?,
            );
            let config = Self::default();
            serde_json::to_writer_pretty(writer, &Self::default())?;
            Ok(config)
        }
    }
}
