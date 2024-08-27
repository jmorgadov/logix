use std::path::PathBuf;

use egui::{CollapsingHeader, Color32, Sense, Ui, Vec2};
use rfd::FileDialog;

use crate::app_ui::{
    app_state::{AppState, LeftPannelState},
    comp_board::IdMap,
    folder_tree::Folder,
    logix_app::LogixApp,
};

impl LogixApp {
    pub fn show_folders(&mut self, ui: &mut Ui) {
        if self.folder.is_none() {
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                if ui.button("Open folder").clicked() {
                    let new_folder = FileDialog::new().pick_folder();
                    if let Some(new_folder) = new_folder {
                        let _ = self.try_load_folder(&new_folder);
                    }
                }
            });
            return;
        }
        ui.heading(
            self.folder
                .as_ref()
                .unwrap()
                .current_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        egui::ScrollArea::vertical()
            .max_width(180.0)
            .show(ui, |ui| {
                let new_file = self
                    .folder
                    .as_mut()
                    .unwrap()
                    .ui_impl(ui, self.selected_file.as_ref());
                if new_file != self.selected_file {
                    if let Some(file) = new_file.clone() {
                        if self.load_board(&file).is_ok() {
                            self.selected_file = new_file;
                        }
                    }
                }
            });
    }

    pub fn show_board_tree(&mut self, ui: &mut Ui) {
        let mut curr_path = Vec::new();
        let main_id = self.board_editing().sim_ids.id;
        let selected_id = self
            .board_editing()
            .sim_at
            .as_ref()
            .map_or(main_id, |(path, _)| *path.last().unwrap());
        let path = self
            .board_editing_mut()
            .sim_ids
            .board_tree(ui, &mut curr_path, selected_id);

        if let Some(path) = path {
            self.board_editing_mut().set_sim_at(&path[1..]);
        }
    }

    pub fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .min_width(160.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::Button::new("🗁").fill(match &self.state {
                            AppState::OnProject(LeftPannelState::Folders) => {
                                Color32::from_rgb(50, 50, 50)
                            }
                            _ => Color32::TRANSPARENT,
                        }))
                        .clicked()
                    {
                        self.state = AppState::OnProject(LeftPannelState::Folders);
                    }

                    if self.board_editing().sim.is_some()
                        && ui
                            .add(egui::Button::new("💻").fill(match &self.state {
                                AppState::OnProject(LeftPannelState::Board) => {
                                    Color32::from_rgb(50, 50, 50)
                                }
                                _ => Color32::TRANSPARENT,
                            }))
                            .clicked()
                    {
                        self.state = AppState::OnProject(LeftPannelState::Board);
                    }
                });
                if let AppState::OnProject(state) = &mut self.state {
                    match state {
                        LeftPannelState::Folders => {
                            self.show_folders(ui);
                        }
                        LeftPannelState::Board => {
                            self.show_board_tree(ui);
                        }
                    }
                };
            });
    }
}

impl IdMap {
    pub fn board_tree(
        &mut self,
        ui: &mut Ui,
        slected_path: &mut Vec<usize>,
        selected_id: usize,
    ) -> Option<Vec<usize>> {
        let id = ui.id().with(("board_tree", self.id));

        let mut to_return = None;
        slected_path.push(self.id);
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
            .show_header(ui, |ui| {
                let text =
                    egui::RichText::new(self.name.clone()).color(if self.id == selected_id {
                        Color32::LIGHT_GREEN
                    } else {
                        Color32::WHITE
                    });
                if ui.button(text).clicked() {
                    to_return = Some(slected_path.clone());
                }
            })
            .body(|ui| {
                for sub in self.sub_ids.iter_mut().filter(|sub| sub.source.is_some()) {
                    to_return = sub
                        .board_tree(ui, slected_path, selected_id)
                        .or_else(|| to_return.clone());
                }
            });

        slected_path.pop();
        to_return
    }
}

impl Folder {
    fn ui_impl(&mut self, ui: &mut Ui, selected_file: Option<&PathBuf>) -> Option<PathBuf> {
        let mut new_file = selected_file.cloned();
        for folder in &mut self.folders {
            let name = folder.current_path.file_name().unwrap().to_str().unwrap();
            CollapsingHeader::new(name).show(ui, |ui| {
                new_file = folder.ui_impl(ui, selected_file);
            });
        }

        for file in &self.files {
            let name = file.file_name().unwrap().to_str().unwrap();
            let mut color = Color32::TRANSPARENT;
            if let Some(selected_file) = selected_file {
                if file == selected_file {
                    color = Color32::from_rgb(40, 40, 40);
                }
            }
            egui::Frame::default().fill(color).show(ui, |ui| {
                ui.allocate_space(Vec2::new(ui.available_width(), 0.0));
                let resp = ui.add(
                    egui::Label::new(name)
                        .selectable(false)
                        .wrap_mode(egui::TextWrapMode::Truncate)
                        .sense(Sense::click()),
                );
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.double_clicked() {
                    new_file = Some(file.clone());
                }
            });
        }

        new_file
    }
}
