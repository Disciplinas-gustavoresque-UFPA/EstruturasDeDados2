const ALPHABET_SIZE: i32 = 26;

/// Shifts an ASCII letter, preserving its case. Callers must validate `c`.
pub fn shift_char(c: char, shift: i32) -> char {
    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
    let offset = (c as i32 - i32::from(base) + shift.rem_euclid(ALPHABET_SIZE))
        .rem_euclid(ALPHABET_SIZE) as u8;

    (base + offset) as char
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_every_ascii_letter_and_shift() {
        for base in *b"Aa" {
            for letter in base..base + 26 {
                for shift in (-104..=104).chain([i32::MIN, i32::MAX]) {
                    let offset = (i64::from(letter - base) + i64::from(shift)).rem_euclid(26);
                    let expected = char::from(base + offset as u8);
                    assert_eq!(super::shift_char(char::from(letter), shift), expected);
                }
            }
        }
    }
}
