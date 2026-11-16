use crate::encryption::symmetric::shift_char;

/// Criptografa quando `encrypt` é verdadeiro e descriptografa caso contrário.
/// Preserva a caixa das letras ASCII e mantém os demais caracteres do texto.
/// Apenas letras ASCII do texto avançam a chave cíclica. A chave é filtrada
/// antes da conversão para maiúsculas ASCII: caracteres como `ß` são ignorados,
/// sem expansão Unicode. Se não restarem letras ASCII na chave, devolve o texto
/// original, inclusive quando ele está vazio.
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
            shift_char(c, if encrypt { shift } else { -shift })
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
    fn test_unicode_key_letters_are_not_expanded() {
        assert_eq!(vigenere_cipher("ABC", "ß", true), "ABC");
        assert_eq!(vigenere_cipher("ABC", "ı", true), "ABC");
        assert_eq!(vigenere_cipher("ABC", "éA中", true), "ABC");
        assert_eq!(vigenere_cipher("STU", "ß", false), "STU");
    }

    #[test]
    fn test_invalid_keys_preserve_input_including_empty_text() {
        for key in ["", "123!?", "é中🦀"] {
            for text in ["", "ABC", "Aa, é中🦀!\0"] {
                for encrypt in [true, false] {
                    assert_eq!(vigenere_cipher(text, key, encrypt), text);
                }
            }
        }
    }
}
