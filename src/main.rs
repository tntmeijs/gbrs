mod cpu;
mod emulator;
mod memory;
mod utility;

use egui::Vec2;
use emulator::emulator::Emulator;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .init()
        .expect("Unable to initialize logger");

    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(1280.0, 720.0)),
        ..Default::default()
    };

    let gui = Box::new(Emulator::new());
    eframe::run_native("GBRS - Tahar Meijs", options, Box::new(|_context| gui));
}
