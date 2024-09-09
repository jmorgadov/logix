use serde::{Deserialize, Serialize};
use std::ops as std_ops;

/// A value with a fixed size in bits.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct AsmValue {
    /// Value of the data
    pub value: usize,
    /// Size of the data in bits
    pub size: usize,
}

impl AsmValue {
    /// Creates a new value with the given value and size
    pub fn new(value: usize, size: usize) -> Self {
        Self { value, size }
    }

    /// Returns a value representing true
    ///
    /// Value: 1
    /// Size: 1
    pub fn true_val() -> Self {
        Self::new(1, 1)
    }

    /// Returns a value representing false
    ///
    /// Value: 0
    /// Size: 1
    pub fn false_val() -> Self {
        Self::new(0, 1)
    }

    /// Sets the value of a bit
    pub fn set_bit(&mut self, bit: usize, value: bool) {
        if bit >= self.size {
            panic!("Bit index out of range");
        }
        self.value = (self.value & !(1 << bit)) | ((value as usize) << bit);
    }

    /// False if the value is 0, true otherwise
    pub fn as_bool(&self) -> bool {
        self.value != 0
    }
}

impl From<bool> for AsmValue {
    fn from(value: bool) -> Self {
        AsmValue::new(value as usize, 1)
    }
}

impl From<usize> for AsmValue {
    fn from(value: usize) -> Self {
        AsmValue::new(value, 64)
    }
}

impl From<&str> for AsmValue {
    fn from(value: &str) -> Self {
        let size = value.len();
        let value = value
            .chars()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Invalid character"),
            })
            .fold(0, |acc, x| (acc << 1) | x);
        Self { value, size }
    }
}

impl std_ops::BitAnd for AsmValue {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        if self.size != rhs.size {
            panic!("Bitwise and between different sizes");
        }
        AsmValue::new(
            (self.value & rhs.value) & ((1 << self.size) - 1),
            self.size.max(rhs.size),
        )
    }
}

impl std_ops::BitOr for AsmValue {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        if self.size != rhs.size {
            panic!("Bitwise or between different sizes");
        }
        AsmValue::new(
            (self.value | rhs.value) & ((1 << self.size) - 1),
            self.size.max(rhs.size),
        )
    }
}

impl std_ops::BitXor for AsmValue {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        if self.size != rhs.size {
            panic!("Bitwise xor between different sizes");
        }
        AsmValue::new(
            (self.value ^ rhs.value) & ((1 << self.size) - 1),
            self.size.max(rhs.size),
        )
    }
}

impl std_ops::Not for AsmValue {
    type Output = Self;

    fn not(self) -> Self {
        AsmValue::new((!self.value) & ((1 << self.size) - 1), self.size)
    }
}
