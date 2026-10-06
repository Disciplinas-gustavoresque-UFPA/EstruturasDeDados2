use super::shift_char;

/// Encrypts when `encrypt` is true and decrypts otherwise, preserving case.
/// Only ASCII letters in `text` advance the repeating key; other characters
/// remain unchanged. The key is Unicode-uppercased before retaining ASCII
/// letters, so characters such as `ß` expand to usable key letters (`SS`).
///
/// # Panics
///
/// Panics if the uppercased key has no ASCII letters, even for empty text.
pub fn vigenere_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let key_shifts: Vec<i32> = key
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c as i32 - b'A' as i32)
        .collect();

    assert!(
        !key_shifts.is_empty(),
        "The key must not be empty. Non-ASCII characters are not supported and will be ignored."
    );

    let mut index = 0;

    text.chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            let shift = key_shifts[index % key_shifts.len()];
            index += 1;
            shift_char(c, if encrypt { shift } else { -shift })
        })
        .collect()
}

#[cfg(test)]
#[path = "../tests/vigenere.rs"]
mod tests;
