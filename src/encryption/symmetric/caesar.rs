use super::shift_char;

/// Shifts ASCII letters by `shift`, preserving case and all other characters.
/// All `i32` shifts are supported modulo 26. Decrypt with the opposite of the
/// normalized shift: `-shift.rem_euclid(26)`.
pub fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            shift_char(c, shift)
        })
        .collect()
}

#[cfg(test)]
#[path = "../tests/caesar.rs"]
mod tests;
