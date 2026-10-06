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
#[path = "../tests/rc4.rs"]
mod tests;
