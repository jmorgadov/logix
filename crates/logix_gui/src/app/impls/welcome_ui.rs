use std::{path::PathBuf, str::FromStr};

use rfd::FileDialog;

use crate::app::{app_state::AppState, LogixApp};

impl LogixApp {
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
                        folder: Default::default(),
                        name: Default::default(),
                    };
                }
                if ui.button("Open a project").clicked() {
                    let new_folder = FileDialog::new().pick_folder();
                    let path = new_folder.unwrap().clone();
                    if self.try_load_folder(&path).is_ok() {
                        self.state = AppState::OnProject;
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

                        let projects = self.data.projects_opened.clone();
                        let mut sorted = projects.iter().collect::<Vec<(&String, &u64)>>();
                        sorted.sort_by(|a, b| b.1.cmp(a.1));

                        for (path, _) in sorted {
                            if ui
                                .button(egui::RichText::new(path))
                                .on_hover_text("Click to open project")
                                .clicked()
                                && self
                                    .try_load_folder(&PathBuf::from_str(path).unwrap())
                                    .is_ok()
                            {
                                self.state = AppState::OnProject;
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
