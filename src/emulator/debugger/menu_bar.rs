use egui::{menu, Ui};
use log::error;

use crate::emulator::gameboy::GameBoy;

use super::{
    message_queue::{MessageData, MessageQueue},
    DebuggerUiElement, GlobalDebuggerState,
};

/// Debugger window menu bar
pub struct MenuBar {}

impl Default for MenuBar {
    fn default() -> Self {
        Self {}
    }
}

impl DebuggerUiElement for MenuBar {
    fn draw(
        &mut self,
        message_queue: &mut MessageQueue,
        ui: &mut Ui,
        _gameboy: &GameBoy,
        _state: &GlobalDebuggerState,
    ) {
        menu::bar(ui, |ui| {
            menu::menu_button(ui, "File", |ui| {
                if ui.button("Load ROM").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("GameBoy ROM", &["gb"])
                        .add_filter("All Files", &["*"])
                        .set_directory("/")
                        .pick_file()
                    {
                        if let Some(path) = path.to_str() {
                            message_queue.push(MessageData::LoadRomFromDisk(path.to_owned()))
                        } else {
                            error!(
                                "Unable to convert path into a string - this should never happen"
                            );
                        }
                    }
                }
            });

            menu::menu_button(ui, "Help", |ui| {
                if ui.button("Open Documentation").clicked() {
                    open::that(format!("{}/wiki", env!("CARGO_PKG_HOMEPAGE"))).ok();
                }

                if ui.button("View License (MIT)").clicked() {
                    open::that(format!(
                        "{}/blob/master/LICENSE",
                        env!("CARGO_PKG_HOMEPAGE")
                    ))
                    .ok();
                }

                if ui.button("Report A Bug").clicked() {
                    open::that(format!("{}/issues", env!("CARGO_PKG_HOMEPAGE"))).ok();
                }

                ui.separator();

                if ui.button("View Source Code").clicked() {
                    open::that(env!("CARGO_PKG_HOMEPAGE")).ok();
                }
            });
        });
    }
}
