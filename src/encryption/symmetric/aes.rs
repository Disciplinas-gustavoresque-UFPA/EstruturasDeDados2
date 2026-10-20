//! AES-128/192/256 segundo a FIPS 197, com estado preenchido por colunas.
//!
//! Primitiva de bloco didática: sem modo, padding, autenticação ou garantia
//! contra canais laterais. As tabelas de substituição são calculadas a partir
//! da definição algébrica em tempo de compilação.

const SBOX: [u8; 256] = make_sbox();
const INVERSE_SBOX: [u8; 256] = invert_sbox();

/// Comprimento inválido de uma chave AES, em bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AesKeyError {
    pub length: usize,
}

impl std::fmt::Display for AesKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a chave AES deve conter 16, 24 ou 32 bytes; recebidos {}",
            self.length
        )
    }
}
impl std::error::Error for AesKeyError {}

/// Chaves expandidas para 10, 12 ou 14 rodadas, conforme o tamanho da chave.
pub struct Aes {
    round_keys: [[u8; 16]; 15],
    rounds: usize,
}

impl Aes {
    /// Seleciona AES-128, AES-192 ou AES-256 e expande a chave correspondente.
    pub fn new(key: &[u8]) -> Result<Self, AesKeyError> {
        if ![16, 24, 32].contains(&key.len()) {
            return Err(AesKeyError { length: key.len() });
        }
        let nk = key.len() / 4;
        let rounds = nk + 6;
        let mut words = [[0; 4]; 60];
        for (word, chunk) in words.iter_mut().zip(key.as_chunks::<4>().0) {
            word.copy_from_slice(chunk);
        }
        let mut rcon = 1;
        for index in nk..4 * (rounds + 1) {
            let mut temp = words[index - 1];
            if index % nk == 0 {
                temp.rotate_left(1);
                temp = temp.map(|byte| SBOX[usize::from(byte)]);
                temp[0] ^= rcon;
                rcon = multiply(rcon, 2);
            } else if nk == 8 && index % nk == 4 {
                temp = temp.map(|byte| SBOX[usize::from(byte)]);
            }
            words[index] = std::array::from_fn(|byte| words[index - nk][byte] ^ temp[byte]);
        }
        let mut round_keys = [[0; 16]; 15];
        for (round, round_key) in round_keys.iter_mut().take(rounds + 1).enumerate() {
            for (column, output) in round_key.as_chunks_mut::<4>().0.iter_mut().enumerate() {
                output.copy_from_slice(&words[4 * round + column]);
            }
        }
        Ok(Self { round_keys, rounds })
    }

    /// Criptografa um bloco de 128 bits; a rodada final omite MixColumns.
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = *block;
        add_round_key(&mut state, &self.round_keys[0]);
        for round in 1..self.rounds {
            state = state.map(|byte| SBOX[usize::from(byte)]);
            shift_rows(&mut state, false);
            mix_columns(&mut state, false);
            add_round_key(&mut state, &self.round_keys[round]);
        }
        state = state.map(|byte| SBOX[usize::from(byte)]);
        shift_rows(&mut state, false);
        add_round_key(&mut state, &self.round_keys[self.rounds]);
        state
    }

    /// Desfaz as transformações, usando as chaves de rodada em ordem inversa.
    pub fn decrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = *block;
        add_round_key(&mut state, &self.round_keys[self.rounds]);
        for round in (1..self.rounds).rev() {
            shift_rows(&mut state, true);
            state = state.map(|byte| INVERSE_SBOX[usize::from(byte)]);
            add_round_key(&mut state, &self.round_keys[round]);
            mix_columns(&mut state, true);
        }
        shift_rows(&mut state, true);
        state = state.map(|byte| INVERSE_SBOX[usize::from(byte)]);
        add_round_key(&mut state, &self.round_keys[0]);
        state
    }
}

fn add_round_key(state: &mut [u8; 16], key: &[u8; 16]) {
    for (byte, key_byte) in state.iter_mut().zip(key) {
        *byte ^= key_byte;
    }
}

fn shift_rows(state: &mut [u8; 16], inverse: bool) {
    let previous = *state;
    for (index, byte) in state.iter_mut().enumerate() {
        let row = index % 4;
        let column = index / 4;
        let offset = if inverse { 4 - row } else { row };
        *byte = previous[4 * ((column + offset) % 4) + row];
    }
}

fn mix_columns(state: &mut [u8; 16], inverse: bool) {
    let matrix = if inverse {
        [
            [14, 11, 13, 9],
            [9, 14, 11, 13],
            [13, 9, 14, 11],
            [11, 13, 9, 14],
        ]
    } else {
        [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]]
    };
    for column in state.as_chunks_mut::<4>().0.iter_mut() {
        let previous = *column;
        for (output, row) in column.iter_mut().zip(matrix) {
            *output = row
                .into_iter()
                .zip(previous)
                .fold(0, |sum, (factor, value)| sum ^ multiply(factor, value));
        }
    }
}

/// Produto em GF(2^8), reduzido pelo polinômio x^8 + x^4 + x^3 + x + 1.
const fn multiply(mut left: u8, mut right: u8) -> u8 {
    let mut product = 0;
    let mut bit = 0;
    while bit < 8 {
        if right & 1 != 0 {
            product ^= left;
        }
        left = (left << 1) ^ if left & 0x80 != 0 { 0x1b } else { 0 };
        right >>= 1;
        bit += 1;
    }
    product
}

const fn make_sbox() -> [u8; 256] {
    let mut sbox = [0; 256];
    let mut value = 0;
    while value < 256 {
        // a^254 = a^-1 para a != 0; para zero, o resultado definido é zero.
        let mut base = value as u8;
        let mut power = 254;
        let mut inverse = 1;
        while power != 0 {
            if power & 1 != 0 {
                inverse = multiply(inverse, base);
            }
            base = multiply(base, base);
            power >>= 1;
        }
        sbox[value] = inverse
            ^ inverse.rotate_left(1)
            ^ inverse.rotate_left(2)
            ^ inverse.rotate_left(3)
            ^ inverse.rotate_left(4)
            ^ 0x63;
        value += 1;
    }
    sbox
}

const fn invert_sbox() -> [u8; 256] {
    let mut inverse = [0; 256];
    let mut index = 0;
    while index < 256 {
        inverse[SBOX[index] as usize] = index as u8;
        index += 1;
    }
    inverse
}

#[cfg(test)]
#[path = "aes_tests.rs"]
mod validation;

#[cfg(test)]
mod tests {
    use super::*;

    fn hex<const N: usize>(text: &str) -> [u8; N] {
        assert_eq!(text.len(), 2 * N);
        std::array::from_fn(|index| {
            u8::from_str_radix(&text[2 * index..2 * index + 2], 16).unwrap()
        })
    }

    #[test]
    fn vetores_fips197_para_as_tres_variantes() {
        // Exemplos completos de AES-128/192/256 da FIPS 197.
        let plain = hex("00112233445566778899aabbccddeeff");
        for (length, encrypted) in [
            (16, "69c4e0d86a7b0430d8cdb78070b4c55a"),
            (24, "dda97ca4864cdfe06eaf70a0ec0d7191"),
            (32, "8ea2b7ca516745bfeafc49904b496089"),
        ] {
            let key: Vec<_> = (0..length).map(|byte| byte as u8).collect();
            let cipher = Aes::new(&key).unwrap();
            let encrypted = hex(encrypted);
            assert_eq!(cipher.encrypt_block(&plain), encrypted);
            assert_eq!(cipher.decrypt_block(&encrypted), plain);
        }
    }

    #[test]
    fn valida_comprimentos_da_chave() {
        for length in 0..=128 {
            match Aes::new(&vec![0; length]) {
                Ok(_) => assert!([16, 24, 32].contains(&length)),
                Err(error) => {
                    assert!(![16, 24, 32].contains(&length));
                    assert_eq!(error, AesKeyError { length });
                    assert!(error.to_string().contains(&length.to_string()));
                    let _: &dyn std::error::Error = &error;
                }
            }
        }
    }

    #[test]
    fn algebra_e_substituicoes_conferem_com_a_fips197() {
        assert_eq!(multiply(0x57, 0x13), 0xfe);
        assert_eq!(SBOX[0], 0x63);
        assert_eq!(SBOX[0x53], 0xed);
        for byte in 0..256 {
            assert_eq!(usize::from(INVERSE_SBOX[usize::from(SBOX[byte])]), byte);
        }
    }

    #[test]
    fn exemplo_com_chave_zerada() {
        let cipher = Aes::new(&[0; 16]).unwrap();
        let encrypted = cipher.encrypt_block(&[0; 16]);
        assert_eq!(cipher.decrypt_block(&encrypted), [0; 16]);
    }
}
