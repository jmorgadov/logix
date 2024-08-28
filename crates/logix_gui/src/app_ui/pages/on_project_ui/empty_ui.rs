use egui::Context;

use crate::app_ui::{
    logix_app::LogixApp,
    shortcuts::{shortcut_string, NEW_BOARD},
};

impl LogixApp {
    pub fn empty_ui(ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 4.0);
                ui.label(format!(
                    "Creating a new board ({}) or open an existing one.",
                    shortcut_string(NEW_BOARD)
                ));
            });
        });
    }
}
