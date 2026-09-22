pub fn xor_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        // erro não recuperável (chave nunca deve ser vazia)
        panic!("`key` must not have zero length.");
    }

    input
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::xor_encrypt;

    #[test]
    fn test_xor_encrypt_basic() {
        let input = b"hello";
        let key = b"key";

        let result = xor_encrypt(input, key);

        assert_eq!(
            result,
            vec![
                b'h' ^ b'k',
                b'e' ^ b'e',
                b'l' ^ b'y',
                b'l' ^ b'k',
                b'o' ^ b'e',
            ]
        );
    }

    #[test]
    fn test_xor_encrypt_repeating_key() {
        let input = b"abcdef";
        let key = b"ab";

        let result = xor_encrypt(input, key);

        assert_eq!(
            result,
            vec![
                b'a' ^ b'a',
                b'b' ^ b'b',
                b'c' ^ b'a',
                b'd' ^ b'b',
                b'e' ^ b'a',
                b'f' ^ b'b',
            ]
        );
    }

    #[test]
    fn test_xor_encrypt_empty_input() {
        let input = b"";
        let key = b"key";

        let result = xor_encrypt(input, key);

        assert!(result.is_empty());
    }

    #[test]
    fn test_xor_encrypt_single_byte() {
        let input = &[0b1010_1010];
        let key = &[0b1111_0000];

        let result = xor_encrypt(input, key);

        assert_eq!(result, vec![0b0101_1010]);
    }

    #[test]
    fn test_xor_encrypt_is_reversible() {
        let input = b"Hello, world!";
        let key = b"secret";

        let encrypted = xor_encrypt(input, key);
        let decrypted = xor_encrypt(&encrypted, key);

        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_xor_encrypt_key_longer_than_input() {
        let input = b"abc";
        let key = b"longerkey";

        let result = xor_encrypt(input, key);

        assert_eq!(result, vec![b'a' ^ b'l', b'b' ^ b'o', b'c' ^ b'n',]);
    }

    #[test]
    fn test_xor_encrypt_zero_key_preserves_input() {
        let input = b"hello";
        let key = &[0u8];

        let encrypted = xor_encrypt(input, key);

        assert_eq!(encrypted, input);
    }

    #[test]
    #[should_panic(expected = "`key` must not have zero length.")]
    fn test_xor_encrypt_empty_key() {
        let input = b"hello";
        let key = b"";

        xor_encrypt(input, key);
    }
}
