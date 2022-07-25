// Hide console window on Windows when running in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

use crate::{
    cpu::cpu::Cpu,
    memory::{cartridge::Cartridge, memory::Memory},
};

use super::debugging::DebugData;

/// Application class
pub struct Emulator {
    /// Loaded cartridge (lives on the heap because it is a rather large object)
    cartridge: Option<Box<Cartridge>>,

    /// Emulated CPU
    cpu: Cpu,

    /// Emulated memory (lives on the heap because it is a rather large object)
    memory: Box<Memory>,

    /// Previous update instance - used to calculate the elapsed time in between updates
    previous_update: Instant,

    /// Debug information
    debug_data: DebugData,
}

impl Emulator {
    /// Create a new emulator
    pub fn new() -> Self {
        Self {
            cartridge: None,
            cpu: Cpu::new(),
            memory: Box::new(Memory::new()),
            previous_update: Instant::now(),
            debug_data: DebugData::new(),
        }
    }
}

impl Default for Emulator {
    /// Create a new emulator instance with default values
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    /// Update the emulator's internal state
    fn update(&mut self) {
        let now = Instant::now();
        let delta = now - self.previous_update;
        self.previous_update = now;

        // Only emulate if a cartridge is available
        if self.cartridge.is_some() {
            self.cpu.tick(self.memory.as_mut());
        }

        self.debug_data.framerate = f64::round(1.0 / delta.as_secs_f64()) as u16;
    }

    /// Display the current state of the emulator
    fn render(&self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            if self.cartridge.is_some() {
                ui.label("Game has been loaded");
            } else {
                ui.label("Please select a game");
            }

            ui.label(format!("{} fps", self.debug_data.framerate));
        });
    }
}

impl eframe::App for Emulator {
    /// Called whenever the UI should be repainted
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update();
        self.render(ctx);

        // Keep rendering continuously
        ctx.request_repaint();
    }
}
