#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireDir {
    Horizontal,
    Vertical,
}

impl WireDir {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }

    pub const fn get_dir(len: usize) -> Self {
        match len % 2 {
            0 => Self::Vertical,
            _ => Self::Horizontal,
        }
    }
}
