pub fn caesar_cipher(text: &str, shift: i32) -> String {
    return text.chars().map(|c| shift_char(c, shift)).collect();
}

fn shift_char(c: char, shift: i32) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }
    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
    let offset = ((c as i32 - base as i32 + shift).rem_euclid(26)) as u8;
    return (base + offset) as char;
}

#[cfg(test)]
mod tests {
    use super::caesar_cipher;

    #[test]
    fn test_basic_shift() {
        assert_eq!(caesar_cipher("abc", 1), "bcd");
        assert_eq!(caesar_cipher("xyz", 1), "yza");
    }

    #[test]
    fn test_case_preserved() {
        assert_eq!(caesar_cipher("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn test_negative_shift() {
        assert_eq!(caesar_cipher("bcd", -1), "abc");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Projeto e Análise de Algoritmos!";
        let shift = 7;
        let enc = caesar_cipher(original, shift);
        let dec = caesar_cipher(&enc, -shift);
        assert_eq!(dec, original);
    }

    #[test]
    fn test_non_alpha_unchanged() {
        assert_eq!(caesar_cipher("123!@#", 5), "123!@#");
    }
}
