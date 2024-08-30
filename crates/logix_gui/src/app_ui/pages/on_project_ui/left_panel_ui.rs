use std::path::PathBuf;

use egui::{CollapsingHeader, Color32, Sense, Ui, Vec2};

use crate::app_ui::{
    app_state::{AppState, LeftPannelState},
    folder_tree::Folder,
    id_map::IdMap,
    logix_app::LogixApp,
};

impl LogixApp {
    pub fn show_folders(&mut self, ui: &mut Ui) {
        ui.heading(
            self.folder
                .current_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        egui::ScrollArea::vertical()
            .max_width(180.0)
            .show(ui, |ui| {
                let new_file = self.folder.ui_impl(ui, self.selected_file.as_ref());
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
        let mut selected_path = Vec::new();
        let mut current_path = vec![self.board_editing().sim_ids.id];
        current_path.extend_from_slice(
            self.board_editing()
                .sim_at
                .as_ref()
                .map_or(&[], |(path, _)| path.as_slice()),
        );
        let path = self.board_editing_mut().sim_ids.board_tree(
            ui,
            &mut selected_path,
            current_path.as_slice(),
        );

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

                    if self.exist_active_board()
                        && self.board_editing().sim.is_some()
                        && ui
                            .add(egui::Button::new("🖳").fill(match &self.state {
                                AppState::OnProject(LeftPannelState::Simulation) => {
                                    Color32::from_rgb(50, 50, 50)
                                }
                                _ => Color32::TRANSPARENT,
                            }))
                            .clicked()
                    {
                        self.state = AppState::OnProject(LeftPannelState::Simulation);
                    }
                });
                if let AppState::OnProject(state) = &mut self.state {
                    match state {
                        LeftPannelState::Folders => {
                            self.show_folders(ui);
                        }
                        LeftPannelState::Simulation => {
                            if self.is_sim_running() {
                                self.show_board_tree(ui);
                            }
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
        current_path: &[usize],
    ) -> Option<Vec<usize>> {
        let id = ui.id().with(("board_tree", self.id));

        let mut to_return = None;
        slected_path.push(self.id);
        let in_path = !current_path.is_empty() && self.id == current_path[0];
        let text =
            egui::RichText::new(self.name.clone()).color(if in_path && current_path.len() == 1 {
                Color32::LIGHT_GREEN
            } else {
                Color32::WHITE
            });
        let header = CollapsingHeader::new(text)
            .id_source(id)
            .default_open(in_path)
            .open(if in_path && current_path.len() > 1 {
                Some(true)
            } else {
                None
            })
            .show(ui, |ui| {
                for sub in self.sub_ids.iter_mut().filter(|sub| sub.source.is_some()) {
                    let next_current_path = current_path.get(1..).unwrap_or(&[]);
                    if let Some(new_selected_path) =
                        sub.board_tree(ui, slected_path, next_current_path)
                    {
                        to_return = Some(new_selected_path);
                    }
                }
            })
            .header_response;

        if header.clicked() {
            to_return = Some(slected_path.clone());
        }

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
                        .sense(Sense::click_and_drag()),
                );
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.double_clicked() {
                    new_file = Some(file.clone());
                }
                resp.dnd_set_drag_payload(file.clone());
            });
        }

        new_file
    }
}
