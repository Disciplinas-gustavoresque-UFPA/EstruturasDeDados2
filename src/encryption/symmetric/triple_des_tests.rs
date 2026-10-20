use super::{Des, TripleDes};
use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

fn compare(key: &[u8], inputs: &[[u8; 8]]) {
    let ours = TripleDes::new(key).unwrap();
    // TdesEde3 recebe K1 || K2 || K1 para representar a variante de duas chaves.
    let mut expanded = [0; 24];
    expanded[..key.len()].copy_from_slice(key);
    if key.len() == 16 {
        expanded[16..].copy_from_slice(&key[..8]);
    }
    let reference = des::TdesEde3::new_from_slice(&expanded).unwrap();
    for input in inputs {
        let mut block = (*input).into();
        reference.encrypt_block(&mut block);
        let expected: [u8; 8] = block.into();
        assert_eq!(ours.encrypt_block(input), expected);
        assert_eq!(ours.decrypt_block(&expected), *input);
        let mut block = (*input).into();
        reference.decrypt_block(&mut block);
        let expected: [u8; 8] = block.into();
        assert_eq!(ours.decrypt_block(input), expected);
    }
}

#[test]
fn vetores_publicados_na_sp800_67_apendice_b() {
    // https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-67r1.pdf
    let key: Vec<_> = [
        0x0123456789abcdefu64,
        0x23456789abcdef01,
        0x456789abcdef0123,
    ]
    .into_iter()
    .flat_map(u64::to_be_bytes)
    .collect();
    let ours = TripleDes::new(&key).unwrap();
    let vectors = [
        // A legenda ASCII da publicação diverge do hexadecimal no primeiro bloco.
        // Usa-se o valor hexadecimal publicado, sem alterar o resultado esperado.
        (0x5468652071756663u64.to_be_bytes(), 0xa826fd8ce53b855fu64),
        (*b"k brown ", 0xcce21c8112256fe6),
        (*b"fox jump", 0x68d5c05dd9b6b900),
    ];
    for (plain, encrypted) in vectors {
        assert_eq!(ours.encrypt_block(&plain), encrypted.to_be_bytes());
        assert_eq!(ours.decrypt_block(&encrypted.to_be_bytes()), plain);
    }
}

#[test]
fn compara_duas_e_tres_chaves_com_padroes_e_bits_variaveis() {
    let mut inputs = vec![[0; 8], [0xff; 8], [0x55; 8], [0xaa; 8], *b"mensagem"];
    for bit in 0..64 {
        let mut input = [0; 8];
        input[bit / 8] = 1 << (bit % 8);
        inputs.push(input);
    }
    for length in [16, 24] {
        for fill in [0, 0xff, 0xaa, 0x55] {
            compare(&vec![fill; length], &inputs);
        }
        for bit in 0..length * 8 {
            let mut key = vec![0; length];
            key[bit / 8] = 1 << (bit % 8);
            compare(&key, &[*b"abcdefgh"]);
        }
    }
}

#[test]
fn duas_chaves_equivalem_a_k1_k2_k1() {
    let two = TripleDes::new(b"12345678abcdefgh").unwrap();
    let three = TripleDes::new(b"12345678abcdefgh12345678").unwrap();
    let reference = des::TdesEde2::new_from_slice(b"12345678abcdefgh").unwrap();
    for input in [[0; 8], [0xff; 8], *b"abcdefgh"] {
        assert_eq!(two.encrypt_block(&input), three.encrypt_block(&input));
        assert_eq!(two.decrypt_block(&input), three.decrypt_block(&input));
        let mut expected = input.into();
        reference.encrypt_block(&mut expected);
        let expected: [u8; 8] = expected.into();
        assert_eq!(two.encrypt_block(&input), expected);
    }
}

#[test]
fn tres_chaves_iguais_equivalem_ao_des_inclusive_chaves_fracas() {
    for key in [[0; 8], [0xff; 8], [1; 8], *b"12345678"] {
        let des = Des::new(&key).unwrap();
        for length in [16, 24] {
            let triple = TripleDes::new(&key.repeat(length / 8)).unwrap();
            for input in [[0; 8], [0xff; 8], *b"abcdefgh"] {
                assert_eq!(triple.encrypt_block(&input), des.encrypt_block(&input));
                assert_eq!(triple.decrypt_block(&input), des.decrypt_block(&input));
            }
        }
    }
}

#[test]
fn paridade_nao_altera_qualquer_subchave() {
    let key = *b"12345678abcdefgh87654321";
    let original = TripleDes::new(&key).unwrap();
    for index in 0..24 {
        let mut changed = key;
        changed[index] ^= 1;
        let changed = TripleDes::new(&changed).unwrap();
        assert_eq!(
            original.encrypt_block(b"abcdefgh"),
            changed.encrypt_block(b"abcdefgh")
        );
        assert_eq!(
            original.decrypt_block(b"abcdefgh"),
            changed.decrypt_block(b"abcdefgh")
        );
    }
}

#[test]
fn amostras_deterministicas_e_instancias_independentes() {
    let mut state = 0xbb67ae8584caa73bu64;
    let first = TripleDes::new(b"12345678abcdefgh").unwrap();
    let before = first.encrypt_block(b"abcdefgh");
    for length in [16, 24] {
        for _ in 0..128 {
            let mut key = vec![0; length];
            for byte in &mut key {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                *byte = (state >> 32) as u8;
            }
            compare(&key, &[state.to_be_bytes()]);
            assert_eq!(first.encrypt_block(b"abcdefgh"), before);
        }
    }
}
