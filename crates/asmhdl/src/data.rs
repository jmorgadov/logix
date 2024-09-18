use serde::{Deserialize, Serialize};
use std::ops as std_ops;

/// A value with a fixed size in bits.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Data {
    /// Value of the data
    pub value: usize,
    /// Size of the data in bits
    pub size: usize,
}

impl Data {
    /// Creates a new value with the given value and size
    pub fn new(value: usize, size: usize) -> Self {
        Self { value, size }
    }

    /// Single bit data with value 1
    pub fn high() -> Self {
        Self::new(1, 1)
    }

    /// Single bit data with value 0
    pub fn low() -> Self {
        Self::new(0, 1)
    }

    /// Sets the value of a bit
    pub fn set_bit(&mut self, bit: usize, value: bool) {
        if bit >= self.size {
            panic!("Bit index out of range");
        }
        self.value = (self.value & !(1 << bit)) | ((value as usize) << bit);
    }

    /// Gets the value of a bit
    pub fn get_bit(&self, bit: usize) -> bool {
        if bit >= self.size {
            panic!("Bit index out of range");
        }
        (self.value & (1 << bit)) != 0
    }

    /// Sets the value according to a boolean
    pub fn set_from_bool(&mut self, value: bool) {
        self.value = match value {
            true => 1,
            false => 0,
        };
    }

    /// Sets the value according to a number
    pub fn set_value(&mut self, value: usize) {
        assert!(value < (1 << self.size));
        self.value = value & ((1 << self.size) - 1);
    }

    /// Sets the value from another data
    pub fn clone_from(&mut self, value: &Data) {
        assert!(self.size == value.size);
        self.value = value.value;
    }

    /// False if the value is 0, true otherwise
    pub fn as_bool(&self) -> bool {
        self.value != 0
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::new(value as usize, 1)
    }
}

impl From<usize> for Data {
    fn from(value: usize) -> Self {
        Data::new(value, 64)
    }
}

impl From<(usize, usize)> for Data {
    fn from(value: (usize, usize)) -> Self {
        Data::new(value.0, value.1)
    }
}

impl From<&str> for Data {
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

impl std_ops::BitAnd for Data {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        if self.size != rhs.size {
            panic!("Bitwise and between different sizes");
        }
        Data::new(
            (self.value & rhs.value) & ((1 << self.size) - 1),
            self.size.max(rhs.size),
        )
    }
}

impl std_ops::BitOr for Data {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        if self.size != rhs.size {
            panic!("Bitwise or between different sizes");
        }
        Data::new(
            (self.value | rhs.value) & ((1 << self.size) - 1),
            self.size.max(rhs.size),
        )
    }
}

impl std_ops::BitXor for Data {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        if self.size != rhs.size {
            panic!("Bitwise xor between different sizes");
        }
        Data::new(
            (self.value ^ rhs.value) & ((1 << self.size) - 1),
            self.size.max(rhs.size),
        )
    }
}

impl std_ops::Not for Data {
    type Output = Self;

    fn not(self) -> Self {
        Data::new((!self.value) & ((1 << self.size) - 1), self.size)
    }
}
