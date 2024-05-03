use std::fmt::Display;
use std::ops as std_ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Bit {
    High,
    #[default]
    Low,
}

impl Bit {
    pub fn show_vec(bits: &Vec<Bit>) -> String {
        bits.iter().map(|b| b.to_string()).collect()
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            Bit::High
        } else {
            Bit::Low
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Bit::High => true,
            Bit::Low => false,
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bit::High => write!(f, "🟩"),
            Bit::Low => write!(f, "⬛"),
        }
    }
}

impl std_ops::BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Bit::High, Bit::High) => Bit::High,
            (Bit::Low, _) | (_, Bit::Low) => Bit::Low,
        }
    }
}

impl std_ops::BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Bit::Low, Bit::Low) => Bit::Low,
            (Bit::High, _) | (_, Bit::High) => Bit::High,
        }
    }
}

impl std_ops::BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Bit::High, Bit::Low) | (Bit::Low, Bit::High) => Bit::High,
            (Bit::High, Bit::High) | (Bit::Low, Bit::Low) => Bit::Low,
        }
    }
}

impl std_ops::Not for Bit {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Bit::High => Bit::Low,
            Bit::Low => Bit::High,
            // _ => Bit::Und,
        }
    }
}
