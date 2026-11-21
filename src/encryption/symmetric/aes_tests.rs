use super::Aes;
use cipher::{BlockDecrypt, BlockEncrypt, BlockSizeUser, KeyInit, consts::U16};

fn hex(text: &str) -> Vec<u8> {
    assert!(text.len().is_multiple_of(2));
    (0..text.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap())
        .collect()
}

fn compare_variant<C: BlockEncrypt + BlockDecrypt + KeyInit + BlockSizeUser<BlockSize = U16>>(
    key: &[u8],
    inputs: &[[u8; 16]],
) {
    let ours = Aes::new(key).unwrap();
    let reference = C::new_from_slice(key).unwrap();
    for input in inputs {
        let mut block = (*input).into();
        reference.encrypt_block(&mut block);
        let expected: [u8; 16] = block.into();
        assert_eq!(ours.encrypt_block(input), expected);
        assert_eq!(ours.decrypt_block(&expected), *input);
        let mut block = (*input).into();
        reference.decrypt_block(&mut block);
        let expected: [u8; 16] = block.into();
        assert_eq!(ours.decrypt_block(input), expected);
    }
}

fn compare(key: &[u8], inputs: &[[u8; 16]]) {
    match key.len() {
        16 => compare_variant::<aes::Aes128>(key, inputs),
        24 => compare_variant::<aes::Aes192>(key, inputs),
        32 => compare_variant::<aes::Aes256>(key, inputs),
        _ => unreachable!("comprimento usado pelos testes"),
    }
}

#[test]
fn doze_vetores_da_sp800_38a_para_as_tres_variantes() {
    // Exemplos ECB da SP 800-38A: cada bloco é testado independentemente.
    // https://csrc.nist.gov/pubs/sp/800/38/a/final
    let plains = [
        "6bc1bee22e409f96e93d7e117393172a",
        "ae2d8a571e03ac9c9eb76fac45af8e51",
        "30c81c46a35ce411e5fbc1191a0a52ef",
        "f69f2445df4f9b17ad2b417be66c3710",
    ];
    for (key, encrypted) in [
        (
            "2b7e151628aed2a6abf7158809cf4f3c",
            [
                "3ad77bb40d7a3660a89ecaf32466ef97",
                "f5d3d58503b9699de785895a96fdbaaf",
                "43b1cd7f598ece23881b00e3ed030688",
                "7b0c785e27e8ad3f8223207104725dd4",
            ],
        ),
        (
            "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b",
            [
                "bd334f1d6e45f25ff712a214571fa5cc",
                "974104846d0ad3ad7734ecb3ecee4eef",
                "ef7afd2270e2e60adce0ba2face6444e",
                "9a4b41ba738d6c72fb16691603c18e0e",
            ],
        ),
        (
            "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            [
                "f3eed1bdb5d2a03c064b5a7e3db181f8",
                "591ccb10d410ed26dc5ba74a31362870",
                "b6ed21b99ca6f4f9f153e7b1beafed1d",
                "23304b7a39f9f3ff067d8d8f9e24ecc7",
            ],
        ),
    ] {
        let ours = Aes::new(&hex(key)).unwrap();
        for (plain, encrypted) in plains.into_iter().zip(encrypted) {
            let plain: [u8; 16] = hex(plain).try_into().unwrap();
            let encrypted: [u8; 16] = hex(encrypted).try_into().unwrap();
            assert_eq!(ours.encrypt_block(&plain), encrypted);
            assert_eq!(ours.decrypt_block(&encrypted), plain);
        }
    }
}

#[test]
fn compara_tamanhos_padroes_e_cada_bit_do_bloco() {
    let mut inputs = vec![[0; 16], [0xff; 16], [0x55; 16], [0xaa; 16]];
    inputs.push(std::array::from_fn(|byte| byte as u8));
    for bit in 0..128 {
        let mut block = [0; 16];
        block[bit / 8] = 1 << (bit % 8);
        inputs.push(block);
    }
    for length in [16, 24, 32] {
        for fill in [0, 0xff, 0x55, 0xaa] {
            compare(&vec![fill; length], &inputs);
        }
    }
}

#[test]
fn cada_bit_da_chave_inclusive_as_palavras_adicionais_do_aes256() {
    for length in [16, 24, 32] {
        for bit in 0..length * 8 {
            let mut key = vec![0; length];
            key[bit / 8] = 1 << (bit % 8);
            compare(&key, &[std::array::from_fn(|byte| (17 * byte) as u8)]);
        }
    }
}

#[test]
fn compara_amostras_deterministicas_nas_tres_variantes() {
    let mut state = 0xa54ff53a5f1d36f1u64;
    for length in [16, 24, 32] {
        for _ in 0..256 {
            let mut key = vec![0; length];
            let mut block = [0; 16];
            for byte in key.iter_mut().chain(block.iter_mut()) {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                *byte = (state >> 32) as u8;
            }
            compare(&key, &[block]);
        }
    }
}

#[test]
fn instancias_independentes_reutilizam_chaves_sem_estado_de_fluxo() {
    let first = Aes::new(&[0; 16]).unwrap();
    let before = first.encrypt_block(&[0; 16]);
    let second = Aes::new(&[0xff; 32]).unwrap();
    assert_ne!(before, second.encrypt_block(&[0; 16]));
    assert_eq!(first.encrypt_block(&[0; 16]), before);
}
