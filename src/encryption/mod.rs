//! Classical and byte-oriented symmetric ciphers with owned outputs.

mod symmetric;

pub use symmetric::{
    Des, DesKeyError, TripleDes, TripleDesKeyError, Twofish, TwofishKeyError, caesar_cipher, rc4,
    vigenere_cipher, xor_encrypt,
};
