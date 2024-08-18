use std::path::PathBuf;

use egui::{CollapsingHeader, Color32, Sense, Ui, Vec2};
use rfd::FileDialog;

use crate::app::{folder_tree::Folder, LogixApp};

impl LogixApp {
    pub fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .min_width(120.0)
            .show(ctx, |ui| {
                if self.folder.is_none() {
                    ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        if ui.button("Open folder").clicked() {
                            let new_folder = FileDialog::new().pick_folder();
                            if let Some(new_folder) = new_folder {
                                self.folder = Some(Folder::from_pathbuf(&new_folder));
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
                                self.load_board(&file);
                            }
                            self.selected_file = new_file;
                        }
                    });
            });
    }
}

impl Folder {
    fn ui_impl(&mut self, ui: &mut Ui, selected_file: Option<&PathBuf>) -> Option<PathBuf> {
        let mut new_file = selected_file.cloned();
        for folder in self.folders.iter_mut() {
            let name = folder.current_path.file_name().unwrap().to_str().unwrap();
            CollapsingHeader::new(name).show(ui, |ui| {
                new_file = folder.ui_impl(ui, selected_file);
            });
        }

        for file in self.files.iter() {
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
