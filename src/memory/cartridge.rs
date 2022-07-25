use std::fs;

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
    pub fn new_from_file(path: &str) -> Result<Self, String> {
        match fs::read(path) {
            Ok(result) => Ok(Self { data: result }),
            Err(error) => Err(format!("Failed to load cartridge: {}", error.to_string())),
        }
    }
}

impl Default for Cartridge {
    /// Create a new Cartridge instance with default values
    fn default() -> Self {
        Self::new_empty()
    }
}
