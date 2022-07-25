use egui::Ui;
use egui_extras::{Size, TableBuilder};

use crate::{
    cpu::cpu::{Cpu, InterruptFlag},
    memory::memory::Memory,
};

/// Height of a single row in a table
const ROW_HEIGHT: f32 = 18.0;

/// Draw the CPU's register information
pub fn draw_register_info(cpu: &Cpu, ui: &mut Ui) {
    ui.heading("Registers");
    ui.separator();

    TableBuilder::new(ui)
        .column(Size::remainder())
        .column(Size::remainder())
        .body(|mut body| {
            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("PC");
                });

                row.col(|ui| {
                    ui.label(format!("{:#06X}", cpu.program_counter));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("SP");
                });

                row.col(|ui| {
                    ui.label(format!("{:#06X}", cpu.stack_pointer));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("A");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.a));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("B");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.b));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("C");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.c));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("D");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.d));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("E");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.e));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("H");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.h));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("L");
                });

                row.col(|ui| {
                    ui.label(format!("{:#04X}", cpu.l));
                });
            });
        });
}

/// Draw the CPU flag states
pub fn draw_cpu_flag_info(cpu: &Cpu, ui: &mut Ui) {
    ui.heading("CPU Flags");
    ui.separator();

    TableBuilder::new(ui)
        .columns(Size::remainder(), 2)
        .body(|mut body| {
            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("IME");
                });

                row.col(|ui| {
                    ui.label(format!("{}", cpu.ime));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Zero");
                });

                row.col(|ui| {
                    ui.label(format!("{}", cpu.zero));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Negative");
                });

                row.col(|ui| {
                    ui.label(format!("{}", cpu.negative));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Half Carry");
                });

                row.col(|ui| {
                    ui.label(format!("{}", cpu.half_carry));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Carry");
                });

                row.col(|ui| {
                    ui.label(format!("{}", cpu.carry));
                });
            });
        });
}

/// Draw the interrupt flag states
pub fn draw_interrupt_flag_info(cpu: &Cpu, memory: &Memory, ui: &mut Ui) {
    ui.heading("Interrupts");
    ui.separator();

    TableBuilder::new(ui)
        .column(Size::remainder().at_least(75.0))
        .column(Size::remainder().at_least(75.0))
        .column(Size::remainder().at_least(50.0))
        .header(ROW_HEIGHT, |mut header| {
            header.col(|_| {});

            header.col(|ui| {
                ui.label("Requested?");
            });

            header.col(|ui| {
                ui.label("Active?");
            });
        })
        .body(|mut body| {
            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("V-Blank");
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_request_interrupt_flag_set(memory, InterruptFlag::VBlank(None))
                    ));
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_interrupt_flag_set(memory, InterruptFlag::VBlank(None))
                    ));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("LCD STAT");
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_request_interrupt_flag_set(memory, InterruptFlag::LcdStat(None))
                    ));
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_interrupt_flag_set(memory, InterruptFlag::LcdStat(None))
                    ));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Timer");
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_request_interrupt_flag_set(memory, InterruptFlag::Timer(None))
                    ));
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_interrupt_flag_set(memory, InterruptFlag::Timer(None))
                    ));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Serial");
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_request_interrupt_flag_set(memory, InterruptFlag::Serial(None))
                    ));
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_interrupt_flag_set(memory, InterruptFlag::Serial(None))
                    ));
                });
            });

            body.row(ROW_HEIGHT, |mut row| {
                row.col(|ui| {
                    ui.label("Joypad");
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_request_interrupt_flag_set(memory, InterruptFlag::Joypad(None))
                    ));
                });

                row.col(|ui| {
                    ui.label(format!(
                        "{}",
                        cpu.is_interrupt_flag_set(memory, InterruptFlag::Joypad(None))
                    ));
                });
            });
        });
}

/// Draw a raw dump of the emulator's memory
pub fn draw_memory_dump(memory: &Memory, ui: &mut Ui) {
    let values_per_row = 16;
    let table_height = 0x10_000 / values_per_row;

    ui.heading("Memory Inspector");
    ui.separator();

    TableBuilder::new(ui)
        .striped(true)
        .column(Size::exact(55.0))
        .columns(Size::exact(25.0), values_per_row)
        .header(ROW_HEIGHT, |mut header| {
            // Empty column, this is the column that will contain the vertical axis' addresses
            header.col(|_| {});

            // These columns will contain the horizontal axis' addresses
            for i in 0..values_per_row {
                header.col(|ui| {
                    ui.label(&format!("{:#04X}", i)[2..]);
                });
            }
        })
        .body(|body| {
            body.rows(ROW_HEIGHT, table_height, |row_index, mut row| {
                // First column is for display purposes only, it simply displays the addresses in
                // steps of 16 (0xF)
                row.col(|ui| {
                    ui.label(format!("{:#06X}", row_index * values_per_row));
                });

                // Display the raw memory dump by mapping each table (2D array) to an address
                for i in 0..values_per_row {
                    let address = (row_index * values_per_row) + i;
                    assert!(address <= 0xFFFF, "Address cannot exceed 0xFFFF");

                    row.col(|ui| {
                        ui.label(&format!("{:#04X}", memory.read_byte_at(address as u16))[2..]);
                    });
                }
            });
        });
}
