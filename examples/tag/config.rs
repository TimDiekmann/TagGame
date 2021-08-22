use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
    ops::Range,
};

use serde::{Deserialize, Serialize};

use crate::Board;

/// Configuration for player properties and behaviors
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentConfig {
    pub untagged_deciding: Range<f64>,
    pub tagged_deciding: Range<f64>,
    pub untagged_speed_multiplied: Range<f32>,
    pub tagged_speed_multiplied: Range<f32>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            untagged_deciding: 0.5..0.8,
            tagged_deciding: 0.7..0.9,
            untagged_speed_multiplied: 0.8..1.0,
            tagged_speed_multiplied: 0.9..1.1,
        }
    }
}

/// Configuration for the Tag game
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub board: Board,
    pub num_players: usize,
    pub step: u32,
    pub agents: AgentConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            board: Board::default(),
            num_players: 10,
            step: 1,
            agents: AgentConfig::default(),
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
