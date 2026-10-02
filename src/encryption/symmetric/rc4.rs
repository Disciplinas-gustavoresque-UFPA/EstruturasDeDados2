pub fn rc4(input: &[u8], key: &[u8]) -> String {
    let mut s: Vec<u8> = (0..=255).collect();

    let mut j = 0;

    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }

    let stream: Vec<u8> = pseudo_random_generator(&mut s, input.len());

    input
        .iter()
        .enumerate()
        .map(|(i, c)| c ^ stream[i])
        .map(|c| format!("{:02x}", c))
        .collect()
}

fn pseudo_random_generator(s: &mut Vec<u8>, input_len: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(input_len).repeat(0);

    let mut i = 0;
    let mut j = 0;

    for _ in 0..input_len {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        result.push(s[(s[i] as usize + s[j] as usize) % 256]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::rc4;

    #[test]
    fn test_rc4_encrypt() {
        let key = b"mysecretkey";
        let encrypted = rc4(b"helloworld", key);
        assert_eq!(encrypted, "82b614d4af3e51e2ac69");
    }

    #[test]
    fn test_rc4_rfc_6229_initial_stream() {
        // RFC 6229, section 2: the first 32 bytes for key 0x0102030405.
        assert_eq!(
            rc4(&[0; 32], &[1, 2, 3, 4, 5]),
            "b2396305f03dc027ccc3524a0a1118a86982944f18fc82d589c403a47a0d0919"
        );
    }
}
