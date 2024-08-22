use crate::app::{comp_board::ComponentBoard, folder_tree::Folder};
use egui::FontId;
use egui_notify::Toasts;
use log::*;
use rfd::FileDialog;
use std::{fmt::Display, path::PathBuf};

use super::{
    app_config::AppSettings, app_data::AppData, app_state::AppState, board_editing::BoardEditing,
    errors::OpenBoardError, shortcuts,
};

pub struct LogixApp {
    pub folder: Option<Folder>,
    pub selected_file: Option<PathBuf>,
    pub board_tabs: Vec<BoardEditing>,
    pub current_tab: usize,
    pub toasts: Toasts,
    pub state: AppState,

    pub settings: AppSettings,
    pub data: AppData,

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
            settings: AppSettings::default(),
            data: AppData::default(),
            new_project_folder: Default::default(),
            new_project_name: Default::default(),
        }
    }
}

impl LogixApp {
    pub fn from_folder(path: &PathBuf) -> Result<Self, std::io::Error> {
        let folder = Folder::from_pathbuf(path)?;
        let mut app = LogixApp {
            folder: Some(folder),
            state: AppState::OnProject,
            ..Default::default()
        };
        app.try_load_folder(path)?;
        Ok(app)
    }

    pub fn data_dir() -> PathBuf {
        let mut data_dir = dirs::data_dir().expect("Failed to get data dir");
        data_dir.push("logix");
        data_dir.push("data.json");
        data_dir
    }

    pub fn config_dir() -> PathBuf {
        let mut config_dir = dirs::config_dir().expect("Failed to get config dir");
        config_dir.push("logix");
        config_dir.push("config.json");
        config_dir
    }

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

    pub fn update_data(&mut self, data_upd: impl FnOnce(&mut AppData)) {
        data_upd(&mut self.data);
        let data_dir = Self::data_dir();
        std::fs::create_dir_all(data_dir.parent().unwrap()).unwrap();
        std::fs::write(data_dir, serde_json::to_string(&self.data).unwrap())
            .expect("Failed to write data file");
    }

    // TODO: Add this when implementing settings state
    //
    // pub fn update_settings(&mut self, settings_upd: impl FnOnce(&mut AppSettings)) {
    //     settings_upd(&mut self.settings);
    //     let config_dir = Self::config_dir();
    //     std::fs::create_dir_all(config_dir.parent().unwrap()).unwrap();
    //     std::fs::write(config_dir, serde_json::to_string(&self.settings).unwrap())
    //         .expect("Failed to write config file");
    // }

    pub fn try_load_folder(&mut self, path: &PathBuf) -> Result<(), std::io::Error> {
        let folder_res = Folder::from_pathbuf(path);
        match folder_res {
            Ok(folder) => {
                self.folder = Some(folder);
                std::env::set_current_dir(path.clone()).unwrap();
                self.update_data(|data| {
                    let current_dir = std::env::current_dir().unwrap();
                    let path = current_dir.to_str().unwrap();
                    data.projects_opened.insert(
                        path.to_string(),
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    );
                });
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

    pub fn load_config_and_data(&mut self) {
        let data_dir = Self::data_dir();
        let data = std::fs::read_to_string(data_dir);
        if let Ok(data) = data {
            let data: AppData = serde_json::from_str(&data).unwrap_or_default();
            self.data = data;
        }

        let config_dir = Self::config_dir();
        let config = std::fs::read_to_string(config_dir);
        if let Ok(config) = config {
            let config: AppSettings = serde_json::from_str(&config).unwrap_or_default();
            self.settings = config;
        }
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_config_and_data();
        ctx.set_pixels_per_point(self.settings.zoom);
        ctx.style_mut(|style| {
            style.visuals.button_frame = false;
        });

        self.draw_app(ctx);
    }
}
