// Hide console window on Windows when running in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

use log::error;

use crate::{
    cpu::cpu::Cpu,
    memory::{cartridge::Cartridge, memory::Memory},
};

use super::{
    debugging::{
        draw_cpu_flag_info, draw_interrupt_flag_info, draw_memory_dump, draw_register_info,
    },
    menu::draw_menu_bar,
};

/// Application class
pub struct Emulator {
    /// Emulated CPU
    pub cpu: Cpu,

    /// Emulated memory (lives on the heap because it is a rather large object)
    memory: Box<Memory>,

    /// Previous update instance - used to calculate the elapsed time in between updates
    previous_update: Instant,

    /// Flag used to indicate that a ROM has been loaded into memory
    is_rom_loaded: bool,
}

impl Emulator {
    /// Create a new emulator
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Box::new(Memory::new()),
            previous_update: Instant::now(),
            is_rom_loaded: false,
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

        if self.is_rom_loaded {
            self.cpu.tick(self.memory.as_mut());
        }
    }

    /// Display the current state of the emulator
    fn render(&mut self, context: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(context, |ui| {
            draw_menu_bar(self, ui);
        });

        egui::SidePanel::right("memory_dump")
            .resizable(false)
            .show(context, |ui| {
                draw_memory_dump(&self.memory, ui);
            });

        egui::TopBottomPanel::bottom("debugger")
            .min_height(225.0)
            .show(context, |ui| {
                egui::SidePanel::left("registers")
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        draw_register_info(&self.cpu, ui);
                    });

                egui::SidePanel::right("cpu_flags")
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        draw_cpu_flag_info(&self.cpu, ui);
                    });

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    draw_interrupt_flag_info(&self.cpu, &self.memory, ui);
                });
            });
    }

    /// Load a cartridge ROM into memory
    pub fn load_rom(&mut self, path: &str) {
        // Try to load data from disk
        match Cartridge::new_from_file(path) {
            Ok(cartridge) => {
                // Load ROM data into memory
                if self
                    .memory
                    .as_mut()
                    .copy_into_memory_at_address(0x0000, &cartridge.data)
                {
                    // Successfully loaded ROM data into memory
                    self.is_rom_loaded = true;
                    self.cpu.reset_program_counter();
                }
            }
            Err(error) => error!("Unable to load cartridge: {}", error),
        };
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
