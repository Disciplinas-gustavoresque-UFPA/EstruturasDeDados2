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

        // Expected behavior depends on your implementation.
        // This assumes spaces and punctuation are left unchanged.
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
}
