use egui::{Layout, RichText, Ui};
use egui_extras::{Size, TableBuilder};

use crate::emulator::gameboy::GameBoy;

use super::{
    message_queue::{MessageData, MessageQueue},
    DebuggerUiElement, GlobalDebuggerState,
};

const ROW_HEIGHT: f32 = 18.0;
const COLUMN_WIDTH: f32 = 64.0;

/// Contains information about a single breakpoint
pub struct Breakpoint {
    label: String,
    address: u16,
    value: u8,
    invocation_count: usize,
    is_enabled: bool,
}

impl Breakpoint {
    /// Create a new [`Breakpoint`]
    pub fn new(label: &str, address: u16) -> Self {
        Self {
            label: label.to_owned(),
            address,
            value: 0,
            invocation_count: 0,
            is_enabled: true,
        }
    }
}

/// Breakpoint debugger UI
pub struct Breakpoints {
    breakpoint_label_input: String,
    breakpoint_address_input: String,
    breakpoints: Vec<Breakpoint>,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            breakpoints: vec![],
            breakpoint_label_input: "".to_owned(),
            breakpoint_address_input: "".to_owned(),
        }
    }
}

impl Breakpoints {
    /// Get the addresses of all currently active breakpoints
    pub fn get_active_breakpoint_addresses(&self) -> Vec<u16> {
        self.breakpoints
            .iter()
            .filter(|breakpoint| breakpoint.is_enabled)
            .map(|breakpoint| breakpoint.address)
            .collect::<Vec<_>>()
    }

    // Returns true if no breakpoint has been set at the current address
    fn is_new_breakpoint_allowed(&self, address: u16) -> bool {
        !self
            .breakpoints
            .iter()
            .any(|breakpoint| breakpoint.address == address)
    }

    /// Draw the input elements used to add new breakpoints   
    fn draw_breakpoint_create_form(
        &mut self,
        message_queue: &mut MessageQueue,
        ui: &mut Ui,
        gameboy: &GameBoy,
        state: &GlobalDebuggerState,
    ) {
        // Keep all breakpoints up-to-date
        for breakpoint in self.breakpoints.iter_mut() {
            // Memory may have updated - ensure the breakpoint shows the same value
            breakpoint.value = gameboy
                .get_memory_readonly()
                .read_byte_at(breakpoint.address);

            // If the next instruction is a breakpoint, the application needs to be paused until
            // the user has acted
            if gameboy.get_cpu_readonly().program_counter == breakpoint.address
                && state.is_running
                && breakpoint.is_enabled
            {
                breakpoint.invocation_count += 1;
                message_queue.push(MessageData::BreakpointTriggered);
            }
        }

        ui.push_id("add_breakpoint_form", |ui| {
            TableBuilder::new(ui)
                .column(Size::initial(COLUMN_WIDTH))
                .column(Size::remainder())
                .column(Size::initial(25.0))
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label("Label");
                        });

                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.breakpoint_label_input);
                        });
                    });

                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label("Address");
                        });

                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.breakpoint_address_input);
                        });

                        row.col(|ui| {
                            if ui.button("set").clicked() {
                                if let Ok(address_hex) = u16::from_str_radix(
                                    &self
                                        .breakpoint_address_input
                                        .to_lowercase()
                                        .trim_start_matches("0x"),
                                    16,
                                ) {
                                    if self.is_new_breakpoint_allowed(address_hex) {
                                        self.breakpoints.push(Breakpoint::new(
                                            &self.breakpoint_label_input,
                                            address_hex,
                                        ));
                                    }

                                    self.breakpoint_label_input.clear();
                                    self.breakpoint_address_input = "".to_owned();
                                }
                            }
                        });
                    });
                });
        });
    }

    /// Draw all breakpoints
    fn draw_breakpoints(
        &mut self,
        message_queue: &mut MessageQueue,
        ui: &mut Ui,
        gameboy: &GameBoy,
        state: &GlobalDebuggerState,
    ) {
        let mut breakpoint_index_to_remove = None;

        ui.push_id("breakpoints_table", |ui| {
            TableBuilder::new(ui)
                .column(Size::initial(COLUMN_WIDTH))
                .column(Size::initial(COLUMN_WIDTH))
                .column(Size::initial(COLUMN_WIDTH))
                .column(Size::initial(COLUMN_WIDTH * 2.0))
                .column(Size::initial(COLUMN_WIDTH))
                .column(Size::initial(COLUMN_WIDTH))
                .header(ROW_HEIGHT, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Enabled").underline());
                    });

                    header.col(|ui| {
                        ui.label(RichText::new("Address").underline());
                    });

                    header.col(|ui| {
                        ui.label(RichText::new("Value").underline());
                    });

                    header.col(|ui| {
                        ui.label(RichText::new("Label").underline());
                    });

                    header.col(|ui| {
                        ui.label(RichText::new("Invocations").underline());
                    });

                    // Empty column used to position the "delete breakpoint" item
                    header.col(|_| {});
                })
                .body(|body| {
                    body.rows(ROW_HEIGHT, self.breakpoints.len(), |row_index, mut row| {
                        let breakpoint = &mut self.breakpoints[row_index];
                        let address_text = RichText::new(&format!("{:#06X}", breakpoint.address));
                        let value_text = RichText::new(&format!("{:#04X}", breakpoint.value));
                        let label_text = RichText::new(&breakpoint.label);
                        let invocations_text =
                            RichText::new(breakpoint.invocation_count.to_string());

                        row.col(|ui| {
                            ui.checkbox(&mut breakpoint.is_enabled, "");
                        });

                        row.col(|ui| {
                            if breakpoint.is_enabled {
                                ui.label(address_text);
                            } else {
                                ui.label(address_text.weak());
                            }
                        });

                        row.col(|ui| {
                            if breakpoint.is_enabled {
                                ui.label(value_text);
                            } else {
                                ui.label(value_text.weak());
                            }
                        });

                        row.col(|ui| {
                            if breakpoint.is_enabled {
                                ui.label(label_text);
                            } else {
                                ui.label(label_text.weak());
                            }
                        });

                        row.col(|ui| {
                            if breakpoint.is_enabled {
                                ui.label(invocations_text);
                            } else {
                                ui.label(invocations_text.weak());
                            }
                        });

                        row.col(|ui| {
                            if ui.button("delete").clicked() {
                                breakpoint_index_to_remove = Some(row_index);
                            }
                        });
                    });
                });
        });

        // Only modify items after the UI has finished iterating over them
        if let Some(index) = breakpoint_index_to_remove {
            self.breakpoints.remove(index);
        }
    }
}

impl DebuggerUiElement for Breakpoints {
    fn draw(
        &mut self,
        message_queue: &mut MessageQueue,
        ui: &mut Ui,
        gameboy: &GameBoy,
        state: &GlobalDebuggerState,
    ) {
        ui.horizontal(|ui| {
            ui.heading("Breakpoints");

            ui.with_layout(Layout::right_to_left(), |ui| {
                if ui.button("set at PC").clicked() {
                    let address = gameboy.get_cpu_readonly().program_counter;

                    if self.is_new_breakpoint_allowed(address) {
                        self.breakpoints
                            .push(Breakpoint::new(&self.breakpoint_label_input, address));
                    }

                    self.breakpoint_label_input.clear();
                    self.breakpoint_address_input = "".to_owned();
                }
            });
        });

        ui.separator();

        self.draw_breakpoint_create_form(message_queue, ui, gameboy, state);
        ui.separator();

        if self.breakpoints.is_empty() {
            ui.label("No breakpoints have been set");
        } else {
            self.draw_breakpoints(message_queue, ui, gameboy, state);
        }
    }
}
