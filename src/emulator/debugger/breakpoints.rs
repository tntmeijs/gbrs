use egui::Ui;

use crate::emulator::gameboy::GameBoy;

use super::{message_queue::MessageQueue, DebuggerUiElement, GlobalDebuggerState};

/// Contains information about a single breakpoint
pub struct Breakpoint {
    label: String,
    address: u16,
    invocation_count: usize,
}

/// Breakpoint debugger UI
pub struct Breakpoints {}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {}
    }
}

impl DebuggerUiElement for Breakpoints {
    fn draw(
        &mut self,
        _message_queue: &mut MessageQueue,
        ui: &mut Ui,
        _gameboy: &GameBoy,
        _state: &GlobalDebuggerState,
    ) {
        ui.heading("Breakpoints");
        ui.separator();
        ui.label("No breakpoints have been set");
    }
}
