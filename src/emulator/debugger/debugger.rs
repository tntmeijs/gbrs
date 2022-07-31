use egui::Context;
use log::error;

use crate::{emulator::gameboy::GameBoy, memory::cartridge::Cartridge};

use super::{
    breakpoints::Breakpoints,
    cpu_info::CpuInfo,
    memory_dump::MemoryDump,
    menu_bar::MenuBar,
    message_queue::{MessageData, MessageQueue},
    toolbar::Toolbar,
    DebuggerUiElement, GlobalDebuggerState,
};

/// GameBoy debugger
pub struct Debugger {
    message_queue: MessageQueue,
    menu: MenuBar,
    toolbar: Toolbar,
    breakpoints: Breakpoints,
    cpu_info: CpuInfo,
    memory_dump: MemoryDump,
    debugger_state: GlobalDebuggerState,
}

impl Debugger {
    /// Create a new debugger instance
    pub fn new() -> Self {
        Self {
            message_queue: MessageQueue::default(),
            menu: MenuBar::default(),
            toolbar: Toolbar::default(),
            breakpoints: Breakpoints::default(),
            cpu_info: CpuInfo::default(),
            memory_dump: MemoryDump::default(),
            debugger_state: GlobalDebuggerState::default(),
        }
    }

    /// Check if the emulator is allowed to execute the next instruction
    pub fn gameboy_should_tick(&mut self) -> bool {
        // Cannot execute anything if no game has been loaded
        if !self.debugger_state.is_game_loaded {
            return false;
        }

        // Only execute the next instruction once and enter the pause state on the next update
        if self.debugger_state.should_tick_once {
            self.debugger_state.is_running = false;
            self.debugger_state.should_tick_once = false;
            return true;
        }

        // No special condition left to check - simply allow the emulator to run if not paused
        self.debugger_state.is_running
    }

    /// Update the debugger's internal state. This should happen before the emulator ticks and
    /// before the debugger's UI is drawn.
    pub fn update(&mut self, gameboy: &mut GameBoy) {
        while let Some(message) = self.message_queue.poll() {
            match message {
                MessageData::LoadRomFromDisk(path) => {
                    self.load_cartridge_from_disk(&path, gameboy);
                }
                MessageData::UpdateRunningState(is_running) => {
                    self.debugger_state.is_running = is_running
                }
                MessageData::TickOnce => self.debugger_state.should_tick_once = true,
                MessageData::ResetGameBoy(clear_memory) => {
                    gameboy.reset(clear_memory);

                    if clear_memory {
                        self.debugger_state.is_game_loaded = false;
                    }
                }
                MessageData::BreakpointTriggered => self.debugger_state.is_running = false,
            };
        }

        // Ensure the memory debugger is able to highlight all known breakpoints
        self.memory_dump
            .set_breakpoint_addresses(&self.breakpoints.get_active_breakpoint_addresses());
    }

    /// Draw the entire debugger
    pub fn draw(&mut self, context: &Context, gameboy: &GameBoy) {
        egui::TopBottomPanel::top("menu").show(context, |ui| {
            self.menu
                .draw(&mut self.message_queue, ui, gameboy, &self.debugger_state)
        });

        egui::TopBottomPanel::top("toolbar").show(context, |ui| {
            self.toolbar
                .draw(&mut self.message_queue, ui, gameboy, &self.debugger_state);
        });

        egui::CentralPanel::default().show(context, |ui| {
            // Breakpoints
            egui::SidePanel::left("breakpoints")
                .resizable(false)
                .show_inside(ui, |ui| {
                    self.breakpoints.draw(
                        &mut self.message_queue,
                        ui,
                        gameboy,
                        &self.debugger_state,
                    );
                });

            // CPU information
            egui::SidePanel::right("registers")
                .resizable(false)
                .show_inside(ui, |ui| {
                    self.cpu_info
                        .draw(&mut self.message_queue, ui, gameboy, &self.debugger_state);
                });

            // Memory dump
            egui::CentralPanel::default().show_inside(ui, |ui| {
                self.memory_dump
                    .draw(&mut self.message_queue, ui, gameboy, &self.debugger_state);
            });
        });
    }

    /// Load a cartridge from disk and store it in the debugger instance as the active cartridge
    pub fn load_cartridge_from_disk(&mut self, path: &str, gameboy: &mut GameBoy) {
        match Cartridge::new_from_file(path) {
            Ok(cartridge) => {
                self.debugger_state.is_game_loaded = gameboy.load_cartridge(&cartridge);

                if !self.debugger_state.is_game_loaded {
                    error!("Unable to load ROM into GameBoy memory");
                }
            }
            Err(error) => error!("Unable to load cartridge from file: {}", error),
        };
    }
}
