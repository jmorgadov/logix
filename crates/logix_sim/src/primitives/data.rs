use std::fmt::Display;
use std::ops as std_ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Data {
    pub value: usize,
    pub size: u8,
}

impl Data {
    pub fn new(value: usize, size: u8) -> Self {
        Data { value, size }
    }

    pub fn high() -> Self {
        Data { value: 1, size: 1 }
    }

    pub fn low() -> Self {
        Data { value: 0, size: 1 }
    }

    pub fn bit(value: bool) -> Self {
        Data {
            value: value as usize,
            size: 1,
        }
    }

    pub fn as_bool(&self) -> bool {
        self.value != 0
    }

    pub fn set_bit(&mut self, value: bool) {
        self.value = value as usize;
    }

    pub fn set_bit_at(&mut self, index: u8, value: bool) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        if value {
            self.value |= 1 << index;
        } else {
            self.value &= !(1 << index);
        }
    }

    pub fn get_bit_at(&self, index: u8) -> bool {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        (self.value >> index) & 1 != 0
    }

    pub fn set_data(&mut self, value: usize) {
        self.value = value & ((1 << self.size) - 1);
    }

    pub fn set_from(&mut self, other: Data) {
        if self.size != other.size {
            panic!("Different sizes");
        }
        self.value = other.value;
    }

    pub fn show_vec(values: &Vec<Data>) -> String {
        values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.size {
            match (self.value >> i) & 1 {
                1 => write!(f, "🟩")?,
                0 => write!(f, "⬛")?,
                _ => unreachable!(),
            }
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let d1 = Data::new(0b1010, 4);
        let d2 = Data::new(0b1100, 4);

        assert_eq!((d1 & d2).value, 0b1000);
        assert_eq!((d1 | d2).value, 0b1110);
        assert_eq!((d1 ^ d2).value, 0b0110);
        assert_eq!((!d1).value, 0b0101);
    }

    #[test]
    fn test_bool() {
        let d1 = Data::bit(true);
        let d2 = Data::bit(false);

        assert_eq!(d1.as_bool(), true);
        assert_eq!(d2.as_bool(), false);
    }

    #[test]
    fn test_display() {
        let d1 = Data::bit(true);
        let d2 = Data::bit(false);

        assert_eq!(format!("{}", d1), "🟩");
        assert_eq!(format!("{}", d2), "⬛");
    }
}
