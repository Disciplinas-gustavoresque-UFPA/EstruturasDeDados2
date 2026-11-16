use super::Blowfish;
use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

fn compare(ours: &Blowfish, reference: &blowfish::Blowfish, input: [u8; 8]) {
    let mut block = input.into();
    reference.encrypt_block(&mut block);
    let expected: [u8; 8] = block.into();
    assert_eq!(ours.encrypt_block(&input), expected);
    assert_eq!(ours.decrypt_block(&expected), input);
    let mut block = input.into();
    reference.decrypt_block(&mut block);
    let expected: [u8; 8] = block.into();
    assert_eq!(ours.decrypt_block(&input), expected);
}

#[test]
fn vetores_publicados_por_eric_young() {
    // https://www.schneier.com/wp-content/uploads/2015/12/vectors-2.txt
    let vectors: [(u64, u64, u64); 8] = [
        (0, 0, 0x4ef997456198dd78),
        (u64::MAX, u64::MAX, 0x51866fd5b85ecb8a),
        (0x3000000000000000, 0x1000000000000001, 0x7d856f9a613063f2),
        (0x1111111111111111, 0x1111111111111111, 0x2466dd878b963c9d),
        (0x0123456789abcdef, 0x1111111111111111, 0x61f9c3802281b096),
        (0x1111111111111111, 0x0123456789abcdef, 0x7d0cc630afda1ec7),
        (0xfedcba9876543210, 0x0123456789abcdef, 0x0aceab0fc6a0a28d),
        (0x7ca110454a1a6e57, 0x01a1d6d039776742, 0x59c68245eb05282b),
    ];
    for (key, plain, encrypted) in vectors {
        let ours = Blowfish::new(&key.to_be_bytes()).unwrap();
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
fn vetores_publicados_com_comprimentos_variaveis() {
    let key = [
        0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b, 0x3c, 0x2d, 0x1e,
        0x0f, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
    ];
    let encrypted: [u64; 21] = [
        0xbe1e639408640f05,
        0xb39e44481bdb1e6e,
        0x9457aa83b1928c0d,
        0x8bb77032f960629d,
        0xe87a244e2cc85e82,
        0x15750e7a4f4ec577,
        0x122ba70b3ab64ae0,
        0x3a833c9affc537f6,
        0x9409da87a90f6bf2,
        0x884f80625060b8b4,
        0x1f85031c19e11968,
        0x79d9373a714ca34f,
        0x93142887ee3be15c,
        0x03429e838ce2d14b,
        0xa4299e27469ff67b,
        0xafd5aed1c1bc96a8,
        0x10851c0e3858da9f,
        0xe6f51ed79b9db21f,
        0x64a6e14afd36b46f,
        0x80c7d7d45a5479ad,
        0x05044b62fa52d080,
    ];
    let plain = 0xfedcba9876543210u64.to_be_bytes();
    for (offset, encrypted) in encrypted.into_iter().enumerate() {
        let ours = Blowfish::new(&key[..offset + 4]).unwrap();
        assert_eq!(ours.encrypt_block(&plain), encrypted.to_be_bytes());
        assert_eq!(ours.decrypt_block(&encrypted.to_be_bytes()), plain);
    }
}

#[test]
fn todos_os_tamanhos_de_chave_com_padroes_binarios_e_bits_do_bloco() {
    for length in 4..=56 {
        for fill in [0u8, 0xff, 0x55, 0xaa] {
            let key = vec![fill; length];
            let ours = Blowfish::new(&key).unwrap();
            let reference = blowfish::Blowfish::new_from_slice(&key).unwrap();
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
}

#[test]
fn cada_bit_da_chave_inclusive_o_ultimo_byte_e_amostras_deterministicas() {
    for length in [4, 5, 16, 32, 55, 56] {
        for bit in 0..length * 8 {
            let mut key = vec![0; length];
            key[bit / 8] = 1 << (bit % 8);
            compare(
                &Blowfish::new(&key).unwrap(),
                &blowfish::Blowfish::new_from_slice(&key).unwrap(),
                *b"abcdefgh",
            );
        }
    }
    let mut state = 0x3c6ef372fe94f82bu64;
    for length in 4..=56 {
        let mut key = vec![0; length];
        for byte in &mut key {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            *byte = (state >> 32) as u8;
        }
        compare(
            &Blowfish::new(&key).unwrap(),
            &blowfish::Blowfish::new_from_slice(&key).unwrap(),
            state.to_be_bytes(),
        );
    }
}

#[test]
fn instancias_independentes_e_sboxes_isoladas() {
    let first = Blowfish::new(b"primeira chave").unwrap();
    let before = first.encrypt_block(b"mensagem");
    let second = Blowfish::new(b"segunda chave").unwrap();
    assert_ne!(before, second.encrypt_block(b"mensagem"));
    assert_eq!(first.encrypt_block(b"mensagem"), before);
}
