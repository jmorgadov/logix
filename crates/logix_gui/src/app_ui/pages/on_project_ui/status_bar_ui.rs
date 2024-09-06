use egui::Color32;

use crate::app_ui::logix_app::LogixApp;

impl LogixApp {
    pub fn is_sim_running(&self) -> bool {
        self.exist_active_board() && self.board_tabs[self.current_tab].sim.is_some()
    }

    pub fn status_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(20.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if self.is_sim_running() {
                        ui.label(egui::RichText::new("Running").color(Color32::LIGHT_GREEN));
                    }
                });
            });
    }
}
