use super::{Twofish, q};
use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

fn hex(text: &str) -> Vec<u8> {
    assert!(text.len().is_multiple_of(2));
    (0..text.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap())
        .collect()
}

fn compare(key: &[u8], inputs: &[[u8; 16]]) {
    let ours = Twofish::new(key).unwrap();
    let reference = twofish::Twofish::new_from_slice(key).unwrap();
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

#[test]
fn vetores_dos_autores_com_chaves_nao_nulas() {
    // Exemplos do artigo/livro Twofish, disponíveis no site dos autores.
    // https://www.schneier.com/academic/twofish/
    for (key, encrypted) in [
        (
            "0123456789abcdeffedcba9876543210",
            "dfbdf399d0d3112ed11900427bf1f9f4",
        ),
        (
            "0123456789abcdeffedcba98765432100011223344556677",
            "cfd1d2e5a9be9cdf501f13b892bd2248",
        ),
        (
            "0123456789abcdeffedcba987654321000112233445566778899aabbccddeeff",
            "37527be0052334b89f0cfccae87cfa20",
        ),
    ] {
        let ours = Twofish::new(&hex(key)).unwrap();
        let encrypted: [u8; 16] = hex(encrypted).try_into().unwrap();
        assert_eq!(ours.encrypt_block(&[0; 16]), encrypted);
        assert_eq!(ours.decrypt_block(&encrypted), [0; 16]);
    }
}

#[test]
fn permutacoes_q_sao_bijetivas_e_conferem_com_os_valores_publicados() {
    assert_eq!(q(0, 0), 0xa9);
    assert_eq!(q(1, 0), 0x75);
    for which in 0..2 {
        let mut seen = [false; 256];
        for input in 0..=255 {
            let output = usize::from(q(which, input));
            assert!(!seen[output]);
            seen[output] = true;
        }
        assert!(seen.into_iter().all(|value| value));
    }
}

#[test]
fn compara_tamanhos_padroes_binarios_e_cada_bit_do_bloco() {
    let mut inputs = vec![[0; 16], [0xff; 16], [0x55; 16], [0xaa; 16]];
    inputs.push(std::array::from_fn(|byte| byte as u8));
    for bit in 0..128 {
        let mut input = [0; 16];
        input[bit / 8] = 1 << (bit % 8);
        inputs.push(input);
    }
    for length in [16, 24, 32] {
        for fill in [0, 0xff, 0x55, 0xaa] {
            compare(&vec![fill; length], &inputs);
        }
    }
}

#[test]
fn cada_bit_da_chave_inclui_rs_e_as_camadas_adicionais_de_h() {
    for length in [16, 24, 32] {
        for bit in 0..length * 8 {
            let mut key = vec![0; length];
            key[bit / 8] = 1 << (bit % 8);
            compare(&key, &[std::array::from_fn(|byte| (17 * byte) as u8)]);
        }
    }
}

#[test]
fn compara_amostras_deterministicas_em_ambas_as_direcoes() {
    let mut state = 0x510e527fade682d1u64;
    for length in [16, 24, 32] {
        for _ in 0..256 {
            let mut key = vec![0; length];
            let mut input = [0; 16];
            for byte in key.iter_mut().chain(input.iter_mut()) {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                *byte = (state >> 32) as u8;
            }
            compare(&key, &[input]);
        }
    }
}

#[test]
fn instancias_e_chaves_expandidas_sao_independentes() {
    let first = Twofish::new(&[0; 16]).unwrap();
    let before = first.encrypt_block(&[0; 16]);
    let second = Twofish::new(&[0xff; 32]).unwrap();
    assert_ne!(before, second.encrypt_block(&[0; 16]));
    assert_eq!(first.encrypt_block(&[0; 16]), before);
}
