use egui::Ui;

use crate::emulator::gameboy::GameBoy;

use super::{
    message_queue::{MessageData, MessageQueue},
    DebuggerUiElement, GlobalDebuggerState,
};

pub struct Toolbar {
    clear_memory_on_cpu_reset: bool,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            clear_memory_on_cpu_reset: false,
        }
    }
}

impl DebuggerUiElement for Toolbar {
    fn draw(
        &mut self,
        message_queue: &mut MessageQueue,
        ui: &mut Ui,
        _gameboy: &GameBoy,
        state: &GlobalDebuggerState,
    ) {
        ui.horizontal(|ui| {
            if state.is_game_loaded {
                if state.is_running {
                    if ui.button("pause").clicked() {
                        message_queue.push(MessageData::UpdateRunningState(false));
                    }
                } else {
                    if ui.button("resume").clicked() {
                        message_queue.push(MessageData::UpdateRunningState(true));
                    }

                    if ui.button("tick").clicked() {
                        message_queue.push(MessageData::TickOnce);
                    }

                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.checkbox(&mut self.clear_memory_on_cpu_reset, "also clear memory");

                        if ui.button("reset").clicked() {
                            message_queue
                                .push(MessageData::ResetGameBoy(self.clear_memory_on_cpu_reset));
                        }
                    });
                }
            } else {
                ui.label("Load a game to start playing / debugging!");
            }
        });
    }
}
