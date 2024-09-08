use egui::Ui;

use crate::app_ui::{library::Library, logix_app::LogixApp};

use super::canvas_payload::CanvasPayload;

impl LogixApp {
    pub fn show_library(&self, ui: &mut Ui) {
        self.library.show(ui);
    }
}

impl Library {
    pub fn show_components(&self, ui: &mut Ui) {
        for (n, comp) in &self.components {
            ui.add(
                egui::Label::new(n)
                    .selectable(false)
                    .sense(egui::Sense::click_and_drag()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .dnd_set_drag_payload(CanvasPayload::Component(comp.clone()));
        }
    }

    pub fn show_sub_libs(&self, ui: &mut Ui, name: &str) {
        for lib in &self.sub_libs {
            ui.collapsing(name, |ui| {
                lib.1.show_components(ui);
                lib.1.show_sub_libs(ui, lib.0);
            });
        }
    }

    pub fn show(&self, ui: &mut Ui) {
        self.show_components(ui);
        self.show_sub_libs(ui, "");
    }
}
