//! Twofish's 128-bit block cipher, using a fresh key schedule for each call.
//! Words use little-endian byte order. No operation mode or padding is applied.
//! The implementation follows sections 4.1–4.3 of the designers' specification:
//! <https://www.schneier.com/wp-content/uploads/2016/02/paper-twofish-paper.pdf>.

/// Encrypts one 16-byte block with a 16-, 24-, or 32-byte key.
/// Returns raw ciphertext bytes and leaves both inputs unchanged. Each call
/// expands the key independently; use [`twofish_decrypt`] to recover the block.
///
/// # Panics
///
/// Panics unless `key` contains exactly 16, 24, or 32 bytes.
///
/// # Examples
///
/// ```
/// use crate::encryption::{twofish_decrypt, twofish_encrypt};
///
/// let block = *b"Twofish in Rust!";
/// let key = b"0123456789abcdef";
/// let ciphertext = twofish_encrypt(&block, key);
/// assert_eq!(twofish_decrypt(&ciphertext, key), block);
/// ```
pub fn twofish_encrypt(block: &[u8; 16], key: &[u8]) -> [u8; 16] {
    Twofish::new(key).encrypt_block(block)
}

/// Decrypts one 16-byte block with the original 16-, 24-, or 32-byte key.
/// Returns raw plaintext bytes and leaves both inputs unchanged. Each call
/// expands the key independently and reverses the whitening and round steps.
///
/// # Panics
///
/// Panics unless `key` contains exactly 16, 24, or 32 bytes.
///
/// # Examples
///
/// ```
/// use crate::encryption::twofish_decrypt;
///
/// let ciphertext = [
///     0x9f, 0x58, 0x9f, 0x5c, 0xf6, 0x12, 0x2c, 0x32,
///     0xb6, 0xbf, 0xec, 0x2f, 0x2a, 0xe8, 0xc3, 0x5a,
/// ];
/// assert_eq!(twofish_decrypt(&ciphertext, &[0; 16]), [0; 16]);
/// ```
pub fn twofish_decrypt(block: &[u8; 16], key: &[u8]) -> [u8; 16] {
    Twofish::new(key).decrypt_block(block)
}

struct Twofish {
    subkeys: [u32; 40],
    s_words: [u32; 4],
    word_count: usize,
}

impl Twofish {
    fn new(key: &[u8]) -> Self {
        assert!(
            matches!(key.len(), 16 | 24 | 32),
            "`key` must contain exactly 16, 24, or 32 bytes."
        );

        let word_count = key.len() / 8;
        let mut even = [0; 4];
        let mut odd = [0; 4];
        let mut state = Self {
            subkeys: [0; 40],
            s_words: [0; 4],
            word_count,
        };
        for (index, chunk) in key.as_chunks::<8>().0.iter().enumerate() {
            even[index] = u32::from_le_bytes(chunk[..4].try_into().unwrap());
            odd[index] = u32::from_le_bytes(chunk[4..].try_into().unwrap());
            // The S vector lists the Reed–Solomon results in reverse order.
            state.s_words[word_count - 1 - index] = rs_multiply(chunk);
        }

        for pair in 0..20 {
            let input = (pair as u32).wrapping_mul(0x0202_0202);
            let first = h(input, &even[..word_count]);
            let second = h(input.wrapping_add(0x0101_0101), &odd[..word_count]).rotate_left(8);
            let sum = first.wrapping_add(second);
            state.subkeys[2 * pair] = sum;
            state.subkeys[2 * pair + 1] = sum.wrapping_add(second).rotate_left(9);
        }
        state
    }

    fn round_outputs(&self, left: u32, right: u32, subkey: usize) -> [u32; 2] {
        let first = h(left, &self.s_words[..self.word_count]);
        let second = h(right.rotate_left(8), &self.s_words[..self.word_count]);
        // Pseudo-Hadamard transform, followed by the two round subkeys.
        let sum = first.wrapping_add(second);
        [
            sum.wrapping_add(self.subkeys[subkey]),
            sum.wrapping_add(second)
                .wrapping_add(self.subkeys[subkey + 1]),
        ]
    }

    fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut words = read_words(block);
        for (word, key) in words.iter_mut().zip(&self.subkeys[..4]) {
            *word ^= key;
        }

        // Each pair updates both halves without an explicit Feistel swap.
        for pair in 0..8 {
            let subkey = 8 + 4 * pair;
            let [first, second] = self.round_outputs(words[0], words[1], subkey);
            words[2] = (words[2] ^ first).rotate_right(1);
            words[3] = words[3].rotate_left(1) ^ second;
            let [first, second] = self.round_outputs(words[2], words[3], subkey + 2);
            words[0] = (words[0] ^ first).rotate_right(1);
            words[1] = words[1].rotate_left(1) ^ second;
        }

        // Undo the last swap and apply output whitening.
        write_words([
            words[2] ^ self.subkeys[4],
            words[3] ^ self.subkeys[5],
            words[0] ^ self.subkeys[6],
            words[1] ^ self.subkeys[7],
        ])
    }

    fn decrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let input = read_words(block);
        let mut words = [
            input[2] ^ self.subkeys[6],
            input[3] ^ self.subkeys[7],
            input[0] ^ self.subkeys[4],
            input[1] ^ self.subkeys[5],
        ];
        for pair in (0..8).rev() {
            let subkey = 8 + 4 * pair;
            let [first, second] = self.round_outputs(words[2], words[3], subkey + 2);
            words[0] = words[0].rotate_left(1) ^ first;
            words[1] = (words[1] ^ second).rotate_right(1);
            let [first, second] = self.round_outputs(words[0], words[1], subkey);
            words[2] = words[2].rotate_left(1) ^ first;
            words[3] = (words[3] ^ second).rotate_right(1);
        }
        for (word, key) in words.iter_mut().zip(&self.subkeys[..4]) {
            *word ^= key;
        }
        write_words(words)
    }
}

fn read_words(block: &[u8; 16]) -> [u32; 4] {
    std::array::from_fn(|index| {
        let offset = index * 4;
        u32::from_le_bytes(block[offset..offset + 4].try_into().unwrap())
    })
}

fn write_words(words: [u32; 4]) -> [u8; 16] {
    let mut output = [0; 16];
    for (chunk, word) in output.as_chunks_mut::<4>().0.iter_mut().zip(words) {
        *chunk = word.to_le_bytes();
    }
    output
}

// Table 1 of the specification: the four nibble permutations for q0 and q1.
const Q_NIBBLES: [[[u8; 16]; 4]; 2] = [
    [
        [8, 1, 7, 13, 6, 15, 3, 2, 0, 11, 5, 9, 14, 12, 10, 4],
        [14, 12, 11, 8, 1, 2, 3, 5, 15, 4, 10, 6, 7, 0, 9, 13],
        [11, 10, 5, 14, 6, 13, 9, 0, 12, 8, 15, 3, 2, 4, 7, 1],
        [13, 7, 15, 4, 1, 2, 6, 14, 9, 11, 3, 0, 8, 5, 12, 10],
    ],
    [
        [2, 8, 11, 13, 15, 7, 6, 14, 3, 1, 9, 4, 0, 10, 12, 5],
        [1, 14, 2, 11, 4, 12, 3, 7, 6, 13, 10, 5, 15, 9, 0, 8],
        [4, 12, 7, 5, 1, 6, 9, 10, 0, 14, 13, 8, 2, 11, 3, 15],
        [11, 9, 5, 1, 12, 3, 13, 14, 6, 4, 7, 15, 2, 0, 8, 10],
    ],
];

fn q(permutation: usize, value: u8) -> u8 {
    let tables = &Q_NIBBLES[permutation];
    let mut high = value >> 4;
    let mut low = value & 0x0f;
    for stage in [0, 2] {
        let mixed_high = high ^ low;
        let mixed_low = high ^ ((low >> 1) | ((low & 1) << 3)) ^ ((high << 3) & 8);
        high = tables[stage][usize::from(mixed_high)];
        low = tables[stage + 1][usize::from(mixed_low)];
    }
    (low << 4) | high
}

fn gf_multiply(mut value: u8, mut coefficient: u8, polynomial: u8) -> u8 {
    let mut result = 0;
    for _ in 0..8 {
        if coefficient & 1 != 0 {
            result ^= value;
        }
        let carry = value & 0x80;
        value <<= 1;
        if carry != 0 {
            value ^= polynomial;
        }
        coefficient >>= 1;
    }
    result
}

fn mds_column(value: u8, column: usize) -> u32 {
    // x^8 + x^6 + x^5 + x^3 + 1 (0x169), with the x^8 term implicit.
    let times_5b = gf_multiply(value, 0x5b, 0x69);
    let times_ef = gf_multiply(value, 0xef, 0x69);
    let bytes = match column {
        0 => [value, times_5b, times_ef, times_ef],
        1 => [times_ef, times_ef, times_5b, value],
        2 => [times_5b, times_ef, value, times_ef],
        3 => [times_5b, value, times_ef, times_5b],
        _ => unreachable!(),
    };
    u32::from_le_bytes(bytes)
}

const RS_MATRIX: [[u8; 8]; 4] = [
    [0x01, 0xa4, 0x55, 0x87, 0x5a, 0x58, 0xdb, 0x9e],
    [0xa4, 0x56, 0x82, 0xf3, 0x1e, 0xc6, 0x68, 0xe5],
    [0x02, 0xa1, 0xfc, 0xc1, 0x47, 0xae, 0x3d, 0x19],
    [0xa4, 0x55, 0x87, 0x5a, 0x58, 0xdb, 0x9e, 0x03],
];

fn rs_multiply(bytes: &[u8]) -> u32 {
    // Reed–Solomon uses 0x14d, a different field from the MDS transform.
    let result = RS_MATRIX.map(|row| {
        row.iter().zip(bytes).fold(0, |sum, (&coefficient, &byte)| {
            sum ^ gf_multiply(byte, coefficient, 0x4d)
        })
    });
    u32::from_le_bytes(result)
}

// For each byte position, list q selectors from outermost to innermost.
const Q_ORDER: [[usize; 5]; 4] = [
    [1, 0, 0, 1, 1],
    [0, 0, 1, 1, 0],
    [1, 1, 0, 0, 0],
    [0, 1, 1, 0, 1],
];

fn h(input: u32, key_words: &[u32]) -> u32 {
    let mut output = 0;
    for (column, mut byte) in input.to_le_bytes().into_iter().enumerate() {
        for (index, word) in key_words.iter().enumerate().rev() {
            byte = q(Q_ORDER[column][index + 1], byte) ^ word.to_le_bytes()[column];
        }
        output ^= mds_column(q(Q_ORDER[column][0], byte), column);
    }
    output
}

#[cfg(test)]
// Criterion imports cfg(test) modules but supplies its own main, so test-only
// fixtures and helpers are not referenced by the benchmark executable.
#[allow(dead_code)]
#[path = "../tests/twofish/mod.rs"]
mod tests;
