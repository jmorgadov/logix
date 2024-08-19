pub enum WireDir {
    Horizontal,
    Vertical,
}

impl WireDir {
    pub fn opposite(&self) -> Self {
        match self {
            WireDir::Horizontal => WireDir::Vertical,
            WireDir::Vertical => WireDir::Horizontal,
        }
    }

    pub fn get_dir(len: usize) -> Self {
        match len % 2 {
            0 => WireDir::Vertical,
            _ => WireDir::Horizontal,
        }
    }
}
