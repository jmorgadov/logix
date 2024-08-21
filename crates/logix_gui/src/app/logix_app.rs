use crate::app::{comp_board::ComponentBoard, folder_tree::Folder};
use eframe::Result;
use egui::{Color32, FontId, Sense, Stroke, Vec2};
use egui_notify::Toasts;
use log::*;
use rfd::FileDialog;
use std::{fmt::Display, path::PathBuf, str::FromStr};

use super::{board_editing::BoardEditing, errors::OpenBoardError, shortcuts};

#[derive(Debug, Default, Clone)]
pub enum AppState {
    #[default]
    OnWelcome,
    CreatingNewProject {
        folder: String,
        name: String,
    },
    OnProject,
}

impl AppState {
    pub fn new_project_folder(&mut self) -> &mut String {
        match self {
            Self::CreatingNewProject { folder, name: _ } => folder,
            _ => panic!("Not in new project state"),
        }
    }

    pub fn new_project_name(&mut self) -> &mut String {
        match self {
            Self::CreatingNewProject { folder: _, name } => name,
            _ => panic!("Not in new project state"),
        }
    }
}

pub struct LogixApp {
    pub folder: Option<Folder>,
    pub selected_file: Option<PathBuf>,
    pub board_tabs: Vec<BoardEditing>,
    pub current_tab: usize,
    pub toasts: Toasts,
    pub state: AppState,

    pub new_project_folder: String,
    pub new_project_name: String,
}

impl Default for LogixApp {
    fn default() -> Self {
        Self {
            folder: None,
            selected_file: None,
            board_tabs: vec![Default::default()],
            current_tab: 0,
            toasts: Toasts::new().with_default_font(FontId::proportional(10.0)),
            state: AppState::default(),
            new_project_folder: Default::default(),
            new_project_name: Default::default(),
        }
    }
}

impl LogixApp {
    pub fn notify_err(&mut self, err: impl Into<String>) {
        self.toasts.error(err).set_closable(true);
    }

    pub fn notify_if_err<T, E>(&mut self, res: Result<T, E>) -> Option<T>
    where
        E: Display,
    {
        match res {
            Ok(val) => Some(val),
            Err(err) => {
                self.notify_err(err.to_string());
                None
            }
        }
    }

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

    pub fn set_current_tab(&mut self, idx: usize) {
        assert!(idx < self.board_tabs.len());
        // Only change if the tab is different
        if idx != self.current_tab {
            self.current_tab = idx;
            self.selected_file = self.board_tabs[idx].file.clone();
            self.board_editing_mut()
                .board
                .reload_imported_components()
                .expect("Failed to reload imported components when changing to tab");
        }
    }

    pub fn board(&mut self) -> &ComponentBoard {
        &self.board_editing().board
    }

    pub fn new_board(&mut self) {
        self.board_tabs.push(BoardEditing::default());
        self.current_tab = self.board_tabs.len() - 1;
    }

    pub fn load_board(&mut self, path: &PathBuf) -> Result<(), OpenBoardError> {
        // Check first if it is already open in a tab
        for (i, tab) in self.board_tabs.iter().enumerate() {
            if tab.file == Some(path.clone()) {
                self.set_current_tab(i);
                return Ok(());
            }
        }

        let comp = ComponentBoard::open(path).map_err(|err| {
            self.notify_err(err.to_string());
            err
        })?;

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
            // If there is only one tab and it is empty, replace it
            self.board_tabs[0] = b_editing;
        } else {
            // Otherwise, add a new tab
            self.board_tabs.push(b_editing);
            self.set_current_tab(self.board_tabs.len() - 1);
        }

        Ok(())
    }

    pub fn try_load_folder(&mut self, path: &PathBuf) -> Result<(), std::io::Error> {
        let folder_res = Folder::from_pathbuf(path);
        match folder_res {
            Ok(folder) => {
                self.folder = Some(folder);
                std::env::set_current_dir(path.clone()).unwrap();
                Ok(())
            }
            Err(err) => {
                self.notify_err(format!("Failed to load folder: {}", err));
                Err(err)
            }
        }
    }

    pub fn save_current_board(&mut self) {
        let path = self.board_editing().file.clone();
        if let Some(file_path) = path {
            let res = self.board().save(&file_path);
            self.notify_if_err(res);
            return;
        }
        let mut file = FileDialog::new();
        if let Some(folder) = &self.folder {
            file = file.set_directory(folder.current_path.clone());
        }
        if let Some(new_folder) = file.pick_file() {
            let res = self.board().save(&new_folder);
            self.notify_if_err(res);
        }
    }

    pub fn run_current_sim(&mut self) {
        if let Err(err) = self.board_editing_mut().run_sim() {
            error!("Failed to run simulation: {}", err);
            self.board_editing_mut().stop_sim();
        }
    }

    pub fn stop_current_sim(&mut self) {
        self.board_editing_mut().stop_sim();
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
                        self.selected_file = None;
                        self.new_board();
                    }
                    self.set_current_tab(next_current_tab);
                });
            });
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
        });
    }

    fn draw_new_project(&mut self, ctx: &egui::Context) {
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
                        ui.end_row();
                        ui.add(
                            egui::TextEdit::singleline(self.state.new_project_folder())
                                .min_size(Vec2::new(200.0, 0.0))
                                .hint_text("Project folder"),
                        );
                        if ui.button("Select").clicked() {
                            let new_folder = FileDialog::new().pick_folder();
                            if let Some(new_folder) = new_folder {
                                let path = new_folder.clone();
                                *self.state.new_project_folder() =
                                    path.to_str().unwrap().to_string();
                            }
                        }
                        ui.end_row();
                        if !self.state.new_project_folder().is_empty() {
                            ui.add(egui::Label::new(
                                egui::RichText::new(format!(
                                    "Create at: {}/{}/",
                                    self.state.new_project_folder().clone(),
                                    self.state.new_project_name().clone()
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
                    if ui
                        .button(egui::RichText::new("Create").size(18.0))
                        .clicked()
                    {
                        let path = path.join(name);
                        match std::fs::create_dir_all(&path) {
                            Ok(_) => {
                                if self.try_load_folder(&path).is_ok() {
                                    self.state = AppState::OnProject;
                                }
                            }
                            Err(_) => {
                                self.notify_err("Failed to create project folder");
                            }
                        }
                    }
                });
                if ui
                    .button(egui::RichText::new("Cancel").size(18.0))
                    .clicked()
                {
                    self.state = AppState::OnWelcome;
                }
            });
        });
    }

    fn draw_app(&mut self, ctx: &egui::Context) {
        match &mut self.state {
            AppState::OnWelcome => {
                self.draw_welcome_page(ctx);
            }
            AppState::CreatingNewProject { folder: _, name: _ } => {
                self.draw_new_project(ctx);
            }
            AppState::OnProject => {
                if self.folder.is_none() {
                    self.state = AppState::OnWelcome;
                    return;
                }
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
        self.toasts.show(ctx);
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.15);
        ctx.style_mut(|style| {
            style.visuals.button_frame = false;
        });

        self.draw_app(ctx);
    }
}
