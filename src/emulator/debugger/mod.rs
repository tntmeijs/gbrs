use egui::Ui;

use self::message_queue::MessageQueue;

use super::gameboy::GameBoy;

mod breakpoints;
mod cpu_info;
pub mod debugger;
mod memory_dump;
mod menu_bar;
mod message_queue;
mod toolbar;

struct GlobalDebuggerState {
    pub is_running: bool,
    pub should_tick_once: bool,
    pub is_game_loaded: bool,
}

impl Default for GlobalDebuggerState {
    fn default() -> Self {
        Self {
            is_running: false,
            should_tick_once: false,
            is_game_loaded: false,
        }
    }
}

trait DebuggerUiElement {
    /// Draw the debugger's UI element
    fn draw(
        &mut self,
        message_queue: &mut MessageQueue,
        ui: &mut Ui,
        gameboy: &GameBoy,
        state: &GlobalDebuggerState,
    );
}
