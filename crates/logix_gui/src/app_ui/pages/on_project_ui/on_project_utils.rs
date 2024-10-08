use std::path::PathBuf;

use egui::KeyboardShortcut;
use log::error;
use rfd::FileDialog;

use crate::app_ui::{
    app_state::{AppState, LeftPannelState},
    board::Board,
    board_editing::BoardEditing,
    errors::OpenBoardError,
    logix_app::LogixApp,
    shortcuts::shortcut_string,
};

impl LogixApp {
    pub fn named_cmd_shorcut(cmd: &str, shortcut: KeyboardShortcut) -> String {
        format!("{} ({})", cmd, shortcut_string(shortcut))
    }

    pub fn exist_active_board(&self) -> bool {
        !self.board_tabs.is_empty()
    }

    pub fn board_editing_mut(&mut self) -> &mut BoardEditing {
        assert!(
            !self.board_tabs.is_empty(),
            "There is no active board to edit"
        );
        &mut self.board_tabs[self.current_tab]
    }

    pub fn board_editing(&self) -> &BoardEditing {
        assert!(
            !self.board_tabs.is_empty(),
            "There is no active board to edit"
        );
        &self.board_tabs[self.current_tab]
    }

    pub fn set_current_tab(&mut self, idx: usize) {
        assert!(idx < self.board_tabs.len());
        // Only change if the tab is different
        if idx != self.current_tab {
            self.current_tab = idx;
            self.selected_file = Some(self.board_tabs[idx].file.clone());
            self.board_editing_mut()
                .board
                .reload_imported_components()
                .expect("Failed to reload imported components when changing to tab");
        }
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board_editing_mut().board
    }

    /// Gets a name that is not already in the project folder
    fn get_board_default_name(&self) -> String {
        let mut i = 0;
        let project_folder = self.folder.current_path.clone();
        let tabs_files: Vec<String> = self
            .board_tabs
            .iter()
            .map(|board_edit| {
                board_edit
                    .file
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();
        let mut name = format!("untitled_{i}.lgxb");
        while project_folder.join(&name).exists() || tabs_files.contains(&name) {
            i += 1;
            name = format!("untitled_{i}.lgxb");
        }
        name
    }

    pub fn new_board(&mut self) {
        let board = BoardEditing {
            file: self.folder.current_path.join(self.get_board_default_name()),
            project_folder: self.folder.current_path.clone(),
            ..Default::default()
        };

        self.board_tabs.push(board);
        self.current_tab = self.board_tabs.len() - 1;
    }

    pub fn load_board(&mut self, path: &PathBuf) -> Result<(), OpenBoardError> {
        // Check first if it is already open in a tab
        for (i, tab) in self.board_tabs.iter().enumerate() {
            if tab.file == path.clone() {
                self.set_current_tab(i);
                return Ok(());
            }
        }

        let comp = Board::open(path).inspect_err(|err| {
            self.notify_err(err.to_string());
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
            file: path.clone(),
            project_folder: self.folder.current_path.clone(),
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

    pub fn save_current_board(&mut self) {
        let path = self.board_editing_mut().file.clone();
        let res = self.board_mut().save(&path);
        self.notify_if_err(res);
    }

    pub fn save_current_board_as(&mut self) {
        let file = FileDialog::new()
            .set_directory(self.folder.current_path.clone())
            .set_file_name(
                self.board_editing()
                    .file
                    .file_name()
                    .unwrap()
                    .to_string_lossy(),
            )
            .add_filter("Logix Board", &["lgxb"]);
        if let Some(mut new_file) = file.save_file() {
            if new_file.extension().is_none() || new_file.extension().unwrap() != "lgxb" {
                new_file = new_file.with_extension("lgxb");
            }
            match self.board_mut().save(&new_file) {
                Ok(()) => {
                    self.board_editing_mut().file.clone_from(&new_file);
                    self.selected_file = Some(new_file);
                }
                Err(err) => self.notify_err(err.to_string()),
            }
        }
    }

    pub fn run_current_sim(&mut self) {
        if let Err(err) = self.board_editing_mut().run_sim() {
            error!("Failed to run simulation: {}", err);
            self.board_editing_mut().stop_sim();
        }
    }

    pub fn pause_resume_current_sim(&mut self) {
        self.board_editing_mut().pause_resume_sim();
    }

    pub fn stop_current_sim(&mut self) {
        self.board_editing_mut().stop_sim();
        self.state = AppState::OnProject(LeftPannelState::Folders);
    }
}
