use futures::{
    channel::mpsc::{channel, Receiver},
    executor::block_on,
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::PathBuf, thread};

#[derive(Debug, Default)]
pub struct Folder {
    pub current_path: PathBuf,
    folders: Vec<Folder>,
    files: Vec<PathBuf>,
    rx: Option<Receiver<bool>>,
}

impl Folder {
    pub fn from_pathbuf(path: &PathBuf) -> Result<Self, std::io::Error> {
        let current_path = path.clone();
        let mut folders = vec![];
        let mut files = vec![];

        for entry in std::fs::read_dir(path)? {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                folders.push(Self::from_pathbuf(&path)?);
            } else {
                files.push(path);
            }
        }

        let (mut ftx, frx) = channel(1);
        let path = path.clone();
        thread::spawn(move || {
            block_on(async {
                let (mut watcher, mut rx) =
                    Self::async_watcher().expect("failed to create watcher");

                watcher
                    .watch(path.as_path(), RecursiveMode::Recursive)
                    .expect("failed to watch folder");

                while let Some(res) = rx.next().await {
                    match res {
                        Ok(_event) => ftx.send(true).await.unwrap(),

                        Err(e) => println!("watch error: {e:?}"),
                    }
                }
            });
        });

        Ok(Self {
            current_path,
            folders,
            files,
            rx: Some(frx),
        })
    }

    pub fn is_loaded(&self) -> bool {
        !self.current_path.as_os_str().is_empty()
    }

    pub fn folders(&mut self) -> &mut Vec<Self> {
        if let Some(rx) = self.rx.as_mut() {
            if matches!(rx.try_next(), Ok(Some(true))) {
                self.reload().expect("failed to reload");
            }
        }
        &mut self.folders
    }

    pub fn files(&mut self) -> &mut Vec<PathBuf> {
        if let Some(rx) = self.rx.as_mut() {
            if matches!(rx.try_next(), Ok(Some(true))) {
                self.reload().expect("failed to reload");
            }
        }
        &mut self.files
    }

    pub fn reload(&mut self) -> Result<(), std::io::Error> {
        self.folders.clear();
        self.files.clear();
        for entry in std::fs::read_dir(self.current_path.clone())? {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                self.folders.push(Self::from_pathbuf(&path)?);
            } else {
                self.files.push(path);
            }
        }

        Ok(())
    }

    fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
        let (mut tx, rx) = channel(1);
        let watcher = RecommendedWatcher::new(
            move |res| {
                block_on(async {
                    tx.send(res).await.unwrap();
                });
            },
            Config::default(),
        )?;

        Ok((watcher, rx))
    }
}
