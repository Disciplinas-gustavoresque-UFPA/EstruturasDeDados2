//! Twofish: blocos de 128 bits, chaves de 128/192/256 bits e 16 rodadas.
//!
//! Segue a especificação dos autores, com palavras little-endian, matrizes
//! MDS/RS e função h calculada diretamente. Primitiva didática sem padding,
//! modo, autenticação ou garantia de resistência a canais laterais.

#[path = "twofish_tables.rs"]
mod tables;
use tables::{MDS, Q, RS};

/// Comprimento inválido de uma chave Twofish, em bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwofishKeyError {
    pub length: usize,
}

impl std::fmt::Display for TwofishKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a chave Twofish deve conter 16, 24 ou 32 bytes; recebidos {}",
            self.length
        )
    }
}
impl std::error::Error for TwofishKeyError {}

/// Subchaves de whitening/rodada e palavras S derivadas por Reed–Solomon.
pub struct Twofish {
    subkeys: [u32; 40],
    s_words: [u32; 4],
    key_parts: usize,
}

impl Twofish {
    /// Expande uma chave de 128, 192 ou 256 bits.
    pub fn new(key: &[u8]) -> Result<Self, TwofishKeyError> {
        if ![16, 24, 32].contains(&key.len()) {
            return Err(TwofishKeyError { length: key.len() });
        }
        let key_parts = key.len() / 8;
        let mut even = [0u32; 4];
        let mut odd = [0u32; 4];
        let mut s_words = [0; 4];
        for (index, part) in key.chunks_exact(8).enumerate() {
            even[index] = u32::from_le_bytes(part[..4].try_into().unwrap());
            odd[index] = u32::from_le_bytes(part[4..].try_into().unwrap());
            let part: &[u8; 8] = part.try_into().unwrap();
            // A especificação armazena as palavras S em ordem inversa.
            s_words[key_parts - 1 - index] = u32::from_le_bytes(matrix_product(&RS, part, 0x14d));
        }
        let mut subkeys = [0; 40];
        for index in 0..20 {
            let a = h((2 * index as u32) * 0x01010101, &even[..key_parts]);
            let b = h((2 * index as u32 + 1) * 0x01010101, &odd[..key_parts]).rotate_left(8);
            subkeys[2 * index] = a.wrapping_add(b);
            subkeys[2 * index + 1] = a.wrapping_add(b.wrapping_mul(2)).rotate_left(9);
        }
        Ok(Self {
            subkeys,
            s_words,
            key_parts,
        })
    }

    /// Aplica whitening de entrada, 16 rodadas e whitening de saída.
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state: [u32; 4] = std::array::from_fn(|word| {
            u32::from_le_bytes(block[4 * word..4 * word + 4].try_into().unwrap())
                ^ self.subkeys[word]
        });
        for round in 0..16 {
            let (first, second) = self.round_function(state[0], state[1], round);
            state[2] = (state[2] ^ first).rotate_right(1);
            state[3] = state[3].rotate_left(1) ^ second;
            state.swap(0, 2);
            state.swap(1, 3);
        }
        let mut output = [0; 16];
        for (index, word) in output.chunks_exact_mut(4).enumerate() {
            word.copy_from_slice(&(state[(index + 2) % 4] ^ self.subkeys[index + 4]).to_le_bytes());
        }
        output
    }

    /// Inverte whitening, trocas de metades, rotações e rodadas.
    pub fn decrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = [0; 4];
        for (index, word) in block.chunks_exact(4).enumerate() {
            state[(index + 2) % 4] =
                u32::from_le_bytes(word.try_into().unwrap()) ^ self.subkeys[index + 4];
        }
        for round in (0..16).rev() {
            state.swap(0, 2);
            state.swap(1, 3);
            let (first, second) = self.round_function(state[0], state[1], round);
            state[2] = state[2].rotate_left(1) ^ first;
            state[3] = (state[3] ^ second).rotate_right(1);
        }
        let mut output = [0; 16];
        for (index, word) in output.chunks_exact_mut(4).enumerate() {
            word.copy_from_slice(&(state[index] ^ self.subkeys[index]).to_le_bytes());
        }
        output
    }

    fn round_function(&self, first: u32, second: u32, round: usize) -> (u32, u32) {
        let keys = &self.s_words[..self.key_parts];
        let a = h(first, keys);
        let b = h(second.rotate_left(8), keys);
        (
            a.wrapping_add(b).wrapping_add(self.subkeys[8 + 2 * round]),
            a.wrapping_add(b.wrapping_mul(2))
                .wrapping_add(self.subkeys[9 + 2 * round]),
        )
    }
}

/// Permutação de um byte por quatro substituições de nibbles (seção 4.3.5).
fn q(which: usize, byte: u8) -> u8 {
    let mix = |left: u8, right: u8| {
        (
            left ^ right,
            left ^ ((right >> 1) | ((right & 1) << 3)) ^ ((left << 3) & 0xf),
        )
    };
    let (left, right) = mix(byte >> 4, byte & 0xf);
    let (left, right) = mix(
        Q[which][0][usize::from(left)],
        Q[which][1][usize::from(right)],
    );
    (Q[which][3][usize::from(right)] << 4) | Q[which][2][usize::from(left)]
}

fn h(word: u32, keys: &[u32]) -> u32 {
    let mut bytes = word.to_le_bytes();
    if keys.len() == 4 {
        let key = keys[3].to_le_bytes();
        for (index, which) in [1, 0, 0, 1].into_iter().enumerate() {
            bytes[index] = q(which, bytes[index]) ^ key[index];
        }
    }
    if keys.len() >= 3 {
        let key = keys[2].to_le_bytes();
        for (index, which) in [1, 1, 0, 0].into_iter().enumerate() {
            bytes[index] = q(which, bytes[index]) ^ key[index];
        }
    }
    let first = keys[0].to_le_bytes();
    let second = keys[1].to_le_bytes();
    let substituted = [
        q(1, q(0, q(0, bytes[0]) ^ second[0]) ^ first[0]),
        q(0, q(0, q(1, bytes[1]) ^ second[1]) ^ first[1]),
        q(1, q(1, q(0, bytes[2]) ^ second[2]) ^ first[2]),
        q(0, q(1, q(1, bytes[3]) ^ second[3]) ^ first[3]),
    ];
    u32::from_le_bytes(matrix_product(&MDS, &substituted, 0x169))
}

fn matrix_product<const N: usize>(
    matrix: &[[u8; N]; 4],
    input: &[u8; N],
    polynomial: u16,
) -> [u8; 4] {
    std::array::from_fn(|row| {
        matrix[row]
            .iter()
            .zip(input)
            .fold(0, |sum, (&factor, &byte)| {
                sum ^ multiply(factor, byte, polynomial)
            })
    })
}

fn multiply(left: u8, mut right: u8, polynomial: u16) -> u8 {
    let mut left = u16::from(left);
    let mut product = 0;
    for _ in 0..8 {
        if right & 1 != 0 {
            product ^= left;
        }
        left <<= 1;
        if left & 0x100 != 0 {
            left ^= polynomial;
        }
        right >>= 1;
    }
    product as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vetores_dos_autores_com_chave_zerada() {
        for (length, text) in [
            (16, "9f589f5cf6122c32b6bfec2f2ae8c35a"),
            (24, "efa71f788965bd4453f860178fc19101"),
            (32, "57ff739d4dc92c1bd7fc01700cc8216f"),
        ] {
            let encrypted = std::array::from_fn(|index| {
                u8::from_str_radix(&text[2 * index..2 * index + 2], 16).unwrap()
            });
            let cipher = Twofish::new(&vec![0; length]).unwrap();
            assert_eq!(cipher.encrypt_block(&[0; 16]), encrypted);
            assert_eq!(cipher.decrypt_block(&encrypted), [0; 16]);
        }
    }

    #[test]
    fn valida_comprimentos_da_chave() {
        for length in 0..=128 {
            match Twofish::new(&vec![0; length]) {
                Ok(_) => assert!([16, 24, 32].contains(&length)),
                Err(error) => {
                    assert!(![16, 24, 32].contains(&length));
                    assert_eq!(error, TwofishKeyError { length });
                    assert!(error.to_string().contains(&length.to_string()));
                    let _: &dyn std::error::Error = &error;
                }
            }
        }
    }

    #[test]
    fn exemplo_com_chave_de_256_bits() {
        let cipher = Twofish::new(&[0; 32]).unwrap();
        let encrypted = cipher.encrypt_block(&[0; 16]);
        assert_eq!(cipher.decrypt_block(&encrypted), [0; 16]);
    }
}
