pub type Bit = bool;

pub fn fmt_bit(bit: &Bit) -> char {
    match bit {
        true => '🟩',
        false => '⬛',
    }
}