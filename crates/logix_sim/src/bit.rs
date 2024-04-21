use std::ops as std_ops;

pub type Bit = bool;

pub fn fmt_bit(bit: &Bit) -> char {
    match bit {
        true => '🟩',
        false => '⬛',
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct BitArray {
    pub len: u8,
    bits: u64,
    mask: u64,
}

pub fn fmt_bit_array(bits: &BitArray) -> String {
    let mut s = String::new();
    for i in 0..bits.len {
        s.push(fmt_bit(&bits.get_bit(i)));
    }
    s
}

impl BitArray {
    pub fn new(len: u8) -> Self {
        let mask = (1 << len) - 1;
        Self { bits: 0, len, mask }
    }

    pub fn set(&mut self, val: u64) {
        self.bits = val & self.mask;
    }

    pub fn set_bit(&mut self, idx: u8, val: Bit) {
        if val {
            self.bits |= 1 << idx;
        } else {
            self.bits &= !(1 << idx);
        }
    }

    pub fn set_bits(&mut self, bits: u64) {
        self.bits = bits & self.mask;
    }

    pub fn get(&self) -> u64 {
        self.bits
    }

    pub fn get_bit(&self, idx: u8) -> Bit {
        (self.bits & (1 << idx)) != 0
    }

    pub fn get_bits(&self) -> u64 {
        self.bits
    }

}

impl Default for BitArray {
    fn default() -> Self {
        Self::new(1)
    }
}

impl std_ops::BitAnd for BitArray {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        assert!(self.len == rhs.len, "BitArray length mismatch");
        let bits = self.bits & rhs.bits;
        Self {
            bits,
            len: self.len,
            mask: self.mask,
        }
    }
}

impl std_ops::BitOr for BitArray {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        assert!(self.len == rhs.len, "BitArray length mismatch");
        let bits = self.bits | rhs.bits;
        Self {
            bits,
            len: self.len,
            mask: self.mask,
        }
    }
}

impl std_ops::BitXor for BitArray {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        assert!(self.len == rhs.len, "BitArray length mismatch");
        let bits = self.bits ^ rhs.bits;
        Self {
            bits,
            len: self.len,
            mask: self.mask,
        }
    }
}

impl std_ops::Not for BitArray {
    type Output = Self;

    fn not(self) -> Self {
        let bits = !self.bits & self.mask;
        Self {
            bits,
            len: self.len,
            mask: self.mask,
        }
    }
}

impl std_ops::Shl<u8> for BitArray {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self {
        let bits = self.bits << rhs;
        Self {
            bits: bits & self.mask,
            len: self.len,
            mask: self.mask,
        }
    }
}

impl std_ops::Shr<u8> for BitArray {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self {
        let bits = self.bits >> rhs;
        Self {
            bits,
            len: self.len,
            mask: self.mask,
        }
    }
}
