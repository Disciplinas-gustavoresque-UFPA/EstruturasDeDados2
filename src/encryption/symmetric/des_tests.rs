//! Validação independente: exemplos FIPS 81 e vetores do DES, além de RustCrypto.
use super::Des;
use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

fn compare(ours: &Des, reference: &des::Des, input: [u8; 8]) {
    let mut encrypted = input.into();
    reference.encrypt_block(&mut encrypted);
    let expected: [u8; 8] = encrypted.into();
    assert_eq!(ours.encrypt_block(&input), expected);
    assert_eq!(ours.decrypt_block(&expected), input);
    // Também compara a descriptografia de um bloco arbitrário, sem roundtrip.
    let mut decrypted = input.into();
    reference.decrypt_block(&mut decrypted);
    let expected: [u8; 8] = decrypted.into();
    assert_eq!(ours.decrypt_block(&input), expected);
}

#[test]
fn vetores_publicados_incluem_o_exemplo_da_fips81() {
    // https://csrc.nist.gov/pubs/fips/81/final — exemplo ECB, primeiro bloco.
    let vectors: [(u64, u64, u64); 4] = [
        (0x0123456789abcdef, 0x4e6f772069732074, 0x3fa40e8a984d4815),
        (0, 0, 0x8ca64de9c1b123a7),
        (0xffffffffffffffff, 0xffffffffffffffff, 0x7359b2163e4edc58),
        (0x133457799bbcdff1, 0x0123456789abcdef, 0x85e813540f0ab405),
    ];
    for (key, plain, encrypted) in vectors {
        let ours = Des::new(&key.to_be_bytes()).unwrap();
        assert_eq!(
            ours.encrypt_block(&plain.to_be_bytes()),
            encrypted.to_be_bytes()
        );
        assert_eq!(
            ours.decrypt_block(&encrypted.to_be_bytes()),
            plain.to_be_bytes()
        );
    }
}

#[test]
fn compara_padroes_binarios_e_reutilizacao_da_chave() {
    for key in [[0; 8], [0xff; 8], [0x55; 8], [0xaa; 8], *b"12345678"] {
        let ours = Des::new(&key).unwrap();
        let reference = des::Des::new_from_slice(&key).unwrap();
        for input in [[0; 8], [0xff; 8], [0x55; 8], [0xaa; 8], *b"abcdefgh"] {
            compare(&ours, &reference, input);
        }
        for bit in 0..64 {
            let mut input = [0; 8];
            input[bit / 8] = 1 << (bit % 8);
            compare(&ours, &reference, input);
        }
    }
}

#[test]
fn compara_cada_bit_da_chave_e_amostras_deterministicas() {
    for bit in 0..64 {
        let mut key = [0; 8];
        key[bit / 8] = 1 << (bit % 8);
        compare(
            &Des::new(&key).unwrap(),
            &des::Des::new_from_slice(&key).unwrap(),
            *b"abcdefgh",
        );
    }
    let mut state = 0x6a09e667f3bcc909u64;
    for _ in 0..512 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let key = state.to_be_bytes();
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        compare(
            &Des::new(&key).unwrap(),
            &des::Des::new_from_slice(&key).unwrap(),
            state.to_be_bytes(),
        );
    }
}

#[test]
fn chaves_fracas_e_semifracas_preservam_as_propriedades_do_des() {
    for key in [
        0x0101010101010101u64,
        0xfefefefefefefefe,
        0xe0e0e0e0f1f1f1f1,
        0x1f1f1f1f0e0e0e0e,
    ] {
        let ours = Des::new(&key.to_be_bytes()).unwrap();
        compare(
            &ours,
            &des::Des::new_from_slice(&key.to_be_bytes()).unwrap(),
            *b"abcdefgh",
        );
        assert_eq!(
            ours.encrypt_block(&ours.encrypt_block(b"abcdefgh")),
            *b"abcdefgh"
        );
    }
    let first = Des::new(&0x01fe01fe01fe01feu64.to_be_bytes()).unwrap();
    let second = Des::new(&0xfe01fe01fe01fe01u64.to_be_bytes()).unwrap();
    assert_eq!(
        first.encrypt_block(b"abcdefgh"),
        second.decrypt_block(b"abcdefgh")
    );
}

#[test]
fn instancias_nao_compartilham_estado() {
    let first = Des::new(b"12345678").unwrap();
    let before = first.encrypt_block(b"abcdefgh");
    let second = Des::new(b"87654321").unwrap();
    assert_ne!(before, second.encrypt_block(b"abcdefgh"));
    assert_eq!(first.encrypt_block(b"abcdefgh"), before);
}
