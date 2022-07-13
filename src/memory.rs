use log::error;

/// Represents a GameBoy's addressable memory - this includes ROM, RAM, VRAM, etc.
pub struct Memory {
    pub bytes: Vec<u8>,
}

impl Memory {
    /// Create a new memory object, allocate the maximum size, and set all values to zero
    pub fn new() -> Self {
        Self {
            bytes: vec![0u8; 0x10_000],
        }
    }

    /// Copy a block of bytes into memory starting at the specified address without allocating a
    /// a new vector (this method iterates over all bytes and copies each value)
    pub fn copy_into_memory_at_address(&mut self, address: u16, data: &Vec<u8>) {
        let last_byte_index = usize::from(address) + data.len();

        if last_byte_index > self.bytes.len() {
            error!(
                "Unable to copy data into memory because it would exceed memory bounds: ({}/{})",
                last_byte_index,
                self.bytes.len()
            );
        } else {
            for (index, byte) in data.iter().enumerate() {
                let write_address = index + usize::from(address);

                // Write address will always fit in a u16 as its bounds were checked already
                self.write_byte_at(write_address as u16, byte);
            }
        }
    }

    /// Read a single byte at the specified address
    pub fn read_byte_at(&self, address: u16) -> u8 {
        self.bytes[usize::from(address)]
    }

    pub fn write_byte_at(&mut self, address: u16, value: &u8) {
        self.bytes[usize::from(address)] = *value;
    }
}
