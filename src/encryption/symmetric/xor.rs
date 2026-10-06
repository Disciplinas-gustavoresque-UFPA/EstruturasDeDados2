/// XORs each input byte with the repeating key. Apply twice to recover the input.
///
/// # Panics
///
/// Panics if `key` is empty, even when `input` is empty.
pub fn xor_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(!key.is_empty(), "`key` must not have zero length.");

    input
        .iter()
        .zip(key.iter().cycle())
        .map(|(&byte, &key_byte)| byte ^ key_byte)
        .collect()
}

#[cfg(test)]
#[path = "../tests/xor.rs"]
mod tests;
