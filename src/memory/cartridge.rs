use std::fs;

use log::{error, info};

/// Represents a GameBoy cartridge with game data
pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    /// Return a new cartridge with no data
    pub fn new_empty() -> Self {
        Self { data: vec![] }
    }

    /// Load a ROM file from disk and store it in a new cartridge structure
    pub fn new_from_file(path: &str) -> Self {
        match fs::read(path) {
            Ok(result) => {
                info!("Successfully loaded {}", path);
                Self { data: result }
            }
            Err(error) => {
                error!("Failed to load cartridge: {}", error.to_string());
                Self::new_empty()
            }
        }
    }
}
