use crate::encryption::symmetric::shift_char;

pub fn vigenere_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let key_bytes: Vec<i32> = key
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c as i32 - b'A' as i32)
        .collect();

    if key_bytes.is_empty() {
        panic!(
            "The key must not be empty. Non-ASCII characters are not supported and will be ignored."
        );
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
    #[test]
    fn test_vigenere_encrypt_classic_example() {
        let input = "ATTACKATDAWN";
        let key = "LEMON";

        let result = super::vigenere_cipher(input, key, true);

        assert_eq!(result, "LXFOPVEFRNHR");
    }

    #[test]
    fn test_vigenere_encrypt_repeating_key() {
        let input = "HELLOWORLD";
        let key = "KEY";

        let result = super::vigenere_cipher(input, key, true);

        assert_eq!(result, "RIJVSUYVJN");
    }

    #[test]
    fn test_vigenere_encrypt_single_character() {
        let input = "A";
        let key = "B";

        let result = super::vigenere_cipher(input, key, true);

        assert_eq!(result, "B");
    }

    #[test]
    fn test_vigenere_encrypt_key_longer_than_input() {
        let input = "ABC";
        let key = "KEYLONG";

        let result = super::vigenere_cipher(input, key, true);

        assert_eq!(result, "KFA");
    }

    #[test]
    fn test_vigenere_encrypt_empty_input() {
        let input = "";
        let key = "KEY";

        let result = super::vigenere_cipher(input, key, true);

        assert!(result.is_empty());
    }

    #[test]
    fn test_vigenere_encrypt_preserves_non_letters() {
        let input = "HELLO WORLD!";
        let key = "KEY";

        let result = super::vigenere_cipher(input, key, true);

        // Expected behavior depends on your implementation.
        // This assumes spaces and punctuation are left unchanged.
        assert_eq!(result, "RIJVS UYVJN!");
    }

    #[test]
    fn test_vigenere_encrypt_is_reversible() {
        let input = "THEQUICKBROWNFOX";
        let key = "SECRET";

        let encrypted = super::vigenere_cipher(input, key, true);
        let decrypted = super::vigenere_cipher(&encrypted, key, false);

        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_case_and_key_filtering() {
        assert_eq!(
            super::vigenere_cipher("Attack at dawn!", "lE-m0oN", true),
            "Lxfopv ef rnhr!"
        );
        assert_eq!(
            super::vigenere_cipher("Lxfopv ef rnhr!", "lE-m0oN", false),
            "Attack at dawn!"
        );
    }

    #[test]
    fn test_non_letters_do_not_advance_key() {
        assert_eq!(
            super::vigenere_cipher("Aé!a中\0Z🦀z", "BC", true),
            "Bé!c中\0A🦀b"
        );
        assert_eq!(
            super::vigenere_cipher("Bé!c中\0A🦀b", "BC", false),
            "Aé!a中\0Z🦀z"
        );
        assert_eq!(super::vigenere_cipher("é中🦀!", "BC", true), "é中🦀!");
    }

    #[test]
    fn test_unicode_key_uppercase_expansion() {
        assert_eq!(super::vigenere_cipher("ABC", "ß", true), "STU");
        assert_eq!(super::vigenere_cipher("ABC", "ı", true), "IJK");
        assert_eq!(super::vigenere_cipher("ABC", "éA中", true), "ABC");
        assert_eq!(super::vigenere_cipher("STU", "ß", false), "ABC");
    }

    #[test]
    fn test_invalid_keys_panic_even_for_empty_input() {
        for key in ["", "123!?", "é中🦀"] {
            for text in ["", "ABC"] {
                for encrypt in [true, false] {
                    let panic =
                        std::panic::catch_unwind(|| super::vigenere_cipher(text, key, encrypt))
                            .expect_err("a key without ASCII letters after uppercasing must panic");
                    assert_eq!(
                        panic.downcast_ref::<&str>().copied(),
                        Some(
                            "The key must not be empty. Non-ASCII characters are not supported and will be ignored."
                        )
                    );
                }
            }
        }
    }
}
