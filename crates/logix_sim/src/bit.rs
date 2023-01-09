use std::ops;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    #[default]
    UNK,
    ONE,
    ZERO,
}

pub const UNK: Bit = Bit::UNK;
pub const ONE: Bit = Bit::ONE;
pub const ZERO: Bit = Bit::ZERO;

pub fn fmt_bit(bit: &Bit) -> char {
    match bit {
        Bit::ONE => '🟩',
        Bit::ZERO => '⬛',
        Bit::UNK => '🟥',
    }
}

impl ops::BitOr for Bit {
    type Output = Bit;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ONE, _) => ONE,
            (_, ONE) => ONE,
            (UNK, UNK) => UNK,
            (UNK, ZERO) => UNK,
            (ZERO, UNK) => UNK,
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
            (UNK, UNK) => UNK,
            (UNK, ONE) => UNK,
            (ONE, UNK) => UNK,
            _ => ONE,
        }
    }
}

impl ops::BitXor for Bit {
    type Output = Bit;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (UNK, UNK) => UNK,
            (UNK, ONE) => UNK,
            (UNK, ZERO) => UNK,
            (ONE, UNK) => UNK,
            (ONE, ONE) => ZERO,
            (ONE, ZERO) => ONE,
            (ZERO, UNK) => UNK,
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
            _ => UNK,
        }
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { ONE } else { ZERO }
    }
}
