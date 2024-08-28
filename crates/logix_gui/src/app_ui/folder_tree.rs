use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Folder {
    pub current_path: PathBuf,
    pub folders: Vec<Folder>,
    pub files: Vec<PathBuf>,
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

        Ok(Self {
            current_path,
            folders,
            files,
        })
    }

    pub fn is_loaded(&self) -> bool {
        !self.current_path.as_os_str().is_empty()
    }
}
