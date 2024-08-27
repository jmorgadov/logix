use std::{path::PathBuf, str::FromStr};

use rfd::FileDialog;

use crate::app_ui::{
    app_state::{AppState, LeftPannelState},
    logix_app::LogixApp,
};

impl LogixApp {
    fn get_recent_projects(&self, max: usize) -> Vec<(String, u64)> {
        let projects = self.data.projects_opened.clone();
        let mut sorted = projects.into_iter().collect::<Vec<(String, u64)>>();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(max);
        sorted
    }

    pub fn draw_welcome_page(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 4.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Welcome to Logix!").size(40.0),
                ));
                ui.label("Create and simulate digital circuits with ease.");

                ui.add_space(20.0);
                if ui.button("New project").clicked() {
                    self.state = AppState::CreatingNewProject {
                        folder: String::default(),
                        name: String::default(),
                    };
                }
                if ui.button("Open a project").clicked() {
                    let new_folder = FileDialog::new().pick_folder();
                    let path = new_folder.unwrap();
                    if self.try_load_folder(&path).is_ok() {
                        self.state = AppState::OnProject(LeftPannelState::Folders);
                    };
                }
            });

            if self.data.projects_opened.is_empty() {
                return;
            }

            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                ui.set_max_width(300.0);
                egui::Grid::new("recent_projects")
                    .num_columns(1)
                    .show(ui, |ui| {
                        ui.label("Recent projects");
                        ui.end_row();

                        for (path, _) in self.get_recent_projects(10) {
                            if ui
                                .button(path.clone())
                                .on_hover_text("Click to open project")
                                .clicked()
                                && self
                                    .try_load_folder(&PathBuf::from_str(&path).unwrap())
                                    .is_ok()
                            {
                                self.state = AppState::OnProject(LeftPannelState::Folders);
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
