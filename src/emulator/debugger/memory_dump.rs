use egui::{Color32, RichText, Ui};
use egui_extras::{Size, TableBuilder};

use crate::emulator::gameboy::GameBoy;

use super::{message_queue::MessageQueue, DebuggerUiElement, GlobalDebuggerState};

const ROW_HEIGHT: f32 = 18.0;
const VALUES_PER_ROW: usize = 16;
const TABLE_HEIGHT: usize = 0x10_000 / VALUES_PER_ROW;
const FIRST_COLUMN_WIDTH: f32 = 55.0;

pub struct MemoryDump {}

impl Default for MemoryDump {
    fn default() -> Self {
        Self {}
    }
}

impl DebuggerUiElement for MemoryDump {
    fn draw(
        &mut self,
        _message_queue: &mut MessageQueue,
        ui: &mut Ui,
        gameboy: &GameBoy,
        _state: &GlobalDebuggerState,
    ) {
        let cpu = gameboy.get_cpu_readonly();
        let memory = gameboy.get_memory_readonly();

        ui.heading("Memory Inspector");
        ui.separator();

        ui.push_id("memory_dump_table", |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::exact(FIRST_COLUMN_WIDTH))
                .columns(Size::remainder(), VALUES_PER_ROW)
                .header(ROW_HEIGHT, |mut header| {
                    // Empty column, this is the column that will contain the vertical axis' addresses
                    header.col(|_| {});

                    // These columns will contain the horizontal axis' addresses
                    for i in 0..VALUES_PER_ROW {
                        header.col(|ui| {
                            ui.label(&format!("{:#04X}", i)[2..]);
                        });
                    }
                })
                .body(|body| {
                    body.rows(ROW_HEIGHT, TABLE_HEIGHT, |row_index, mut row| {
                        // First column is for display purposes only, it simply displays the addresses in
                        // steps of 16 (0xF)
                        row.col(|ui| {
                            ui.label(format!("{:#06X}", row_index * VALUES_PER_ROW));
                        });

                        // Display the raw memory dump by mapping each table (2D array) to an address
                        for i in 0..VALUES_PER_ROW {
                            let address = (row_index * VALUES_PER_ROW) + i;
                            assert!(address <= 0xFFFF, "Address cannot exceed 0xFFFF");

                            row.col(|ui| {
                                let address = address as u16;
                                let address_text = RichText::new(
                                    &format!("{:#04X}", memory.read_byte_at(address))[2..],
                                );

                                // Highlight the data pointed to be the program counter and stack
                                // pointer
                                if address == cpu.program_counter {
                                    ui.label(
                                        address_text.background_color(Color32::LIGHT_RED).strong(),
                                    );
                                } else if address == cpu.stack_pointer {
                                    ui.label(address_text.background_color(Color32::GOLD).strong());
                                } else {
                                    ui.label(address_text);
                                }
                            });
                        }
                    });
                });
        });
    }
}
