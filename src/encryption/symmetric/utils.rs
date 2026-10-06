const ALPHABET_SIZE: i64 = 26;

/// Shifts an ASCII letter, preserving its case. Callers must validate `c`.
pub fn shift_char(c: char, shift: i32) -> char {
    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
    let offset = (c as i64 - i64::from(base) + i64::from(shift)).rem_euclid(ALPHABET_SIZE) as u8;

    (base + offset) as char
}

#[cfg(test)]
#[path = "../tests/utils.rs"]
mod tests;
