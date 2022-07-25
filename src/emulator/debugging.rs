/// Structure that contains all debug data for the emulator
pub struct DebugData {
    /// Framerate expressed in number of frames per second of elapsed time
    pub framerate: u16,
}

impl DebugData {
    /// Create a new DebugData instance
    pub fn new() -> Self {
        Self { framerate: 0 }
    }
}

impl Default for DebugData {
    /// Create a new DebugData instance with default values
    fn default() -> Self {
        Self::new()
    }
}
