use log::{error, info};

use crate::{
    memory::Memory,
    opcodes::{jp_a16, nop},
};

/// Represents the GameBoy's CPU (a Sharp CPU based on Intel's 8080 CPU)
pub struct Cpu {
    /// Accumulator and flags register
    af: u16,

    /// BC register
    bc: u16,

    /// DE register
    de: u16,

    /// HL register
    hl: u16,

    /// Stack pointer
    stack_pointer: u16,

    /// Program counter
    program_counter: u16,

    /// Current cycle
    current_cycle: u64,
}

impl Cpu {
    /// Create a new Cpu instance, reset all registers, and set the program counter to the starting
    /// location (0x0100). This emulator will skip the built-in ROM and immediately jumps to the
    /// memory at which the game's logic is located.
    pub fn new() -> Self {
        Self {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            stack_pointer: 0,
            program_counter: 0x0100,
            current_cycle: 0,
        }
    }

    /// Execute the next instruction in memory
    pub fn tick(&mut self, memory: &mut Memory) {
        self.process_opcode(memory);
    }

    /// Move the program counter by adding an offset to the current position
    pub fn move_program_counter_by(&mut self, delta: u16) {
        self.program_counter += delta;
    }

    /// Set the program counter to the specified address
    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    /// Get the current value of the program counter
    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Update the internal cycle tracker of the CPU
    pub fn add_cycles(&mut self, cycles: u64) {
        self.current_cycle += cycles;
    }

    /// Process the current opcode
    fn process_opcode(&mut self, memory: &mut Memory) {
        let opcode = self.read_opcode(memory);

        match opcode {
            0x00 => nop(self),
            0xC3 => jp_a16(self, memory),
            _ => error!(
                "Unknown opcode \"{:#03x}\" at address \"{:#06x}\"",
                opcode, self.program_counter
            ),
        };
    }

    /// Read an opcode from the location in memory to which the program counter points
    fn read_opcode(&self, memory: &mut Memory) -> u8 {
        memory.read_byte_at(self.program_counter)
    }
}
