/// Applies RC4 and returns lowercase hexadecimal ciphertext.
/// To decrypt, decode the hexadecimal ciphertext to bytes before calling again.
/// Each call starts a new stream; key bytes after the first 256 are unused.
///
/// # Panics
///
/// Panics if `key` is empty, even when `input` is empty.
pub fn rc4(input: &[u8], key: &[u8]) -> String {
    assert!(!key.is_empty(), "`key` must not have zero length.");

    let mut state: [u8; 256] = std::array::from_fn(|i| i as u8);
    let mut j = 0;

    // Key scheduling.
    for i in 0..state.len() {
        j = (j + usize::from(state[i]) + usize::from(key[i % key.len()])) % state.len();
        state.swap(i, j);
    }

    let mut output = String::with_capacity(input.len() * 2);
    let hex = b"0123456789abcdef";
    let (mut i, mut j) = (0, 0);

    // Generate the stream and encode ciphertext without intermediate buffers.
    for &byte in input {
        i = (i + 1) % state.len();
        j = (j + usize::from(state[i])) % state.len();
        state.swap(i, j);
        let stream_byte = state[(usize::from(state[i]) + usize::from(state[j])) % state.len()];
        let encrypted = byte ^ stream_byte;
        output.push(char::from(hex[usize::from(encrypted >> 4)]));
        output.push(char::from(hex[usize::from(encrypted & 0x0f)]));
    }

    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rc4_encrypt() {
        let key = b"mysecretkey";
        let encrypted = super::rc4(b"helloworld", key);
        assert_eq!(encrypted, "82b614d4af3e51e2ac69");
    }

    #[test]
    fn test_rc4_rfc_6229_initial_stream() {
        // RFC 6229, section 2: the first 32 bytes for key 0x0102030405.
        assert_eq!(
            super::rc4(&[0; 32], &[1, 2, 3, 4, 5]),
            "b2396305f03dc027ccc3524a0a1118a86982944f18fc82d589c403a47a0d0919"
        );
    }

    #[test]
    fn test_rc4_rfc_6229_stream_boundaries() {
        // RFC 6229, section 2: published blocks at and beyond state wraparound.
        let stream = super::rc4(&[0; 4112], &[1, 2, 3, 4, 5]);
        for (offset, expected) in [
            (240, "28cb1132c96ce286421dcaadb8b69eae"),
            (256, "1cfcf62b03eddb641d77dfcf7f8d8c93"),
            (512, "6459844432a7da923cfb3eb4980661f6"),
            (1024, "30abbcc7c20b01609f23ee2d5f6bb7df"),
            (4096, "ff25b58995996707e51fbdf08b34d875"),
        ] {
            assert_eq!(&stream[offset * 2..(offset + 16) * 2], expected);
        }
        for len in [0, 1, 15, 16, 255, 256, 257, 511, 512, 513, 4096, 4112] {
            assert_eq!(
                super::rc4(&vec![0; len], &[1, 2, 3, 4, 5]),
                stream[..len * 2]
            );
        }
    }

    #[test]
    fn test_binary_input_and_key_lengths() {
        for (key, expected) in [
            (&[0][..], "de1909be"),
            (&[255][..], "6d24afdb"),
            (&[1, 2, 3, 4, 5][..], "b238e3fa"),
        ] {
            assert_eq!(super::rc4(&[0, 1, 128, 255], key), expected);
            assert_eq!(super::rc4(&[], key), "");
        }
    }

    #[test]
    fn test_key_bytes_after_state_size_are_unused() {
        let key: Vec<u8> = (0..=255).chain(0..44).collect();
        assert_eq!(super::rc4(&[0, 1, 128, 255], &key), "5e2f374d");
        assert_eq!(
            super::rc4(&[0, 1, 128, 255], &key),
            super::rc4(&[0, 1, 128, 255], &key[..256])
        );
    }

    #[test]
    fn test_binary_roundtrip() {
        let input: Vec<u8> = (0..=255).cycle().take(513).collect();
        let decode = |hex: String| -> Vec<u8> {
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
                .collect()
        };
        for key in [&[0][..], &[255][..], b"mysecretkey"] {
            let encrypted = decode(super::rc4(&input, key));
            assert_eq!(decode(super::rc4(&encrypted, key)), input);
        }
    }

    #[test]
    #[should_panic(expected = "`key` must not have zero length.")]
    fn test_empty_key() {
        super::rc4(b"text", &[]);
    }

    #[test]
    #[should_panic(expected = "`key` must not have zero length.")]
    fn test_empty_input_still_requires_key() {
        super::rc4(&[], &[]);
    }
}
