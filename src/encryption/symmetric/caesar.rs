use crate::encryption::symmetric::shift_char;

pub fn caesar_cipher(text: &str, shift: i32) -> String {
    return text
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            shift_char(c, shift)
        })
        .collect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_shift() {
        assert_eq!(super::caesar_cipher("abc", 1), "bcd");
        assert_eq!(super::caesar_cipher("xyz", 1), "yza");
    }

    #[test]
    fn test_case_preserved() {
        assert_eq!(super::caesar_cipher("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn test_negative_shift() {
        assert_eq!(super::caesar_cipher("bcd", -1), "abc");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Projeto e Análise de Algoritmos!";
        let shift = 7;
        let enc = super::caesar_cipher(original, shift);
        let dec = super::caesar_cipher(&enc, -shift);
        assert_eq!(dec, original);
    }

    #[test]
    fn test_non_alpha_unchanged() {
        assert_eq!(super::caesar_cipher("123!@#", 5), "123!@#");
    }

    #[test]
    fn test_empty_and_unicode_input() {
        assert_eq!(super::caesar_cipher("", 26), "");
        assert_eq!(super::caesar_cipher("é中🦀\0\nAz", 1), "é中🦀\0\nBa");
        assert_eq!(super::caesar_cipher("Az é!", 0), "Az é!");
    }

    #[test]
    fn test_full_alphabet_roundtrips() {
        let text = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        for shift in -104..=104 {
            let encrypted = super::caesar_cipher(text, shift);
            assert_eq!(super::caesar_cipher(&encrypted, -shift), text);
            assert_eq!(super::caesar_cipher(text, shift + 26), encrypted);
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_extreme_positive_shift() {
        super::caesar_cipher("zZ", i32::MAX);
    }
}
