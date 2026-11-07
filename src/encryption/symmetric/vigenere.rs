use crate::encryption::symmetric::shift_char;

pub fn vigenere_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let key_bytes: Vec<i32> = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase() as i32 - b'A' as i32)
        .collect();

    if key_bytes.is_empty() {
        return text.to_string();
    }

    let mut index = 0;

    text.chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            let shift = key_bytes[index % key_bytes.len()];
            index += 1;
            return shift_char(c, if encrypt { shift } else { -shift });
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::vigenere_cipher;

    #[test]
    fn test_vigenere_encrypt_classic_example() {
        let input = "ATTACKATDAWN";
        let key = "LEMON";

        let result = vigenere_cipher(input, key, true);

        assert_eq!(result, "LXFOPVEFRNHR");
    }

    #[test]
    fn test_vigenere_encrypt_repeating_key() {
        let input = "HELLOWORLD";
        let key = "KEY";

        let result = vigenere_cipher(input, key, true);

        assert_eq!(result, "RIJVSUYVJN");
    }

    #[test]
    fn test_vigenere_encrypt_single_character() {
        let input = "A";
        let key = "B";

        let result = vigenere_cipher(input, key, true);

        assert_eq!(result, "B");
    }

    #[test]
    fn test_vigenere_encrypt_key_longer_than_input() {
        let input = "ABC";
        let key = "KEYLONG";

        let result = vigenere_cipher(input, key, true);

        assert_eq!(result, "KFA");
    }

    #[test]
    fn test_vigenere_encrypt_empty_input() {
        let input = "";
        let key = "KEY";

        let result = vigenere_cipher(input, key, true);

        assert!(result.is_empty());
    }

    #[test]
    fn test_vigenere_encrypt_preserves_non_letters() {
        let input = "HELLO WORLD!";
        let key = "KEY";

        let result = vigenere_cipher(input, key, true);

        assert_eq!(result, "RIJVS UYVJN!");
    }

    #[test]
    fn test_vigenere_encrypt_is_reversible() {
        let input = "THEQUICKBROWNFOX";
        let key = "SECRET";

        let encrypted = vigenere_cipher(input, key, true);
        let decrypted = vigenere_cipher(&encrypted, key, false);

        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_vigenere_ignores_non_ascii_key_letters() {
        for key in ["ß", "ﬃ", "ſ", "é🙂"] {
            for encrypt in [true, false] {
                assert_eq!(vigenere_cipher("Aa, é🙂!", key, encrypt), "Aa, é🙂!");
            }
        }
    }

    #[test]
    fn test_vigenere_filters_mixed_key_before_normalizing() {
        let key = "bßcﬃdſ";
        assert_eq!(vigenere_cipher("AAAAAA", key, true), "BCDBCD");
        assert_eq!(vigenere_cipher("BCDBCD", key, false), "AAAAAA");
        assert_eq!(vigenere_cipher("AAAA,AA", key, true), "BCDB,CD");
    }
}
