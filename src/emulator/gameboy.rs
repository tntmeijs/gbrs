use std::time::Instant;

use crate::{
    cpu::cpu::Cpu,
    memory::{cartridge::Cartridge, memory::Memory},
};

/// Represents a GameBoy device
pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,
    previous_tick: Instant,
}

impl GameBoy {
    /// Create a new GameBoy instance
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            previous_tick: Instant::now(),
        }
    }

    /// Execute the next instruction
    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now - self.previous_tick;
        self.previous_tick = now;

        self.cpu.tick(&mut self.memory);
    }

    /// Reset the GameBoy to its default state
    pub fn reset(&mut self, clear_memory: bool) {
        self.cpu.reset();

        if clear_memory {
            self.memory.clear();
        }
    }

    /// Use this to read the state of the GameBoy's CPU
    pub fn get_cpu_readonly(&self) -> &Cpu {
        &self.cpu
    }

    /// Use this to read the state of the GameBoy's memory
    pub fn get_memory_readonly(&self) -> &Memory {
        &self.memory
    }

    /// Load a cartiridge into memory and reset the GameBoy to its initial state.
    /// Returns true on success and false on failure.
    pub fn load_cartridge(&mut self, cartridge: &Cartridge) -> bool {
        let success = self
            .memory
            .copy_into_memory_at_address(0x0000, cartridge.get_data());

        if success {
            self.cpu.reset();
        }

        success
    }
}
