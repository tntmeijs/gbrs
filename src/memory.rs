use log::error;

use crate::{
    bits::{lsb_msb_to_u16, u16_to_lsb_msb},
    cpu::Cpu,
};

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
                self.write_byte_at(write_address as u16, *byte);
            }
        }
    }

    /// Read a single byte from the specified address
    pub fn read_byte_at(&self, address: u16) -> u8 {
        self.bytes[usize::from(address)]
    }

    /// Read a 16-bit value from the specified address and address + 1.
    /// The GameBoy is little-endian, which means the least significant byte comes first!
    pub fn read_16_bit_value_at(&self, address: u16) -> u16 {
        let lsb = self.read_byte_at(address);
        let msb = self.read_byte_at(address + 1);

        lsb_msb_to_u16(lsb, msb)
    }

    /// Write a single byte to the specified address
    pub fn write_byte_at(&mut self, address: u16, value: u8) {
        self.bytes[usize::from(address)] = value;
    }

    /// Write a 16-bit value to the specified address and address + 1.
    /// The GameBoy is little-endian, which means the least significant byte comes first!
    pub fn write_16_bit_value_at(&mut self, address: u16, value: u16) {
        let (lsb, msb) = u16_to_lsb_msb(value);

        self.bytes[usize::from(address)] = lsb;
        self.bytes[usize::from(address + 1)] = msb;
    }

    /// Push a new 8-bit value onto the stack
    pub fn push_stack_u8(&mut self, value: u8, cpu: &mut Cpu) {
        cpu.stack_pointer -= 1;
        self.bytes[usize::from(cpu.stack_pointer)] = value;
    }

    /// Pop an existing 8-bit value from the stack
    pub fn pop_stack_u8(&mut self, cpu: &mut Cpu) -> u8 {
        let value = self.bytes[usize::from(cpu.stack_pointer)];
        cpu.stack_pointer += 1;
        value
    }

    /// Push a new 16-bit value onto the stack
    pub fn push_stack_u16(&mut self, value: u16, cpu: &mut Cpu) {
        let (lsb, msb) = u16_to_lsb_msb(value);

        // MSB has to be stored first because the stack grows "downwards" / "backwards"
        self.push_stack_u8(msb, cpu);

        // LSB has to be stored last because the stack grows "downwards" / "backwards"
        self.push_stack_u8(lsb, cpu);
    }

    /// Pop an existing u16 value from the stack
    pub fn pop_stack_u16(&mut self, cpu: &mut Cpu) -> u16 {
        // Pop in reverse order because the stack grows "downwards" (see push_stack_u16)
        let lsb = self.pop_stack_u8(cpu);
        let msb = self.pop_stack_u8(cpu);

        lsb_msb_to_u16(lsb, msb)
    }
}
