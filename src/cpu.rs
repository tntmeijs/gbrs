use log::{error, info};

use crate::memory::Memory;

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
    cycle: u64,
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
            cycle: 0,
        }
    }

    /// Execute the next instruction in memory
    pub fn tick(&mut self, memory: &mut Memory) {
        self.process_opcode(memory);
    }

    fn process_opcode(&mut self, memory: &mut Memory) {
        let opcode = self.read_opcode(memory);

        match opcode {
            _ => error!("Unknown opcode: {:#06x}", opcode),
        };
    }

    fn read_opcode(&mut self, memory: &mut Memory) -> u8 {
        let opcode = memory.read_byte_at(self.program_counter);
        info!(
            "PC: {:#06x} --> opcode {:#06x}]",
            self.program_counter, opcode
        );

        self.program_counter += 1;
        opcode
    }
}
