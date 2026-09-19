pub fn rc4(input: &[u8], key: &[u8]) -> String {
    let mut S: Vec<u8> = (0..=255).collect();

    let mut j = 0;

    for i in 0..256 {
        j = (j + S[i] as usize + key[i % key.len()] as usize) % 256;
        S.swap(i, j);
    }

    let stream: Vec<u8> = pseudo_random_generator(&mut S, input.len());

    return input
        .iter()
        .enumerate()
        .map(|(i, c)| c ^ stream[i])
        .map(|c| format!("{:02x}", c))
        .collect();
}

fn pseudo_random_generator(S: &mut Vec<u8>, input_len: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(input_len).repeat(0);

    let mut i = 0;
    let mut j = 0;

    for _ in 0..input_len {
        i = (i + 1) % 256;
        j = (j + S[i] as usize) % 256;
        S.swap(i, j);
        result.push(S[(S[i] as usize + S[j] as usize) % 256]);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::rc4;

    #[test]
    fn test_rc4_encrypt() {
        let key = b"mysecretkey";
        let encrypted = rc4(b"helloworld", key);
        assert_eq!(encrypted, "3e14233d258b2c8b1a155a");
    }
}
