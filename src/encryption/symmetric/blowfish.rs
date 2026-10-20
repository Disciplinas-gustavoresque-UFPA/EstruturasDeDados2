//! Blowfish de 16 rodadas, com bloco de 64 bits e chave de 32 a 448 bits.
//!
//! Implementação didática da primitiva: sem padding, modo ou autenticação.
//! As S-boxes não oferecem acesso em tempo constante. O bloco de 64 bits limita
//! o volume de dados que pode ser protegido por uma mesma chave.

#[path = "blowfish_constants.rs"]
mod constants;
use constants::{INITIAL_P, INITIAL_S};

/// Comprimento inválido de uma chave Blowfish, em bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlowfishKeyError {
    pub length: usize,
}

impl std::fmt::Display for BlowfishKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a chave Blowfish deve conter de 4 a 56 bytes; recebidos {}",
            self.length
        )
    }
}
impl std::error::Error for BlowfishKeyError {}

/// Vetor P e S-boxes expandidos para uma chave, reutilizáveis entre blocos.
pub struct Blowfish {
    p: [u32; 18],
    s: [[u32; 256]; 4],
}

impl Blowfish {
    /// Expande entre quatro e 56 bytes, sem completar ou truncar a chave.
    pub fn new(key: &[u8]) -> Result<Self, BlowfishKeyError> {
        if !(4..=56).contains(&key.len()) {
            return Err(BlowfishKeyError { length: key.len() });
        }
        let mut cipher = Self {
            p: INITIAL_P,
            s: INITIAL_S,
        };
        let mut offset = 0;
        for subkey in &mut cipher.p {
            let mut word = 0u32;
            for _ in 0..4 {
                word = (word << 8) | u32::from(key[offset]);
                offset = (offset + 1) % key.len();
            }
            *subkey ^= word;
        }
        let (mut left, mut right) = (0, 0);
        for index in (0..18).step_by(2) {
            (left, right) = cipher.transform_words(left, right, false);
            cipher.p[index] = left;
            cipher.p[index + 1] = right;
        }
        for box_index in 0..4 {
            for index in (0..256).step_by(2) {
                (left, right) = cipher.transform_words(left, right, false);
                cipher.s[box_index][index] = left;
                cipher.s[box_index][index + 1] = right;
            }
        }
        Ok(cipher)
    }

    /// Criptografa oito bytes, com palavras de 32 bits em big-endian.
    pub fn encrypt_block(&self, block: &[u8; 8]) -> [u8; 8] {
        self.transform(block, false)
    }

    /// Descriptografa oito bytes, invertendo a ordem das subchaves P.
    pub fn decrypt_block(&self, block: &[u8; 8]) -> [u8; 8] {
        self.transform(block, true)
    }

    fn transform(&self, block: &[u8; 8], decrypt: bool) -> [u8; 8] {
        let left = u32::from_be_bytes(block[..4].try_into().unwrap());
        let right = u32::from_be_bytes(block[4..].try_into().unwrap());
        let (left, right) = self.transform_words(left, right, decrypt);
        let mut result = [0; 8];
        result[..4].copy_from_slice(&left.to_be_bytes());
        result[4..].copy_from_slice(&right.to_be_bytes());
        result
    }

    fn transform_words(&self, mut left: u32, mut right: u32, decrypt: bool) -> (u32, u32) {
        for round in 0..16 {
            left ^= self.p[if decrypt { 17 - round } else { round }];
            right ^= self.round_function(left);
            std::mem::swap(&mut left, &mut right);
        }
        std::mem::swap(&mut left, &mut right);
        right ^= self.p[if decrypt { 1 } else { 16 }];
        left ^= self.p[if decrypt { 0 } else { 17 }];
        (left, right)
    }

    fn round_function(&self, word: u32) -> u32 {
        let [a, b, c, d] = word.to_be_bytes().map(usize::from);
        (self.s[0][a].wrapping_add(self.s[1][b]) ^ self.s[2][c]).wrapping_add(self.s[3][d])
    }
}

#[cfg(test)]
#[path = "blowfish_tests.rs"]
mod validation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vetor_publicado_com_bloco_e_chave_zerados() {
        // https://www.schneier.com/wp-content/uploads/2015/12/vectors-2.txt
        let cipher = Blowfish::new(&[0; 8]).unwrap();
        let encrypted = 0x4ef997456198dd78u64.to_be_bytes();
        assert_eq!(cipher.encrypt_block(&[0; 8]), encrypted);
        assert_eq!(cipher.decrypt_block(&encrypted), [0; 8]);
    }

    #[test]
    fn exemplo_reutiliza_a_chave_expandida() {
        let cipher = Blowfish::new(b"chave de exemplo").unwrap();
        let plain = *b"mensagem";
        assert_eq!(cipher.decrypt_block(&cipher.encrypt_block(&plain)), plain);
    }

    #[test]
    fn valida_todos_os_comprimentos_da_chave() {
        for length in 0..=128 {
            match Blowfish::new(&vec![0; length]) {
                Ok(_) => assert!((4..=56).contains(&length)),
                Err(error) => {
                    assert!(!(4..=56).contains(&length));
                    assert_eq!(error, BlowfishKeyError { length });
                    assert!(error.to_string().contains(&length.to_string()));
                    let _: &dyn std::error::Error = &error;
                }
            }
        }
    }
}
