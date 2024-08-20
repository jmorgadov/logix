use crate::app::{comp_board::ComponentBoard, folder_tree::Folder};
use egui::{Color32, Sense, Stroke};
use logix_sim::{flatten::FlattenComponent, Simulator};
use rfd::FileDialog;
use std::path::PathBuf;

use super::{board_editing::BoardEditing, shortcuts};

pub struct LogixApp {
    pub folder: Option<Folder>,
    pub selected_file: Option<PathBuf>,
    pub board_tabs: Vec<BoardEditing>,
    pub current_tab: usize,
}

impl Default for LogixApp {
    fn default() -> Self {
        let current_folder = std::env::current_dir();
        let folder = match current_folder {
            Ok(folder) => Some(Folder::from_pathbuf(&folder)),
            Err(_) => None,
        };
        Self {
            folder,
            selected_file: None,
            board_tabs: vec![Default::default()],
            current_tab: 0,
        }
    }
}

impl LogixApp {
    pub fn board_editing_mut(&mut self) -> &mut BoardEditing {
        if self.board_tabs.is_empty() {
            self.board_tabs.push(Default::default());
            self.current_tab = 0;
        }
        &mut self.board_tabs[self.current_tab]
    }

    pub fn board_editing(&mut self) -> &BoardEditing {
        if self.board_tabs.is_empty() {
            self.board_tabs.push(Default::default());
            self.current_tab = 0;
        }
        &self.board_tabs[self.current_tab]
    }

    pub fn set_current_tab(&mut self, idx: usize) -> Result<(), ()> {
        if idx < self.board_tabs.len() {
            self.current_tab = idx;
            self.selected_file = self.board_tabs[idx].file.clone();
            let _ = self.board_editing_mut().board.reload_imported_components();
            return Ok(());
        }
        Err(())
    }

    pub fn board(&mut self) -> &ComponentBoard {
        &self.board_editing().board
    }

    pub fn board_mut(&mut self) -> &mut ComponentBoard {
        &mut self.board_editing_mut().board
    }

    pub fn new_board(&mut self) {
        self.board_tabs.push(Default::default());
        self.current_tab = self.board_tabs.len() - 1;
    }

    pub fn load_board(&mut self, path: &PathBuf) {
        let comp_res = ComponentBoard::load(path);
        if let Ok(comp) = comp_res {
            let next_id = comp
                .components
                .iter()
                .map(|c| c.id)
                .max()
                .unwrap_or_default()
                + 1;

            let b_editing = BoardEditing {
                board: comp,
                file: Some(path.clone()),
                next_id,
                ..Default::default()
            };
            if self.board_tabs.len() == 1 && self.board_tabs[0].is_empty() {
                self.board_tabs[0] = b_editing;
            } else {
                self.board_tabs.push(b_editing);
                self.set_current_tab(self.board_tabs.len() - 1).unwrap();
            }
        }
    }

    pub fn save_current_board(&mut self) {
        let path = self.board_editing().file.clone();
        if let Some(file_path) = path {
            let _ = self.board().save(&file_path);
            return;
        }
        let mut file = FileDialog::new();
        if let Some(folder) = &self.folder {
            file = file.set_directory(folder.current_path.clone());
        }
        if let Some(new_folder) = file.pick_file() {
            let _ = self.board().save(&new_folder);
        }
    }

    pub fn run_current_sim(&mut self) {
        self.board_editing_mut().run_sim();
    }

    pub fn stop_current_sim(&mut self) {
        self.board_editing_mut().stop_sim()
    }

    pub fn draw_tabs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs")
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(0.0))
                    .fill(ctx.style().visuals.panel_fill),
            )
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let mut i = 0;
                    let mut next_current_tab = self.current_tab;
                    while i < self.board_tabs.len() {
                        let mut removed = false;
                        let color = if i == self.current_tab {
                            Color32::from_gray(35)
                        } else {
                            ui.style().visuals.panel_fill
                        };
                        egui::Frame::default().fill(color).show(ui, |ui| {
                            ui.allocate_space(egui::vec2(0.0, ui.available_height()));
                            ui.horizontal(|ui| {
                                ui.set_max_width(150.0);
                                let tab_label = if self.board_tabs[i].board.name.is_empty() {
                                    "Untitled"
                                } else {
                                    self.board_tabs[i].board.name.as_str()
                                };
                                let resp = ui
                                    .add(egui::Label::new(tab_label).truncate().selectable(false))
                                    .interact(Sense::click());

                                if resp.clicked() {
                                    next_current_tab = i;
                                }
                            });
                            if ui
                                .add(
                                    egui::Button::new("❌")
                                        .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                                        .fill(Color32::TRANSPARENT),
                                )
                                .clicked()
                            {
                                self.board_tabs.remove(i);
                                removed = true;

                                if i < self.current_tab
                                    || (i == self.current_tab
                                        && i == self.board_tabs.len()
                                        && i > 0)
                                {
                                    next_current_tab -= 1;
                                }
                            }
                        });

                        if !removed {
                            i += 1;
                        }
                    }
                    if self.board_tabs.is_empty() {
                        self.new_board();
                    }
                    let _ = self.set_current_tab(next_current_tab);
                });
            });
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.15);

        ctx.input_mut(|input| {
            if input.consume_shortcut(&shortcuts::SAVE) {
                self.save_current_board();
            }
            if input.consume_shortcut(&shortcuts::RUN) {
                self.run_current_sim();
            }
            if input.consume_shortcut(&shortcuts::STOP) {
                self.stop_current_sim();
            }
        });

        self.top_panel(ctx);
        self.left_panel(ctx);
        self.draw_tabs(ctx);
        self.board_editing_mut().show(ctx);
    }
}
