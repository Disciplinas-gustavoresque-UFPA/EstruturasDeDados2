//! Primitiva DES da FIPS 46-3, para estudo de cifras de bloco.
//!
//! Opera um bloco de oito bytes, sem padding, modo de operação ou autenticação.
//! DES está obsoleto para proteção de dados. As consultas às tabelas não oferecem
//! garantia de tempo constante. Os oito bits de paridade da chave são ignorados;
//! chaves fracas são aceitas para permitir estudar o algoritmo completo.

#[path = "des_tables.rs"]
mod tables;

use tables::{EXPANSION, FINAL, INITIAL, PC1, PC2, PERMUTATION, SBOXES, SHIFTS};

/// Comprimento inválido de uma chave DES, em bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesKeyError {
    pub length: usize,
}

impl std::fmt::Display for DesKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a chave DES deve conter 8 bytes; recebidos {}",
            self.length
        )
    }
}

impl std::error::Error for DesKeyError {}

/// Chave DES expandida em 16 subchaves de 48 bits.
pub struct Des {
    subkeys: [u64; 16],
}

impl Des {
    /// Expande uma chave de oito bytes (56 bits efetivos e oito de paridade).
    pub fn new(key: &[u8]) -> Result<Self, DesKeyError> {
        let key: [u8; 8] = key
            .try_into()
            .map_err(|_| DesKeyError { length: key.len() })?;
        let selected = permute(u64::from_be_bytes(key), 64, &PC1);
        let mut left = (selected >> 28) as u32;
        let mut right = (selected & 0x0fff_ffff) as u32;
        let mut subkeys = [0; 16];
        for (subkey, shift) in subkeys.iter_mut().zip(SHIFTS) {
            left = rotate28(left, shift);
            right = rotate28(right, shift);
            *subkey = permute((u64::from(left) << 28) | u64::from(right), 56, &PC2);
        }
        Ok(Self { subkeys })
    }

    /// Criptografa um bloco, interpretando os bits na ordem da FIPS 46-3.
    pub fn encrypt_block(&self, block: &[u8; 8]) -> [u8; 8] {
        self.transform(block, false)
    }

    /// Descriptografa um bloco usando as subchaves em ordem inversa.
    pub fn decrypt_block(&self, block: &[u8; 8]) -> [u8; 8] {
        self.transform(block, true)
    }

    fn transform(&self, block: &[u8; 8], decrypt: bool) -> [u8; 8] {
        let initial = permute(u64::from_be_bytes(*block), 64, &INITIAL);
        let mut left = (initial >> 32) as u32;
        let mut right = initial as u32;
        for round in 0..16 {
            let subkey = self.subkeys[if decrypt { 15 - round } else { round }];
            (left, right) = (right, left ^ feistel(right, subkey));
        }
        let swapped = (u64::from(right) << 32) | u64::from(left);
        permute(swapped, 64, &FINAL).to_be_bytes()
    }
}

fn rotate28(value: u32, shift: u32) -> u32 {
    ((value << shift) | (value >> (28 - shift))) & 0x0fff_ffff
}

/// As tabelas numeram os bits a partir de 1, do mais para o menos significativo.
fn permute(input: u64, width: u32, positions: &[u8]) -> u64 {
    positions.iter().fold(0, |output, &position| {
        (output << 1) | ((input >> (width - u32::from(position))) & 1)
    })
}

fn feistel(right: u32, subkey: u64) -> u32 {
    let expanded = permute(u64::from(right), 32, &EXPANSION) ^ subkey;
    let mut substituted = 0u32;
    for (index, sbox) in SBOXES.iter().enumerate() {
        let six = ((expanded >> (42 - 6 * index)) & 0x3f) as usize;
        let row = ((six & 0x20) >> 4) | (six & 1);
        let column = (six >> 1) & 0xf;
        substituted = (substituted << 4) | u32::from(sbox[row * 16 + column]);
    }
    permute(u64::from(substituted), 32, &PERMUTATION) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemplo_conhecido_em_ambas_as_direcoes() {
        let cipher = Des::new(&0x133457799bbcdff1u64.to_be_bytes()).unwrap();
        let plain = 0x0123456789abcdefu64.to_be_bytes();
        let encrypted = 0x85e813540f0ab405u64.to_be_bytes();
        assert_eq!(cipher.encrypt_block(&plain), encrypted);
        assert_eq!(cipher.decrypt_block(&encrypted), plain);
    }

    #[test]
    fn rejeita_chaves_com_comprimento_invalido() {
        for length in 0..=64 {
            match Des::new(&vec![0; length]) {
                Ok(_) => assert_eq!(length, 8),
                Err(error) => {
                    assert_ne!(length, 8);
                    assert_eq!(error, DesKeyError { length });
                    assert!(error.to_string().contains(&length.to_string()));
                    let _: &dyn std::error::Error = &error;
                }
            }
        }
    }

    #[test]
    fn bits_de_paridade_nao_alteram_a_cifra() {
        let key = 0x133457799bbcdff1u64.to_be_bytes();
        let expected = Des::new(&key).unwrap();
        let block = *b"abcdefgh";
        for mask in 0..256u16 {
            let changed = std::array::from_fn::<_, 8, _>(|i| key[i] ^ ((mask >> i) & 1) as u8);
            let actual = Des::new(&changed).unwrap();
            assert_eq!(actual.encrypt_block(&block), expected.encrypt_block(&block));
            assert_eq!(actual.decrypt_block(&block), expected.decrypt_block(&block));
        }
    }
}
