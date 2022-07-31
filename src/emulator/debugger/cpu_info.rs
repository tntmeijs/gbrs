use egui::{Color32, Frame, RichText, TopBottomPanel, Ui};
use egui_extras::{Size, TableBuilder};

use crate::{
    cpu::cpu::{Cpu, InterruptFlag},
    emulator::gameboy::GameBoy,
    memory::memory::Memory,
};

use super::{message_queue::MessageQueue, DebuggerUiElement, GlobalDebuggerState};

const ROW_HEIGHT: f32 = 18.0;

pub struct CpuInfo {}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {}
    }
}

impl CpuInfo {
    /// Draw a table with all CPU register values
    fn draw_register_info(&self, ui: &mut Ui, cpu: &Cpu) {
        ui.heading("Registers");
        ui.separator();

        ui.push_id("register_info_table", |ui| {
            TableBuilder::new(ui)
                .column(Size::exact(128.0))
                .column(Size::remainder())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label("Program Counter");
                        });

                        row.col(|ui| {
                            ui.label(
                                RichText::new(format!("{:#06X}", cpu.program_counter))
                                    .background_color(Color32::LIGHT_RED),
                            );
                        });
                    });

                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label("Stack Pointer");
                        });

                        row.col(|ui| {
                            ui.label(
                                RichText::new(format!("{:#06X}", cpu.stack_pointer))
                                    .background_color(Color32::GOLD),
                            );
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
        });
    }

    /// Draw a table with all CPU flag values
    fn draw_flag_info(&self, ui: &mut Ui, cpu: &Cpu) {
        ui.heading("CPU Flags");
        ui.separator();

        ui.push_id("cpu_flags_info_table", |ui| {
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
        });
    }

    /// Draw a table with all interrupt flag values
    fn draw_interrupt_flag_info(&self, ui: &mut Ui, cpu: &Cpu, memory: &Memory) {
        ui.heading("Interrupts");
        ui.separator();

        ui.push_id("interrupts_info_table", |ui| {
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
                                cpu.is_request_interrupt_flag_set(
                                    memory,
                                    InterruptFlag::VBlank(None)
                                )
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
                                cpu.is_request_interrupt_flag_set(
                                    memory,
                                    InterruptFlag::LcdStat(None)
                                )
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
                                cpu.is_request_interrupt_flag_set(
                                    memory,
                                    InterruptFlag::Timer(None)
                                )
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
                                cpu.is_request_interrupt_flag_set(
                                    memory,
                                    InterruptFlag::Serial(None)
                                )
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
                                cpu.is_request_interrupt_flag_set(
                                    memory,
                                    InterruptFlag::Joypad(None)
                                )
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
        });
    }
}

impl DebuggerUiElement for CpuInfo {
    fn draw(
        &mut self,
        _message_queue: &mut MessageQueue,
        ui: &mut Ui,
        gameboy: &GameBoy,
        _state: &GlobalDebuggerState,
    ) {
        let height = ui.available_height();

        TopBottomPanel::top("cpu_register_info")
            .frame(Frame::none())
            .default_height(height * 0.5)
            .show_inside(ui, |ui| {
                self.draw_register_info(ui, gameboy.get_cpu_readonly());
            });

        TopBottomPanel::top("cpu_interrupt_flag)info")
            .frame(Frame::none())
            .default_height(height * 0.25)
            .show_inside(ui, |ui| {
                self.draw_interrupt_flag_info(
                    ui,
                    gameboy.get_cpu_readonly(),
                    gameboy.get_memory_readonly(),
                );
            });

        TopBottomPanel::top("cpu_flag_info")
            .frame(Frame::none())
            .default_height(height * 0.25)
            .show_inside(ui, |ui| {
                self.draw_flag_info(ui, gameboy.get_cpu_readonly());
            });
    }
}
