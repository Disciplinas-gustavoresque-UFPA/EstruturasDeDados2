//! 3DES (TDEA) em composição EDE, com duas ou três chaves DES.
//!
//! Primitiva histórica para estudo: sem padding, autenticação ou modo de operação.
//! Reutiliza DES, inclusive o descarte dos bits de paridade. Não rejeita chaves
//! fracas ou degeneradas e não oferece garantia contra canais laterais.

use super::des::Des;

/// Comprimento inválido de uma chave 3DES, em bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TripleDesKeyError {
    pub length: usize,
}

impl std::fmt::Display for TripleDesKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a chave 3DES deve conter 16 ou 24 bytes; recebidos {}",
            self.length
        )
    }
}

impl std::error::Error for TripleDesKeyError {}

/// Três chaves DES expandidas; na variante de duas chaves, K3 = K1.
pub struct TripleDes {
    first: Des,
    second: Des,
    third: Des,
}

impl TripleDes {
    /// Expande K1 || K2 (16 bytes) ou K1 || K2 || K3 (24 bytes).
    pub fn new(key: &[u8]) -> Result<Self, TripleDesKeyError> {
        if key.len() != 16 && key.len() != 24 {
            return Err(TripleDesKeyError { length: key.len() });
        }
        // Após a validação, cada slice possui exatamente oito bytes.
        let expand = |part: &[u8]| Des::new(part).expect("subchave DES de oito bytes");
        Ok(Self {
            first: expand(&key[..8]),
            second: expand(&key[8..16]),
            third: expand(if key.len() == 16 {
                &key[..8]
            } else {
                &key[16..]
            }),
        })
    }

    /// E(K3, D(K2, E(K1, bloco))).
    pub fn encrypt_block(&self, block: &[u8; 8]) -> [u8; 8] {
        self.third
            .encrypt_block(&self.second.decrypt_block(&self.first.encrypt_block(block)))
    }

    /// D(K1, E(K2, D(K3, bloco))).
    pub fn decrypt_block(&self, block: &[u8; 8]) -> [u8; 8] {
        self.first
            .decrypt_block(&self.second.encrypt_block(&self.third.decrypt_block(block)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemplo_com_tres_chaves() {
        let cipher = TripleDes::new(b"12345678abcdefgh87654321").unwrap();
        let plain = *b"mensagem";
        assert_eq!(cipher.decrypt_block(&cipher.encrypt_block(&plain)), plain);
    }

    #[test]
    fn rejeita_comprimentos_invalidos() {
        for length in 0..=64 {
            match TripleDes::new(&vec![0; length]) {
                Ok(_) => assert!(length == 16 || length == 24),
                Err(error) => {
                    assert!(length != 16 && length != 24);
                    assert_eq!(error, TripleDesKeyError { length });
                    assert!(error.to_string().contains(&length.to_string()));
                    let _: &dyn std::error::Error = &error;
                }
            }
        }
    }
}
