use egui::{menu, Ui};
use log::error;

use super::emulator::Emulator;

/// Draw the emulator's menu bar
pub fn draw_menu_bar(emulator: &mut Emulator, ui: &mut Ui) {
    menu::bar(ui, |ui| {
        menu::menu_button(ui, "File", |ui| {
            file_menu(emulator, ui);
        });

        menu::menu_button(ui, "Help", |ui| {
            about_menu(ui);
        });
    });
}

/// Draw the "file" menu
fn file_menu(emulator: &mut Emulator, ui: &mut Ui) {
    if ui.button("Load ROM").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("GameBoy ROM", &["gb"])
            .add_filter("All Files", &["*"])
            .set_directory("/")
            .pick_file()
        {
            if let Some(path) = path.to_str() {
                emulator.load_rom(path);
            } else {
                error!("Unable to convert path into a string - this should never happen");
            }
        }
    }
}

/// Draw the "about" menu
fn about_menu(ui: &mut Ui) {
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
}
