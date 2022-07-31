use std::fs;

/// Represents a GameBoy cartridge with game data
pub struct Cartridge {
    name: String,
    data: Box<Vec<u8>>,
}

impl Cartridge {
    /// Return a new cartridge with no data
    pub fn new_empty() -> Self {
        Self {
            name: "Empty".to_owned(),
            data: Box::new(vec![]),
        }
    }

    /// Load a ROM file from disk and store it in a new cartridge structure
    pub fn new_from_file(path: &str) -> Result<Self, String> {
        match fs::read(path) {
            Ok(result) => Ok(Self {
                name: path
                    .split(&['/', '\\'])
                    .last()
                    .unwrap_or("Unknown")
                    .to_owned(),
                data: Box::new(result),
            }),
            Err(error) => Err(format!("Failed to load cartridge: {}", error.to_string())),
        }
    }

    /// Get the name of this cartridge
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get a reference to the cartridge's raw data
    pub fn get_data(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}
