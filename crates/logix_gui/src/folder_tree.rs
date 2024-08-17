use std::path::PathBuf;

pub struct Folder {
    pub current_path: PathBuf,
    pub folders: Vec<Folder>,
    pub files: Vec<PathBuf>,
}

impl Folder {
    pub fn from_pathbuf(path: &PathBuf) -> Self {
        let current_path = path.clone();
        let mut folders = vec![];
        let mut files = vec![];

        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                folders.push(Folder::from_pathbuf(&path));
            } else {
                files.push(path);
            }
        }

        Folder {
            current_path,
            folders,
            files,
        }
    }
}
