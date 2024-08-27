use std::{path::PathBuf, str::FromStr};

use egui::Vec2;
use rfd::FileDialog;

use crate::app_ui::{
    app_state::{AppState, LeftPannelState},
    logix_app::LogixApp,
};

impl LogixApp {
    fn build_path(folder: &str, name: &str) -> String {
        let mut path = String::new();
        path.push_str(folder);
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(name);
        if !path.ends_with('/') {
            path.push('/');
        }
        path
    }

    pub fn draw_new_project(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 4.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Create a new project").size(30.0),
                ));
                ui.add_space(20.0);
            });
            ui.vertical_centered(|ui| {
                ui.set_max_width(250.0);

                egui::Grid::new("new_project_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(self.state.new_project_name())
                                .hint_text("Project name"),
                        );
                        ui.label("ℹ").on_hover_text(
                            "If empty, the selected folder\nwill be used as project root",
                        );
                        ui.end_row();
                        ui.add(
                            egui::TextEdit::singleline(self.state.new_project_folder())
                                .min_size(Vec2::new(200.0, 0.0))
                                .hint_text("Project folder"),
                        );
                        if ui.button("Select").clicked() {
                            let new_folder = FileDialog::new().pick_folder();
                            if let Some(new_folder) = new_folder {
                                let path = new_folder;
                                *self.state.new_project_folder() =
                                    path.to_str().unwrap().to_string();
                            }
                        }
                        ui.end_row();
                        if !self.state.new_project_folder().is_empty() {
                            ui.add(egui::Label::new(
                                egui::RichText::new(format!(
                                    "Create at: {}",
                                    Self::build_path(
                                        &self.state.new_project_folder().clone(),
                                        self.state.new_project_name()
                                    )
                                ))
                                .small(),
                            ));
                            ui.end_row();
                        }
                    });

                let folder = self.state.new_project_folder().clone();
                let name = self.state.new_project_name().clone();
                ui.add_space(20.0);
                let path_res = PathBuf::from_str(&folder);
                let valid =
                    !folder.is_empty() && path_res.is_ok() && path_res.clone().unwrap().exists();
                ui.add_enabled_ui(valid, |ui| {
                    let path = path_res.unwrap();
                    if ui.button(egui::RichText::new("Create")).clicked() {
                        let path = path.join(name);
                        match std::fs::create_dir_all(&path) {
                            Ok(()) => {
                                if self.try_load_folder(&path).is_ok() {
                                    self.state = AppState::OnProject(LeftPannelState::Folders);
                                }
                            }
                            Err(_) => {
                                self.notify_err("Failed to create project folder");
                            }
                        }
                    }
                });
                if ui.button(egui::RichText::new("Back")).clicked() {
                    self.state = AppState::OnWelcome;
                }
            });
        });
    }
}
