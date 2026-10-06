use super::Twofish;
use crate::encryption::{twofish_decrypt, twofish_encrypt};

struct Vector {
    bits: usize,
    index: usize,
    key: Vec<u8>,
    plaintext: [u8; 16],
    ciphertext: [u8; 16],
}

fn decode_hex(hex: &str) -> Vec<u8> {
    assert!(hex.len().is_multiple_of(2));
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap())
        .collect()
}

fn decode_block(hex: &str) -> [u8; 16] {
    decode_hex(hex).try_into().unwrap()
}

// The official VK file shares PT across a section, whereas VT shares KEY.
// Retain those fields until the next KEYSIZE; emit every CT without filtering.
fn known_answer_vectors(data: &str) -> Vec<Vector> {
    let mut vectors = Vec::new();
    let (mut bits, mut index) = (0, 0);
    let (mut key, mut plaintext) = (None, None);
    for line in data.lines().map(str::trim) {
        if let Some(value) = line.strip_prefix("KEYSIZE=") {
            bits = value.parse().unwrap();
            key = None;
            plaintext = None;
        } else if let Some(value) = line.strip_prefix("I=") {
            index = value.parse().unwrap();
        } else if let Some(value) = line.strip_prefix("KEY=") {
            key = Some(decode_hex(value));
        } else if let Some(value) = line.strip_prefix("PT=") {
            plaintext = Some(decode_block(value));
        } else if let Some(value) = line.strip_prefix("CT=") {
            let key = key.as_ref().expect("every vector must specify a key");
            assert_eq!(key.len() * 8, bits);
            vectors.push(Vector {
                bits,
                index,
                key: key.clone(),
                plaintext: plaintext.expect("every vector must specify a plaintext"),
                ciphertext: decode_block(value),
            });
        }
    }
    vectors
}

fn assert_vector(vector: &Vector) {
    assert_eq!(
        twofish_encrypt(&vector.plaintext, &vector.key),
        vector.ciphertext,
        "encryption: KEYSIZE={}, I={}",
        vector.bits,
        vector.index
    );
    assert_eq!(
        twofish_decrypt(&vector.ciphertext, &vector.key),
        vector.plaintext,
        "decryption: KEYSIZE={}, I={}",
        vector.bits,
        vector.index
    );
}

fn assert_known_answers(data: &str, expected_counts: [usize; 3]) {
    let mut counts = [0; 3];
    for vector in known_answer_vectors(data) {
        let position = [128, 192, 256]
            .iter()
            .position(|&bits| bits == vector.bits)
            .unwrap();
        counts[position] += 1;
        assert_eq!(vector.index, counts[position]);
        assert_vector(&vector);
    }
    assert_eq!(
        counts, expected_counts,
        "all published cases must be checked"
    );
}

#[test]
fn test_official_variable_key_vectors() {
    assert_known_answers(include_str!("test_data/ECB_VK.TXT"), [128, 192, 256]);
}

#[test]
fn test_official_variable_text_vectors() {
    assert_known_answers(include_str!("test_data/ECB_VT.TXT"), [128; 3]);
}

#[test]
fn test_official_table_vectors() {
    assert_known_answers(include_str!("test_data/ECB_TBL.TXT"), [49; 3]);
}

#[test]
fn test_official_q_permutations() {
    let permutations: Vec<_> = include_str!("test_data/q_permutations.txt")
        .lines()
        .collect();
    assert_eq!(permutations.len(), 2);
    for (permutation, hex) in permutations.into_iter().enumerate() {
        let expected = decode_hex(hex);
        assert_eq!(expected.len(), 256);
        for value in 0..=255 {
            assert_eq!(super::q(permutation, value), expected[usize::from(value)]);
        }
    }
}

#[test]
fn test_official_intermediate_key_schedules() {
    let sections: Vec<_> = include_str!("test_data/ECB_IVAL.TXT")
        .split("KEYSIZE=")
        .skip(1)
        .collect();
    assert_eq!(sections.len(), 3);
    for section in sections {
        let key = section
            .lines()
            .find_map(|line| line.strip_prefix("KEY="))
            .unwrap();
        let state = Twofish::new(&decode_hex(key));
        let (rs_comments, subkey_comments) = section.split_once(";           Subkeys").unwrap();
        let expected_s: Vec<u32> = rs_comments
            .lines()
            .filter_map(|line| line.split_once("-->"))
            .map(|(_, output)| output.trim())
            .filter(|output| !output.starts_with("S-box key"))
            .map(|output| u32::from_str_radix(output, 16).unwrap())
            .rev()
            .collect();
        assert_eq!(expected_s.len(), state.word_count);
        assert_eq!(state.s_words[..state.word_count], expected_s);
        let expected_subkeys: Vec<u32> = subkey_comments
            .split_once("PT=")
            .unwrap()
            .0
            .split_whitespace()
            .filter(|word| word.len() == 8 && word.bytes().all(|byte| byte.is_ascii_hexdigit()))
            .map(|word| u32::from_str_radix(word, 16).unwrap())
            .collect();
        assert_eq!(expected_subkeys.len(), 40);
        assert_eq!(state.subkeys.as_slice(), expected_subkeys);
    }
}

#[test]
fn test_fixed_binary_reference_vectors() {
    let mut count = 0;
    for line in include_str!("test_data/reference_vectors.txt").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 3);
        let key = decode_hex(fields[0]);
        count += 1;
        assert_vector(&Vector {
            bits: key.len() * 8,
            index: count,
            key,
            plaintext: decode_block(fields[1]),
            ciphertext: decode_block(fields[2]),
        });
    }
    assert_eq!(count, 778);
}

#[test]
fn test_calls_are_independent_and_preserve_inputs() {
    let block = std::array::from_fn(|index| (index as u8) * 17);
    let original_block = block;
    let keys: [Vec<u8>; 3] = [16, 24, 32].map(|length| (0x80..).take(length).collect());
    let original_keys = keys.clone();
    let expected = keys.each_ref().map(|key| twofish_encrypt(&block, key));
    for index in [0, 1, 2, 1, 0, 2] {
        assert_eq!(twofish_encrypt(&block, &keys[index]), expected[index]);
        assert_eq!(twofish_decrypt(&expected[index], &keys[index]), block);
    }
    assert_eq!(block, original_block);
    assert_eq!(keys, original_keys);
}

#[test]
fn test_all_invalid_key_lengths_panic() {
    let operations = [twofish_encrypt, twofish_decrypt];
    for length in (0..=64)
        .chain([256])
        .filter(|length| !matches!(length, 16 | 24 | 32))
    {
        let key = vec![0xff; length];
        for block in [[0; 16], [0xff; 16]] {
            for operation in operations {
                let panic = std::panic::catch_unwind(|| operation(&block, &key))
                    .expect_err("invalid key lengths must panic");
                let message = panic
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| panic.downcast_ref::<String>().map(String::as_str));
                assert_eq!(
                    message,
                    Some("`key` must contain exactly 16, 24, or 32 bytes.")
                );
            }
        }
    }
}

#[test]
fn test_documented_examples() {
    let block = *b"Twofish in Rust!";
    let key = b"0123456789abcdef";
    assert_eq!(twofish_decrypt(&twofish_encrypt(&block, key), key), block);
    assert_vector(&Vector {
        bits: 128,
        index: 0,
        key: vec![0; 16],
        plaintext: [0; 16],
        ciphertext: decode_block("9F589F5CF6122C32B6BFEC2F2AE8C35A"),
    });
}

// TST2FISH.C's AES_Test_ECB_E_MCT/AES_Test_ECB_D_MCT: 400 outer cases
// for each key size, 10,000 chained block operations with each fixed key.
fn assert_monte_carlo(data: &str, decrypt: bool) {
    let sections: Vec<_> = data.split("KEYSIZE=").skip(1).collect();
    assert_eq!(sections.len(), 3);
    let mut total_cases = 0;
    for (section, expected_bits) in sections.into_iter().zip([128, 192, 256]) {
        let bits: usize = section.lines().next().unwrap().trim().parse().unwrap();
        assert_eq!(bits, expected_bits);
        let mut key = vec![0; bits / 8];
        let mut block = [0; 16];
        let cases: Vec<_> = section.split("\nI=").skip(1).collect();
        assert_eq!(cases.len(), 400);
        for (index, case) in cases.into_iter().enumerate() {
            let published_index: usize = case.lines().next().unwrap().trim().parse().unwrap();
            assert_eq!(published_index, index);
            let field = |prefix| {
                case.lines()
                    .find_map(|line| line.strip_prefix(prefix))
                    .unwrap()
                    .trim()
            };
            assert_eq!(decode_hex(field("KEY=")), key, "KEYSIZE={bits}, I={index}");
            let (input, output) = if decrypt {
                ("CT=", "PT=")
            } else {
                ("PT=", "CT=")
            };
            assert_eq!(
                decode_block(field(input)),
                block,
                "KEYSIZE={bits}, I={index}"
            );

            let state = Twofish::new(&key);
            let mut previous = block;
            for _ in 0..10_000 {
                previous = block;
                block = if decrypt {
                    state.decrypt_block(&block)
                } else {
                    state.encrypt_block(&block)
                };
            }
            assert_eq!(
                block,
                decode_block(field(output)),
                "KEYSIZE={bits}, I={index}"
            );

            // For keys longer than a block, prepend the required suffix of
            // the penultimate output to the final output before XORing the key.
            let prefix = key.len() - 16;
            for (key_byte, &byte) in key[..prefix].iter_mut().zip(&previous[16 - prefix..]) {
                *key_byte ^= byte;
            }
            for (key_byte, &byte) in key[prefix..].iter_mut().zip(&block) {
                *key_byte ^= byte;
            }
            total_cases += 1;
        }
        eprintln!("official ECB Monte Carlo: decrypt={decrypt}, KEYSIZE={bits}, 400 cases passed");
    }
    assert_eq!(total_cases, 1_200);
}

#[test]
#[ignore = "long-running official ECB Monte Carlo validation; run in release"]
fn test_official_monte_carlo_encrypt() {
    assert_monte_carlo(include_str!("test_data/ECB_E_M.TXT"), false);
}

#[test]
#[ignore = "long-running official ECB Monte Carlo validation; run in release"]
fn test_official_monte_carlo_decrypt() {
    assert_monte_carlo(include_str!("test_data/ECB_D_M.TXT"), true);
}
