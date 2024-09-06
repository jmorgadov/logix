use log::info;
use std::{fmt::Display, path::PathBuf};

use super::{
    app_config::AppSettings, app_data::AppData, folder_tree::Folder, library::Library,
    logix_app::LogixApp,
};

impl LogixApp {
    pub fn load_app(&mut self) {
        // Load data
        let data_dir = Self::data_dir();
        let data = std::fs::read_to_string(data_dir);
        if let Ok(data) = data {
            let data: AppData = serde_json::from_str(&data).unwrap_or_default();
            self.data = data;
        }

        // Load config
        let config_dir = Self::config_dir();
        let config = std::fs::read_to_string(config_dir);
        if let Ok(config) = config {
            let config: AppSettings = serde_json::from_str(&config).unwrap_or_default();
            self.settings = config;
        }

        // Load library
        self.library = Library::load();
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

    pub fn update_data<T>(
        &mut self,
        data_upd: impl FnOnce(&mut AppData) -> T,
    ) -> Result<T, std::io::Error> {
        let val = data_upd(&mut self.data);
        let data_dir = Self::data_dir();
        if !data_dir.exists() {
            std::fs::create_dir_all(data_dir.parent().unwrap())?;
        }
        info!("Data: {:?}", self.data);
        std::fs::write(data_dir, serde_json::to_string(&self.data).unwrap())?;
        Ok(val)
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
                self.folder = folder;
                std::env::set_current_dir(path.clone())?;
                self.update_data(|data| -> Result<_, std::io::Error> {
                    let current_dir = std::env::current_dir()?;
                    let path = current_dir.to_str().unwrap();
                    data.projects_opened.insert(
                        path.to_string(),
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    );
                    Ok(())
                })??;
                Ok(())
            }
            Err(err) => {
                self.notify_err(format!("Failed to load folder: {err}"));
                Err(err)
            }
        }
    }
}
