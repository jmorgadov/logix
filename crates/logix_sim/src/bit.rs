use std::ops;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    #[default]
    ONE,
    ZERO,
}

pub const ONE: Bit = Bit::ONE;
pub const ZERO: Bit = Bit::ZERO;

pub fn fmt_bit(bit: &Bit) -> char {
    match bit {
        Bit::ONE => '🟩',
        Bit::ZERO => '⬛',
    }
}

impl ops::BitOr for Bit {
    type Output = Bit;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ONE, _) => ONE,
            (_, ONE) => ONE,
            _ => ZERO,
        }
    }
}


impl ops::BitAnd for Bit {
    type Output = Bit;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (_, ZERO) => ZERO,
            (ZERO, _) => ZERO,
            _ => ONE,
        }
    }
}

impl ops::BitXor for Bit {
    type Output = Bit;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ONE, ONE) => ZERO,
            (ONE, ZERO) => ONE,
            (ZERO, ONE) => ONE,
            (ZERO, ZERO) => ZERO,
        }
    }
}

impl ops::Not for Bit {
    type Output = Bit;

    fn not(self) -> Self::Output {
        match self {
            ONE => ZERO,
            ZERO => ONE,
        }
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { ONE } else { ZERO }
    }
}
