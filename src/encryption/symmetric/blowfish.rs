//! Blowfish's 64-bit block cipher with a fresh key schedule for each call.
//! Blocks and key words use big-endian byte order. No operation mode or padding
//! is applied; the caller supplies exactly one eight-byte block.

/// Encrypts one eight-byte block using a key of 4 to 56 bytes.
/// Returns raw ciphertext bytes and leaves both inputs unchanged. Each call
/// expands the key independently; use [`blowfish_decrypt`] to recover the block.
///
/// # Panics
///
/// Panics if `key` contains fewer than 4 or more than 56 bytes.
///
/// # Examples
///
/// ```
/// use crate::encryption::{blowfish_decrypt, blowfish_encrypt};
///
/// let block = *b"Blowfish";
/// let key = b"mysecretkey";
/// let encrypted = blowfish_encrypt(&block, key);
/// assert_eq!(blowfish_decrypt(&encrypted, key), block);
/// ```
pub fn blowfish_encrypt(block: &[u8; 8], key: &[u8]) -> [u8; 8] {
    Blowfish::new(key).crypt_block(block, false)
}

/// Decrypts one eight-byte block using the same 4-to-56-byte key as encryption.
/// Returns raw plaintext bytes and leaves both inputs unchanged. Each call
/// expands the key independently, then uses the round subkeys in reverse order.
///
/// # Panics
///
/// Panics if `key` contains fewer than 4 or more than 56 bytes.
///
/// # Examples
///
/// ```
/// use crate::encryption::blowfish_decrypt;
///
/// let ciphertext = [0x4e, 0xf9, 0x97, 0x45, 0x61, 0x98, 0xdd, 0x78];
/// assert_eq!(blowfish_decrypt(&ciphertext, &[0; 8]), [0; 8]);
/// ```
pub fn blowfish_decrypt(block: &[u8; 8], key: &[u8]) -> [u8; 8] {
    Blowfish::new(key).crypt_block(block, true)
}

struct Blowfish {
    p: [u32; 18],
    s: [[u32; 256]; 4],
}

impl Blowfish {
    fn new(key: &[u8]) -> Self {
        assert!(
            (4..=56).contains(&key.len()),
            "`key` must contain between 4 and 56 bytes."
        );

        let mut state = Self {
            p: INITIAL_P,
            s: INITIAL_S,
        };

        // XOR cyclic, big-endian key words into the initial P-array.
        let mut key_index = 0;
        for entry in &mut state.p {
            let mut word = 0;
            for _ in 0..4 {
                word = (word << 8) | u32::from(key[key_index]);
                key_index = (key_index + 1) % key.len();
            }
            *entry ^= word;
        }

        // Carry each encrypted pair forward using the progressively updated
        // state: 9 encryptions for P and 128 for each of the four S-boxes.
        let mut words = [0; 2];
        for pair in 0..9 {
            words = state.crypt_words(words, false);
            state.p[pair * 2..pair * 2 + 2].copy_from_slice(&words);
        }
        for box_index in 0..4 {
            for pair in 0..128 {
                words = state.crypt_words(words, false);
                state.s[box_index][pair * 2..pair * 2 + 2].copy_from_slice(&words);
            }
        }

        state
    }

    fn round_function(&self, word: u32) -> u32 {
        let [first, second, third, fourth] = word.to_be_bytes().map(usize::from);
        (self.s[0][first].wrapping_add(self.s[1][second]) ^ self.s[2][third])
            .wrapping_add(self.s[3][fourth])
    }

    fn crypt_words(&self, [mut left, mut right]: [u32; 2], decrypt: bool) -> [u32; 2] {
        for round in 0..16 {
            let subkey = if decrypt { 17 - round } else { round };
            left ^= self.p[subkey];
            right ^= self.round_function(left);
            std::mem::swap(&mut left, &mut right);
        }

        // Undo the final round's swap before applying the last two subkeys.
        std::mem::swap(&mut left, &mut right);
        right ^= self.p[if decrypt { 1 } else { 16 }];
        left ^= self.p[if decrypt { 0 } else { 17 }];
        [left, right]
    }

    fn crypt_block(&self, block: &[u8; 8], decrypt: bool) -> [u8; 8] {
        let words = [
            u32::from_be_bytes([block[0], block[1], block[2], block[3]]),
            u32::from_be_bytes([block[4], block[5], block[6], block[7]]),
        ];
        let [left, right] = self.crypt_words(words, decrypt);
        let mut output = [0; 8];
        output[..4].copy_from_slice(&left.to_be_bytes());
        output[4..].copy_from_slice(&right.to_be_bytes());
        output
    }
}

// Initial hexadecimal digits of pi, from Bruce Schneier's published tables.
// https://www.schneier.com/wp-content/uploads/2015/12/constants-2.txt
const INITIAL_P: [u32; 18] = [
    0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344, 0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89,
    0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c, 0xc0ac29b7, 0xc97c50dd, 0x3f84d5b5, 0xb5470917,
    0x9216d5d9, 0x8979fb1b,
];

const INITIAL_S: [[u32; 256]; 4] = [
    [
        0xd1310ba6, 0x98dfb5ac, 0x2ffd72db, 0xd01adfb7, 0xb8e1afed, 0x6a267e96, 0xba7c9045,
        0xf12c7f99, 0x24a19947, 0xb3916cf7, 0x0801f2e2, 0x858efc16, 0x636920d8, 0x71574e69,
        0xa458fea3, 0xf4933d7e, 0x0d95748f, 0x728eb658, 0x718bcd58, 0x82154aee, 0x7b54a41d,
        0xc25a59b5, 0x9c30d539, 0x2af26013, 0xc5d1b023, 0x286085f0, 0xca417918, 0xb8db38ef,
        0x8e79dcb0, 0x603a180e, 0x6c9e0e8b, 0xb01e8a3e, 0xd71577c1, 0xbd314b27, 0x78af2fda,
        0x55605c60, 0xe65525f3, 0xaa55ab94, 0x57489862, 0x63e81440, 0x55ca396a, 0x2aab10b6,
        0xb4cc5c34, 0x1141e8ce, 0xa15486af, 0x7c72e993, 0xb3ee1411, 0x636fbc2a, 0x2ba9c55d,
        0x741831f6, 0xce5c3e16, 0x9b87931e, 0xafd6ba33, 0x6c24cf5c, 0x7a325381, 0x28958677,
        0x3b8f4898, 0x6b4bb9af, 0xc4bfe81b, 0x66282193, 0x61d809cc, 0xfb21a991, 0x487cac60,
        0x5dec8032, 0xef845d5d, 0xe98575b1, 0xdc262302, 0xeb651b88, 0x23893e81, 0xd396acc5,
        0x0f6d6ff3, 0x83f44239, 0x2e0b4482, 0xa4842004, 0x69c8f04a, 0x9e1f9b5e, 0x21c66842,
        0xf6e96c9a, 0x670c9c61, 0xabd388f0, 0x6a51a0d2, 0xd8542f68, 0x960fa728, 0xab5133a3,
        0x6eef0b6c, 0x137a3be4, 0xba3bf050, 0x7efb2a98, 0xa1f1651d, 0x39af0176, 0x66ca593e,
        0x82430e88, 0x8cee8619, 0x456f9fb4, 0x7d84a5c3, 0x3b8b5ebe, 0xe06f75d8, 0x85c12073,
        0x401a449f, 0x56c16aa6, 0x4ed3aa62, 0x363f7706, 0x1bfedf72, 0x429b023d, 0x37d0d724,
        0xd00a1248, 0xdb0fead3, 0x49f1c09b, 0x075372c9, 0x80991b7b, 0x25d479d8, 0xf6e8def7,
        0xe3fe501a, 0xb6794c3b, 0x976ce0bd, 0x04c006ba, 0xc1a94fb6, 0x409f60c4, 0x5e5c9ec2,
        0x196a2463, 0x68fb6faf, 0x3e6c53b5, 0x1339b2eb, 0x3b52ec6f, 0x6dfc511f, 0x9b30952c,
        0xcc814544, 0xaf5ebd09, 0xbee3d004, 0xde334afd, 0x660f2807, 0x192e4bb3, 0xc0cba857,
        0x45c8740f, 0xd20b5f39, 0xb9d3fbdb, 0x5579c0bd, 0x1a60320a, 0xd6a100c6, 0x402c7279,
        0x679f25fe, 0xfb1fa3cc, 0x8ea5e9f8, 0xdb3222f8, 0x3c7516df, 0xfd616b15, 0x2f501ec8,
        0xad0552ab, 0x323db5fa, 0xfd238760, 0x53317b48, 0x3e00df82, 0x9e5c57bb, 0xca6f8ca0,
        0x1a87562e, 0xdf1769db, 0xd542a8f6, 0x287effc3, 0xac6732c6, 0x8c4f5573, 0x695b27b0,
        0xbbca58c8, 0xe1ffa35d, 0xb8f011a0, 0x10fa3d98, 0xfd2183b8, 0x4afcb56c, 0x2dd1d35b,
        0x9a53e479, 0xb6f84565, 0xd28e49bc, 0x4bfb9790, 0xe1ddf2da, 0xa4cb7e33, 0x62fb1341,
        0xcee4c6e8, 0xef20cada, 0x36774c01, 0xd07e9efe, 0x2bf11fb4, 0x95dbda4d, 0xae909198,
        0xeaad8e71, 0x6b93d5a0, 0xd08ed1d0, 0xafc725e0, 0x8e3c5b2f, 0x8e7594b7, 0x8ff6e2fb,
        0xf2122b64, 0x8888b812, 0x900df01c, 0x4fad5ea0, 0x688fc31c, 0xd1cff191, 0xb3a8c1ad,
        0x2f2f2218, 0xbe0e1777, 0xea752dfe, 0x8b021fa1, 0xe5a0cc0f, 0xb56f74e8, 0x18acf3d6,
        0xce89e299, 0xb4a84fe0, 0xfd13e0b7, 0x7cc43b81, 0xd2ada8d9, 0x165fa266, 0x80957705,
        0x93cc7314, 0x211a1477, 0xe6ad2065, 0x77b5fa86, 0xc75442f5, 0xfb9d35cf, 0xebcdaf0c,
        0x7b3e89a0, 0xd6411bd3, 0xae1e7e49, 0x00250e2d, 0x2071b35e, 0x226800bb, 0x57b8e0af,
        0x2464369b, 0xf009b91e, 0x5563911d, 0x59dfa6aa, 0x78c14389, 0xd95a537f, 0x207d5ba2,
        0x02e5b9c5, 0x83260376, 0x6295cfa9, 0x11c81968, 0x4e734a41, 0xb3472dca, 0x7b14a94a,
        0x1b510052, 0x9a532915, 0xd60f573f, 0xbc9bc6e4, 0x2b60a476, 0x81e67400, 0x08ba6fb5,
        0x571be91f, 0xf296ec6b, 0x2a0dd915, 0xb6636521, 0xe7b9f9b6, 0xff34052e, 0xc5855664,
        0x53b02d5d, 0xa99f8fa1, 0x08ba4799, 0x6e85076a,
    ],
    [
        0x4b7a70e9, 0xb5b32944, 0xdb75092e, 0xc4192623, 0xad6ea6b0, 0x49a7df7d, 0x9cee60b8,
        0x8fedb266, 0xecaa8c71, 0x699a17ff, 0x5664526c, 0xc2b19ee1, 0x193602a5, 0x75094c29,
        0xa0591340, 0xe4183a3e, 0x3f54989a, 0x5b429d65, 0x6b8fe4d6, 0x99f73fd6, 0xa1d29c07,
        0xefe830f5, 0x4d2d38e6, 0xf0255dc1, 0x4cdd2086, 0x8470eb26, 0x6382e9c6, 0x021ecc5e,
        0x09686b3f, 0x3ebaefc9, 0x3c971814, 0x6b6a70a1, 0x687f3584, 0x52a0e286, 0xb79c5305,
        0xaa500737, 0x3e07841c, 0x7fdeae5c, 0x8e7d44ec, 0x5716f2b8, 0xb03ada37, 0xf0500c0d,
        0xf01c1f04, 0x0200b3ff, 0xae0cf51a, 0x3cb574b2, 0x25837a58, 0xdc0921bd, 0xd19113f9,
        0x7ca92ff6, 0x94324773, 0x22f54701, 0x3ae5e581, 0x37c2dadc, 0xc8b57634, 0x9af3dda7,
        0xa9446146, 0x0fd0030e, 0xecc8c73e, 0xa4751e41, 0xe238cd99, 0x3bea0e2f, 0x3280bba1,
        0x183eb331, 0x4e548b38, 0x4f6db908, 0x6f420d03, 0xf60a04bf, 0x2cb81290, 0x24977c79,
        0x5679b072, 0xbcaf89af, 0xde9a771f, 0xd9930810, 0xb38bae12, 0xdccf3f2e, 0x5512721f,
        0x2e6b7124, 0x501adde6, 0x9f84cd87, 0x7a584718, 0x7408da17, 0xbc9f9abc, 0xe94b7d8c,
        0xec7aec3a, 0xdb851dfa, 0x63094366, 0xc464c3d2, 0xef1c1847, 0x3215d908, 0xdd433b37,
        0x24c2ba16, 0x12a14d43, 0x2a65c451, 0x50940002, 0x133ae4dd, 0x71dff89e, 0x10314e55,
        0x81ac77d6, 0x5f11199b, 0x043556f1, 0xd7a3c76b, 0x3c11183b, 0x5924a509, 0xf28fe6ed,
        0x97f1fbfa, 0x9ebabf2c, 0x1e153c6e, 0x86e34570, 0xeae96fb1, 0x860e5e0a, 0x5a3e2ab3,
        0x771fe71c, 0x4e3d06fa, 0x2965dcb9, 0x99e71d0f, 0x803e89d6, 0x5266c825, 0x2e4cc978,
        0x9c10b36a, 0xc6150eba, 0x94e2ea78, 0xa5fc3c53, 0x1e0a2df4, 0xf2f74ea7, 0x361d2b3d,
        0x1939260f, 0x19c27960, 0x5223a708, 0xf71312b6, 0xebadfe6e, 0xeac31f66, 0xe3bc4595,
        0xa67bc883, 0xb17f37d1, 0x018cff28, 0xc332ddef, 0xbe6c5aa5, 0x65582185, 0x68ab9802,
        0xeecea50f, 0xdb2f953b, 0x2aef7dad, 0x5b6e2f84, 0x1521b628, 0x29076170, 0xecdd4775,
        0x619f1510, 0x13cca830, 0xeb61bd96, 0x0334fe1e, 0xaa0363cf, 0xb5735c90, 0x4c70a239,
        0xd59e9e0b, 0xcbaade14, 0xeecc86bc, 0x60622ca7, 0x9cab5cab, 0xb2f3846e, 0x648b1eaf,
        0x19bdf0ca, 0xa02369b9, 0x655abb50, 0x40685a32, 0x3c2ab4b3, 0x319ee9d5, 0xc021b8f7,
        0x9b540b19, 0x875fa099, 0x95f7997e, 0x623d7da8, 0xf837889a, 0x97e32d77, 0x11ed935f,
        0x16681281, 0x0e358829, 0xc7e61fd6, 0x96dedfa1, 0x7858ba99, 0x57f584a5, 0x1b227263,
        0x9b83c3ff, 0x1ac24696, 0xcdb30aeb, 0x532e3054, 0x8fd948e4, 0x6dbc3128, 0x58ebf2ef,
        0x34c6ffea, 0xfe28ed61, 0xee7c3c73, 0x5d4a14d9, 0xe864b7e3, 0x42105d14, 0x203e13e0,
        0x45eee2b6, 0xa3aaabea, 0xdb6c4f15, 0xfacb4fd0, 0xc742f442, 0xef6abbb5, 0x654f3b1d,
        0x41cd2105, 0xd81e799e, 0x86854dc7, 0xe44b476a, 0x3d816250, 0xcf62a1f2, 0x5b8d2646,
        0xfc8883a0, 0xc1c7b6a3, 0x7f1524c3, 0x69cb7492, 0x47848a0b, 0x5692b285, 0x095bbf00,
        0xad19489d, 0x1462b174, 0x23820e00, 0x58428d2a, 0x0c55f5ea, 0x1dadf43e, 0x233f7061,
        0x3372f092, 0x8d937e41, 0xd65fecf1, 0x6c223bdb, 0x7cde3759, 0xcbee7460, 0x4085f2a7,
        0xce77326e, 0xa6078084, 0x19f8509e, 0xe8efd855, 0x61d99735, 0xa969a7aa, 0xc50c06c2,
        0x5a04abfc, 0x800bcadc, 0x9e447a2e, 0xc3453484, 0xfdd56705, 0x0e1e9ec9, 0xdb73dbd3,
        0x105588cd, 0x675fda79, 0xe3674340, 0xc5c43465, 0x713e38d8, 0x3d28f89e, 0xf16dff20,
        0x153e21e7, 0x8fb03d4a, 0xe6e39f2b, 0xdb83adf7,
    ],
    [
        0xe93d5a68, 0x948140f7, 0xf64c261c, 0x94692934, 0x411520f7, 0x7602d4f7, 0xbcf46b2e,
        0xd4a20068, 0xd4082471, 0x3320f46a, 0x43b7d4b7, 0x500061af, 0x1e39f62e, 0x97244546,
        0x14214f74, 0xbf8b8840, 0x4d95fc1d, 0x96b591af, 0x70f4ddd3, 0x66a02f45, 0xbfbc09ec,
        0x03bd9785, 0x7fac6dd0, 0x31cb8504, 0x96eb27b3, 0x55fd3941, 0xda2547e6, 0xabca0a9a,
        0x28507825, 0x530429f4, 0x0a2c86da, 0xe9b66dfb, 0x68dc1462, 0xd7486900, 0x680ec0a4,
        0x27a18dee, 0x4f3ffea2, 0xe887ad8c, 0xb58ce006, 0x7af4d6b6, 0xaace1e7c, 0xd3375fec,
        0xce78a399, 0x406b2a42, 0x20fe9e35, 0xd9f385b9, 0xee39d7ab, 0x3b124e8b, 0x1dc9faf7,
        0x4b6d1856, 0x26a36631, 0xeae397b2, 0x3a6efa74, 0xdd5b4332, 0x6841e7f7, 0xca7820fb,
        0xfb0af54e, 0xd8feb397, 0x454056ac, 0xba489527, 0x55533a3a, 0x20838d87, 0xfe6ba9b7,
        0xd096954b, 0x55a867bc, 0xa1159a58, 0xcca92963, 0x99e1db33, 0xa62a4a56, 0x3f3125f9,
        0x5ef47e1c, 0x9029317c, 0xfdf8e802, 0x04272f70, 0x80bb155c, 0x05282ce3, 0x95c11548,
        0xe4c66d22, 0x48c1133f, 0xc70f86dc, 0x07f9c9ee, 0x41041f0f, 0x404779a4, 0x5d886e17,
        0x325f51eb, 0xd59bc0d1, 0xf2bcc18f, 0x41113564, 0x257b7834, 0x602a9c60, 0xdff8e8a3,
        0x1f636c1b, 0x0e12b4c2, 0x02e1329e, 0xaf664fd1, 0xcad18115, 0x6b2395e0, 0x333e92e1,
        0x3b240b62, 0xeebeb922, 0x85b2a20e, 0xe6ba0d99, 0xde720c8c, 0x2da2f728, 0xd0127845,
        0x95b794fd, 0x647d0862, 0xe7ccf5f0, 0x5449a36f, 0x877d48fa, 0xc39dfd27, 0xf33e8d1e,
        0x0a476341, 0x992eff74, 0x3a6f6eab, 0xf4f8fd37, 0xa812dc60, 0xa1ebddf8, 0x991be14c,
        0xdb6e6b0d, 0xc67b5510, 0x6d672c37, 0x2765d43b, 0xdcd0e804, 0xf1290dc7, 0xcc00ffa3,
        0xb5390f92, 0x690fed0b, 0x667b9ffb, 0xcedb7d9c, 0xa091cf0b, 0xd9155ea3, 0xbb132f88,
        0x515bad24, 0x7b9479bf, 0x763bd6eb, 0x37392eb3, 0xcc115979, 0x8026e297, 0xf42e312d,
        0x6842ada7, 0xc66a2b3b, 0x12754ccc, 0x782ef11c, 0x6a124237, 0xb79251e7, 0x06a1bbe6,
        0x4bfb6350, 0x1a6b1018, 0x11caedfa, 0x3d25bdd8, 0xe2e1c3c9, 0x44421659, 0x0a121386,
        0xd90cec6e, 0xd5abea2a, 0x64af674e, 0xda86a85f, 0xbebfe988, 0x64e4c3fe, 0x9dbc8057,
        0xf0f7c086, 0x60787bf8, 0x6003604d, 0xd1fd8346, 0xf6381fb0, 0x7745ae04, 0xd736fccc,
        0x83426b33, 0xf01eab71, 0xb0804187, 0x3c005e5f, 0x77a057be, 0xbde8ae24, 0x55464299,
        0xbf582e61, 0x4e58f48f, 0xf2ddfda2, 0xf474ef38, 0x8789bdc2, 0x5366f9c3, 0xc8b38e74,
        0xb475f255, 0x46fcd9b9, 0x7aeb2661, 0x8b1ddf84, 0x846a0e79, 0x915f95e2, 0x466e598e,
        0x20b45770, 0x8cd55591, 0xc902de4c, 0xb90bace1, 0xbb8205d0, 0x11a86248, 0x7574a99e,
        0xb77f19b6, 0xe0a9dc09, 0x662d09a1, 0xc4324633, 0xe85a1f02, 0x09f0be8c, 0x4a99a025,
        0x1d6efe10, 0x1ab93d1d, 0x0ba5a4df, 0xa186f20f, 0x2868f169, 0xdcb7da83, 0x573906fe,
        0xa1e2ce9b, 0x4fcd7f52, 0x50115e01, 0xa70683fa, 0xa002b5c4, 0x0de6d027, 0x9af88c27,
        0x773f8641, 0xc3604c06, 0x61a806b5, 0xf0177a28, 0xc0f586e0, 0x006058aa, 0x30dc7d62,
        0x11e69ed7, 0x2338ea63, 0x53c2dd94, 0xc2c21634, 0xbbcbee56, 0x90bcb6de, 0xebfc7da1,
        0xce591d76, 0x6f05e409, 0x4b7c0188, 0x39720a3d, 0x7c927c24, 0x86e3725f, 0x724d9db9,
        0x1ac15bb4, 0xd39eb8fc, 0xed545578, 0x08fca5b5, 0xd83d7cd3, 0x4dad0fc4, 0x1e50ef5e,
        0xb161e6f8, 0xa28514d9, 0x6c51133c, 0x6fd5c7e7, 0x56e14ec4, 0x362abfce, 0xddc6c837,
        0xd79a3234, 0x92638212, 0x670efa8e, 0x406000e0,
    ],
    [
        0x3a39ce37, 0xd3faf5cf, 0xabc27737, 0x5ac52d1b, 0x5cb0679e, 0x4fa33742, 0xd3822740,
        0x99bc9bbe, 0xd5118e9d, 0xbf0f7315, 0xd62d1c7e, 0xc700c47b, 0xb78c1b6b, 0x21a19045,
        0xb26eb1be, 0x6a366eb4, 0x5748ab2f, 0xbc946e79, 0xc6a376d2, 0x6549c2c8, 0x530ff8ee,
        0x468dde7d, 0xd5730a1d, 0x4cd04dc6, 0x2939bbdb, 0xa9ba4650, 0xac9526e8, 0xbe5ee304,
        0xa1fad5f0, 0x6a2d519a, 0x63ef8ce2, 0x9a86ee22, 0xc089c2b8, 0x43242ef6, 0xa51e03aa,
        0x9cf2d0a4, 0x83c061ba, 0x9be96a4d, 0x8fe51550, 0xba645bd6, 0x2826a2f9, 0xa73a3ae1,
        0x4ba99586, 0xef5562e9, 0xc72fefd3, 0xf752f7da, 0x3f046f69, 0x77fa0a59, 0x80e4a915,
        0x87b08601, 0x9b09e6ad, 0x3b3ee593, 0xe990fd5a, 0x9e34d797, 0x2cf0b7d9, 0x022b8b51,
        0x96d5ac3a, 0x017da67d, 0xd1cf3ed6, 0x7c7d2d28, 0x1f9f25cf, 0xadf2b89b, 0x5ad6b472,
        0x5a88f54c, 0xe029ac71, 0xe019a5e6, 0x47b0acfd, 0xed93fa9b, 0xe8d3c48d, 0x283b57cc,
        0xf8d56629, 0x79132e28, 0x785f0191, 0xed756055, 0xf7960e44, 0xe3d35e8c, 0x15056dd4,
        0x88f46dba, 0x03a16125, 0x0564f0bd, 0xc3eb9e15, 0x3c9057a2, 0x97271aec, 0xa93a072a,
        0x1b3f6d9b, 0x1e6321f5, 0xf59c66fb, 0x26dcf319, 0x7533d928, 0xb155fdf5, 0x03563482,
        0x8aba3cbb, 0x28517711, 0xc20ad9f8, 0xabcc5167, 0xccad925f, 0x4de81751, 0x3830dc8e,
        0x379d5862, 0x9320f991, 0xea7a90c2, 0xfb3e7bce, 0x5121ce64, 0x774fbe32, 0xa8b6e37e,
        0xc3293d46, 0x48de5369, 0x6413e680, 0xa2ae0810, 0xdd6db224, 0x69852dfd, 0x09072166,
        0xb39a460a, 0x6445c0dd, 0x586cdecf, 0x1c20c8ae, 0x5bbef7dd, 0x1b588d40, 0xccd2017f,
        0x6bb4e3bb, 0xdda26a7e, 0x3a59ff45, 0x3e350a44, 0xbcb4cdd5, 0x72eacea8, 0xfa6484bb,
        0x8d6612ae, 0xbf3c6f47, 0xd29be463, 0x542f5d9e, 0xaec2771b, 0xf64e6370, 0x740e0d8d,
        0xe75b1357, 0xf8721671, 0xaf537d5d, 0x4040cb08, 0x4eb4e2cc, 0x34d2466a, 0x0115af84,
        0xe1b00428, 0x95983a1d, 0x06b89fb4, 0xce6ea048, 0x6f3f3b82, 0x3520ab82, 0x011a1d4b,
        0x277227f8, 0x611560b1, 0xe7933fdc, 0xbb3a792b, 0x344525bd, 0xa08839e1, 0x51ce794b,
        0x2f32c9b7, 0xa01fbac9, 0xe01cc87e, 0xbcc7d1f6, 0xcf0111c3, 0xa1e8aac7, 0x1a908749,
        0xd44fbd9a, 0xd0dadecb, 0xd50ada38, 0x0339c32a, 0xc6913667, 0x8df9317c, 0xe0b12b4f,
        0xf79e59b7, 0x43f5bb3a, 0xf2d519ff, 0x27d9459c, 0xbf97222c, 0x15e6fc2a, 0x0f91fc71,
        0x9b941525, 0xfae59361, 0xceb69ceb, 0xc2a86459, 0x12baa8d1, 0xb6c1075e, 0xe3056a0c,
        0x10d25065, 0xcb03a442, 0xe0ec6e0e, 0x1698db3b, 0x4c98a0be, 0x3278e964, 0x9f1f9532,
        0xe0d392df, 0xd3a0342b, 0x8971f21e, 0x1b0a7441, 0x4ba3348c, 0xc5be7120, 0xc37632d8,
        0xdf359f8d, 0x9b992f2e, 0xe60b6f47, 0x0fe3f11d, 0xe54cda54, 0x1edad891, 0xce6279cf,
        0xcd3e7e6f, 0x1618b166, 0xfd2c1d05, 0x848fd2c5, 0xf6fb2299, 0xf523f357, 0xa6327623,
        0x93a83531, 0x56cccd02, 0xacf08162, 0x5a75ebb5, 0x6e163697, 0x88d273cc, 0xde966292,
        0x81b949d0, 0x4c50901b, 0x71c65614, 0xe6c6c7bd, 0x327a140a, 0x45e1d006, 0xc3f27b9a,
        0xc9aa53fd, 0x62a80f00, 0xbb25bfe2, 0x35bdd2f6, 0x71126905, 0xb2040222, 0xb6cbcf7c,
        0xcd769c2b, 0x53113ec0, 0x1640e3d3, 0x38abbd60, 0x2547adf0, 0xba38209c, 0xf746ce76,
        0x77afa1c5, 0x20756060, 0x85cbfe4e, 0x8ae88dd8, 0x7aaaf9b0, 0x4cf9aa7e, 0x1948c25c,
        0x02fb8a8c, 0x01c36ae4, 0xd6ebe1f9, 0x90d4f869, 0xa65cdea0, 0x3f09252d, 0xc208e69f,
        0xb74e6132, 0xce77e25b, 0x578fdfe3, 0x3ac372e6,
    ],
];

#[cfg(test)]
// Criterion imports this module with cfg(test), but its own main does not run
// the Rust unit-test harness, so the test-only fixtures and helpers are unused.
#[allow(dead_code)]
mod tests {
    use crate::encryption::{blowfish_decrypt, blowfish_encrypt};

    // Eric Young's published 16-round Blowfish vectors, preserved verbatim.
    // https://www.schneier.com/wp-content/uploads/2015/12/vectors-2.txt
    const BLOCK_VECTORS: [(&str, &str, &str); 34] = [
        ("0000000000000000", "0000000000000000", "4EF997456198DD78"),
        ("FFFFFFFFFFFFFFFF", "FFFFFFFFFFFFFFFF", "51866FD5B85ECB8A"),
        ("3000000000000000", "1000000000000001", "7D856F9A613063F2"),
        ("1111111111111111", "1111111111111111", "2466DD878B963C9D"),
        ("0123456789ABCDEF", "1111111111111111", "61F9C3802281B096"),
        ("1111111111111111", "0123456789ABCDEF", "7D0CC630AFDA1EC7"),
        ("0000000000000000", "0000000000000000", "4EF997456198DD78"),
        ("FEDCBA9876543210", "0123456789ABCDEF", "0ACEAB0FC6A0A28D"),
        ("7CA110454A1A6E57", "01A1D6D039776742", "59C68245EB05282B"),
        ("0131D9619DC1376E", "5CD54CA83DEF57DA", "B1B8CC0B250F09A0"),
        ("07A1133E4A0B2686", "0248D43806F67172", "1730E5778BEA1DA4"),
        ("3849674C2602319E", "51454B582DDF440A", "A25E7856CF2651EB"),
        ("04B915BA43FEB5B6", "42FD443059577FA2", "353882B109CE8F1A"),
        ("0113B970FD34F2CE", "059B5E0851CF143A", "48F4D0884C379918"),
        ("0170F175468FB5E6", "0756D8E0774761D2", "432193B78951FC98"),
        ("43297FAD38E373FE", "762514B829BF486A", "13F04154D69D1AE5"),
        ("07A7137045DA2A16", "3BDD119049372802", "2EEDDA93FFD39C79"),
        ("04689104C2FD3B2F", "26955F6835AF609A", "D887E0393C2DA6E3"),
        ("37D06BB516CB7546", "164D5E404F275232", "5F99D04F5B163969"),
        ("1F08260D1AC2465E", "6B056E18759F5CCA", "4A057A3B24D3977B"),
        ("584023641ABA6176", "004BD6EF09176062", "452031C1E4FADA8E"),
        ("025816164629B007", "480D39006EE762F2", "7555AE39F59B87BD"),
        ("49793EBC79B3258F", "437540C8698F3CFA", "53C55F9CB49FC019"),
        ("4FB05E1515AB73A7", "072D43A077075292", "7A8E7BFA937E89A3"),
        ("49E95D6D4CA229BF", "02FE55778117F12A", "CF9C5D7A4986ADB5"),
        ("018310DC409B26D6", "1D9D5C5018F728C2", "D1ABB290658BC778"),
        ("1C587F1C13924FEF", "305532286D6F295A", "55CB3774D13EF201"),
        ("0101010101010101", "0123456789ABCDEF", "FA34EC4847B268B2"),
        ("1F1F1F1F0E0E0E0E", "0123456789ABCDEF", "A790795108EA3CAE"),
        ("E0FEE0FEF1FEF1FE", "0123456789ABCDEF", "C39E072D9FAC631D"),
        ("0000000000000000", "FFFFFFFFFFFFFFFF", "014933E0CDAFF6E4"),
        ("FFFFFFFFFFFFFFFF", "0000000000000000", "F21E9A77B71C49BC"),
        ("0123456789ABCDEF", "0000000000000000", "245946885754369A"),
        ("FEDCBA9876543210", "FFFFFFFFFFFFFFFF", "6B5C5A9C5D9E0A5A"),
    ];

    // The public API accepts keys from 4 to 56 bytes. The published 1-, 2-,
    // and 3-byte extension cases are outside that contract; their lengths are
    // covered by the invalid-key test. No published ciphertext is changed.
    const KEY_SCHEDULE_VECTORS: [(&str, &str); 21] = [
        ("F0E1D2C3", "BE1E639408640F05"),
        ("F0E1D2C3B4", "B39E44481BDB1E6E"),
        ("F0E1D2C3B4A5", "9457AA83B1928C0D"),
        ("F0E1D2C3B4A596", "8BB77032F960629D"),
        ("F0E1D2C3B4A59687", "E87A244E2CC85E82"),
        ("F0E1D2C3B4A5968778", "15750E7A4F4EC577"),
        ("F0E1D2C3B4A596877869", "122BA70B3AB64AE0"),
        ("F0E1D2C3B4A5968778695A", "3A833C9AFFC537F6"),
        ("F0E1D2C3B4A5968778695A4B", "9409DA87A90F6BF2"),
        ("F0E1D2C3B4A5968778695A4B3C", "884F80625060B8B4"),
        ("F0E1D2C3B4A5968778695A4B3C2D", "1F85031C19E11968"),
        ("F0E1D2C3B4A5968778695A4B3C2D1E", "79D9373A714CA34F"),
        ("F0E1D2C3B4A5968778695A4B3C2D1E0F", "93142887EE3BE15C"),
        ("F0E1D2C3B4A5968778695A4B3C2D1E0F00", "03429E838CE2D14B"),
        ("F0E1D2C3B4A5968778695A4B3C2D1E0F0011", "A4299E27469FF67B"),
        ("F0E1D2C3B4A5968778695A4B3C2D1E0F001122", "AFD5AED1C1BC96A8"),
        (
            "F0E1D2C3B4A5968778695A4B3C2D1E0F00112233",
            "10851C0E3858DA9F",
        ),
        (
            "F0E1D2C3B4A5968778695A4B3C2D1E0F0011223344",
            "E6F51ED79B9DB21F",
        ),
        (
            "F0E1D2C3B4A5968778695A4B3C2D1E0F001122334455",
            "64A6E14AFD36B46F",
        ),
        (
            "F0E1D2C3B4A5968778695A4B3C2D1E0F00112233445566",
            "80C7D7D45A5479AD",
        ),
        (
            "F0E1D2C3B4A5968778695A4B3C2D1E0F0011223344556677",
            "05044B62FA52D080",
        ),
    ];

    // Additional fixed vectors generated independently with OpenSSL 3.6.4
    // BF_set_key / BF_ecb_encrypt (16 rounds, encryption, no padding).
    // No expected value is generated by the implementation under test, and
    // running these tests does not require OpenSSL or another dependency.
    // The base block is 00017F80FEFFAA55; base key byte i is 0x80 + i.
    const REFERENCE_BLOCK: [u8; 8] = [0x00, 0x01, 0x7f, 0x80, 0xfe, 0xff, 0xaa, 0x55];

    // Entry i uses the first i + 4 bytes of the base key (4 through 56).
    const KEY_LENGTH_CIPHERTEXTS: [&str; 53] = [
        "AB30F50DC293E37A",
        "AA2A82EB022FE8EE",
        "5D73433BC90B6944",
        "8539EC687D7DAB4B",
        "4567DFBD0B796D44",
        "F5A6C560FC613082",
        "C483003D76898C0D",
        "6CEE381799B38B99",
        "5706BDA0E85E9791",
        "6491A6D4EED28ED9",
        "221CCB024A258389",
        "F11F523A58F330D9",
        "A7E067DA7F507110",
        "157652103696267B",
        "26F51C18E6D701C6",
        "A30575BF83AAEC1F",
        "B905BED2F33218EF",
        "C1BFE2251E5ECAAC",
        "CA6D2E00DB258617",
        "AC7B2832A64226C8",
        "DDFADED8AB2151A0",
        "F9DA5E4DCA9DE5E1",
        "3460ED3C39B2E2DF",
        "85775ECCC145C87D",
        "663F616E31D5A839",
        "153E35FB879A0530",
        "9E73389E7650097A",
        "8F7882B6C98EF2AB",
        "4831352342877EC3",
        "0A30D975D6E89A5E",
        "FD00D7F19349A076",
        "A1AA96AFE3ED18A5",
        "852418D2CCCEE229",
        "5B9504E504050068",
        "A68D01F56B1B75CC",
        "B78ED8EF4E010464",
        "984D65DB3666EB19",
        "9AFEAEAA9AE985D2",
        "F90B16C6E82B86CD",
        "6D5786E62B383179",
        "CFFE5AA9EFF1AEA7",
        "FF5A9664311088EC",
        "09B5E4BE43648D85",
        "6B7D77A2F498F62A",
        "6AE01123689394AA",
        "BC485091B936CB22",
        "7E2BA9DB737BE373",
        "17EB4EF49B3353FC",
        "DE7BD6A78A42DD83",
        "5038D77CE9E3953B",
        "3DFAE83AFDD08146",
        "FE5C6E588B154177",
        "EA0EFD0ECD022EED",
    ];

    // Entry i flips bit 0 of byte i in the full 56-byte base key.
    const KEY_BYTE_CIPHERTEXTS: [&str; 56] = [
        "366FAACEC3038975",
        "47120C168295AD29",
        "BA47317B01A80978",
        "E5E35BD347CB037E",
        "957E9C24A600A292",
        "B708061CCBA2F96B",
        "982FA0164D57D25E",
        "DA4BBA723AF75B66",
        "D1C20BFF91FB031D",
        "7D3F8B41CC1A3350",
        "42E05E6D97A3EB27",
        "4554F885B394074A",
        "C8AB722EC978E984",
        "AD1A5D73134F7186",
        "9C8B43A00CA72645",
        "2134D600CE7C6519",
        "FEADB0F5D6500CC0",
        "A97AA6183A16153D",
        "56EBB942EB53233C",
        "F69615A201FC904D",
        "D57FADD9D85AD2B2",
        "9756D0499C8381AE",
        "286D793B3C47E5C0",
        "C04620BE0748CAAD",
        "291E6B81C0E03269",
        "7B903A923E4296F6",
        "CFB38B490E0A4422",
        "60BDFB4C28F2CFEF",
        "B2720CDBB8289D48",
        "27BCE8EBA415BD21",
        "0087D2BF322C6303",
        "F23A20EA46705C29",
        "E60DC2C557878159",
        "C7FFD5CE3AD48F2D",
        "4C0F226BEA2E09A2",
        "E0A9857DF62E016E",
        "0572F7D16EB4DBEA",
        "4292C0B03F5B2651",
        "E26DFB63C111F3ED",
        "06541B3D9666C6D6",
        "8861FA812227A29C",
        "88FBCF9FE6A76C59",
        "EF15376CF6C22EB9",
        "47AB9DD504965895",
        "DCACD2C5D7205818",
        "1849B06E86E1AE76",
        "3E5C17356ECCCCFC",
        "7F8DE9367DA0E1A3",
        "A89018E0239B61AC",
        "53F5F3C78FA7FA65",
        "BF3E12E61A43A2D1",
        "94B3B1160CBBD6A6",
        "592841BA963E0503",
        "324D7C7200D90BD7",
        "C117CF5B737C0FD0",
        "4FEF3342E0AC7D09",
    ];

    // Entry v uses block byte i = (v + 17 * i) mod 256 and the full base key.
    const BYTE_VALUE_CIPHERTEXTS: [&str; 256] = [
        "D510F349EB7B2D6D",
        "454524FDC69A60CE",
        "E503474ED036112A",
        "8F841762CEDC1E36",
        "29F8D4A484CB2DA4",
        "380BA061417E0F54",
        "8839FF9F862F147B",
        "43B5F6F8A4F68F14",
        "2715010442AD526C",
        "7FC941FDEFCAAE0B",
        "3C2B2E37F60AF9BD",
        "E5DE90A8DB276864",
        "2A9E3EC994E48615",
        "5CE1416694DE450F",
        "728EB681284D93A5",
        "D377E773977115B5",
        "F37103174B7A57CD",
        "488435AB65D2071C",
        "0230A1961BE22067",
        "6B57B50532CD874E",
        "CA38126E3A9604F8",
        "C0CA075ADC2BA771",
        "9C0C2233FA93FE7F",
        "9871D3766E904696",
        "D597E3F831382FC3",
        "15EC84E44A2B52E7",
        "1925035B046BC60B",
        "338B938FB93183AE",
        "E0DB16F227A7E52B",
        "14FE37DB5028C35C",
        "E28112878FB34DBA",
        "95755FBB4D47CADB",
        "A23005B5FF8DF2F8",
        "C67C8D4BF6EC65BC",
        "4568E1D56FC4872F",
        "DD343E5D285D3C56",
        "DF1BD719DB9D3FC1",
        "BCD7F8B58070886F",
        "636FBF9B5964C174",
        "2A183E08761F8C97",
        "5CF6E0B5305B48AC",
        "C192E5C50E46E4E4",
        "AE346785F79E11CD",
        "BA00EA47545794E3",
        "DD9588CB5DBA53E2",
        "4AEF151FFE3BC8D4",
        "BF33CE28A914A2CC",
        "F40CB0208D331D8F",
        "3F865E5AD1142E9A",
        "ADC94D9212EFAE92",
        "050BBF65118780E2",
        "3A73DE5430003A01",
        "07C2EF541E2B1B3B",
        "3B481DF60E28FE91",
        "42289BD4ABEBD582",
        "A452223C73B3D0BF",
        "87A03165860C7F08",
        "9A58501A01F8D844",
        "42F37C76ECC6BCF6",
        "8B6F0BDF1BFC717C",
        "DAE778435DF83815",
        "F5AC0FD4B84F570B",
        "03FAD15DB7859ADF",
        "190368FAC7E4E944",
        "6B5A3A41F04696C4",
        "1CBCEA14D30B0EED",
        "94D8B0207F4F527E",
        "569622085BA47398",
        "CFF098BF610BA225",
        "95A55AC538FCD6C9",
        "C7B525D099A86915",
        "0B28A49DFBC59729",
        "6EE7070173DAB799",
        "82132C74DC47CA11",
        "5C046AF54F8553A1",
        "87F5A47ECFE25F17",
        "95F578E4703C0E1C",
        "3753C64E1548BA67",
        "86AD5933FB00B749",
        "FC32037430B2A8B3",
        "CFDC7741F426E061",
        "1B4E52780B92CA78",
        "153D3D8BA84D7471",
        "0F4E273C69FFC5F2",
        "1BC56D7878DF8A57",
        "501B334280FEE901",
        "DF1A38DD5F00BC78",
        "3E667799388BD90F",
        "10FD990A6C559E13",
        "8F4427D9A856FA62",
        "64F001000C41C716",
        "69EDEF2C000710E6",
        "AADDF65A98A90CCC",
        "E8004206B2E5C3E8",
        "73CB4BA80CC29675",
        "9B8BB4352F65B11C",
        "961C0F8B195F984B",
        "165FE7C2D7C65C0C",
        "58B996A9601FFADB",
        "83DB237175017DB3",
        "94D0386F4EF079C2",
        "C70F4A21A63C222E",
        "F6C025C6E7C9F645",
        "C6F2F63A12238847",
        "475A6066F5B1DD01",
        "9CB0E4DF4C9604E3",
        "0EAA7E23D994C968",
        "CA99BA6462915705",
        "AFF8C966530826C4",
        "39A1BDDAD563FEDE",
        "6326A4763C5CC350",
        "02B9885011A2890C",
        "565546EA1706EBB7",
        "FC3475F02163ACCB",
        "8D48D7EEA1BC6F13",
        "5DF7E8B274876C27",
        "8EC21F95432A26EA",
        "DDA35EFC8BA7941E",
        "42DACADFA2122627",
        "E12822BA582A19E7",
        "DB388D7805DDC868",
        "3F0799B002DAA4C7",
        "A747C5D5ED8B2979",
        "E82133B8682D2CB2",
        "051200BBFB4CED5C",
        "E26E2B99AD663ABE",
        "178138712FB8816D",
        "0DE23AE034C1FE59",
        "D61408502E161264",
        "1444455EB1ED348F",
        "C492BAC9AF1895EC",
        "9703E339871A1594",
        "9F55622FEE720796",
        "C51EB94BD83AC82F",
        "9DA53565344FD3F8",
        "5817CFBBAE574CBA",
        "BBA4F4EAE80CA5E3",
        "3CE6F5B0F60CA816",
        "A871C9FB47A72FD0",
        "00D0C06D012A21AD",
        "9C1EAADC86D8A7B8",
        "1E0442D017AE7F17",
        "A2AD195EE1231DBB",
        "E85BBEB35288EE3F",
        "D8F383FD06DC6194",
        "1A5B82BFA31F08E6",
        "D614B7FE084CFA63",
        "A2483853B6312A61",
        "D73673EA36A4327D",
        "3E76E4A0F6A20C31",
        "58C71726B7E86EA2",
        "A56C9160CE235916",
        "93FA4EB5D5D441C9",
        "38CC0AF2CABCD487",
        "B7D30B966B55D607",
        "DC49603D87862387",
        "E56143025DCAEE48",
        "0BA1A1F55E1C9B40",
        "E80E2B86F04F8B48",
        "B47E1DB495DFAF55",
        "4DACC14A79AA612E",
        "F82E9D00C8EFDAAF",
        "5CA3944C2B461E5F",
        "9612FAD794CD64DA",
        "FC6FD525C713F6EE",
        "0C23F005A1B9FCF5",
        "650B02B7EA3F64FA",
        "4FD447E6CB0EDE2C",
        "BA14A3E67BCF9830",
        "FF75E3B1B920780D",
        "D27E2A1866EC76DF",
        "F0E637DB17010464",
        "5D62B049E4CD00D7",
        "732E321DFA0043BF",
        "0CDB01C503C07CE8",
        "4239C441237FE6FD",
        "1B75807F1780088B",
        "941B405A5FC9ABEA",
        "6B7083F0D18E59F6",
        "6540E709C4D4FF52",
        "1D4E904B2545E42A",
        "689D70366AE671D2",
        "0054BA0E0F499958",
        "FAC5E5A0C76017AC",
        "6AAD3420AF8754EA",
        "47C9E3215C571084",
        "51D2B38B19CC2BD3",
        "B5064E2D42DC2C15",
        "DD5A838052C73D20",
        "F7997944FDF6A1ED",
        "DC9E9D489B4FA96A",
        "ECB16CA6B8EA6F58",
        "FAD0D66A3A4DD87A",
        "D3775430CAA8710B",
        "150A3FCB2DE37152",
        "3A2FF74EAEDF6EB8",
        "45F1FD8921BB77E6",
        "407185283635F25C",
        "BF64344DFD8FD016",
        "4C6BDFE83577C218",
        "EA115C9A9C97E825",
        "4E9FC797BD3D4389",
        "B201DC91E01626C4",
        "7802C4870C1481DB",
        "183545FDC17BB6BB",
        "B9EBF2821AD1DFA5",
        "62302FA77DA06F17",
        "5D8A079115D1742A",
        "182924ADCD426BB1",
        "5E2B34961684D746",
        "6DAD5DCF8234E826",
        "C7F2F324792D2EC4",
        "1A36B9FC2B61F14D",
        "EE4B709B929F891A",
        "22851753D0B2B0BC",
        "81DC9DCE22784C4A",
        "F458EA7BC615F66D",
        "BD8997B0C8632CAA",
        "4C491C8114A0FF6A",
        "0E22E08850D63582",
        "1F4829F636860F0B",
        "F9609FA06A8F2104",
        "6A39C281C7B10F62",
        "623387943F82A418",
        "EB6AF9CD9D98E5C8",
        "3B832310A5DF239D",
        "C55379D2586E0534",
        "945151809D7524A8",
        "A47CD8BCD2D0BDC8",
        "1599A560A97DEBA2",
        "CCFCB353A698CBE1",
        "45CFC7C978589795",
        "9430C2B53F072F0C",
        "884E2A78657D29C2",
        "D87F342F32C9402B",
        "9BBB8CB68FE6B694",
        "BC50AA7EADDB1AEB",
        "A49F12DA66617AA0",
        "ABC559375BB1B67D",
        "6DA206D65B2AB5DB",
        "66831DB35F742DDB",
        "E52D181993C4A848",
        "FBC8DD7AC5C6B3DA",
        "BB24CC4219B4B285",
        "D2D60BC5CB66A6C8",
        "643B35AE6A9B4E28",
        "F719C52AB8A17745",
        "EFC8D5FD80910342",
        "8E5538545F084EAD",
        "2B92E77E0D83B432",
        "DFF6E5800F1ED76E",
        "51A3EEC0E7FA111B",
        "BF88A589C08FEE3D",
        "D6FE76E265F5559E",
        "AAFA422ECA653C69",
        "76B44CFF3AAA1885",
    ];

    // Zero, 0xff and alternating 0xaa/0x55 keys at lengths 4, 5, 16 and 56.
    const BINARY_KEY_VECTORS: [(&str, &str); 12] = [
        ("00000000", "C884203B2CB667A1"),
        ("FFFFFFFF", "1CAC8918A370C58E"),
        ("AA55AA55", "181E972BC6603A2A"),
        ("0000000000", "C884203B2CB667A1"),
        ("FFFFFFFFFF", "1CAC8918A370C58E"),
        ("AA55AA55AA", "B7AACB0285870387"),
        ("00000000000000000000000000000000", "C884203B2CB667A1"),
        ("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", "1CAC8918A370C58E"),
        ("AA55AA55AA55AA55AA55AA55AA55AA55", "181E972BC6603A2A"),
        (
            "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "C884203B2CB667A1",
        ),
        (
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            "1CAC8918A370C58E",
        ),
        (
            "AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55AA55",
            "181E972BC6603A2A",
        ),
    ];

    fn decode_hex(hex: &str) -> Vec<u8> {
        assert!(hex.len().is_multiple_of(2));
        (0..hex.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap())
            .collect()
    }

    fn decode_block(hex: &str) -> [u8; 8] {
        decode_hex(hex).try_into().unwrap()
    }

    fn assert_vector(block: &[u8; 8], key: &[u8], ciphertext: &str) {
        let expected = decode_block(ciphertext);
        assert_eq!(
            blowfish_encrypt(block, key),
            expected,
            "encryption: key={key:02x?}, block={block:02x?}"
        );
        assert_eq!(
            blowfish_decrypt(&expected, key),
            *block,
            "decryption: key={key:02x?}, ciphertext={expected:02x?}"
        );
    }

    #[test]
    fn test_published_block_vectors() {
        for (key, plaintext, ciphertext) in BLOCK_VECTORS {
            assert_vector(&decode_block(plaintext), &decode_hex(key), ciphertext);
        }
    }

    #[test]
    fn test_published_key_schedule_vectors() {
        let block = decode_block("FEDCBA9876543210");
        for (key, ciphertext) in KEY_SCHEDULE_VECTORS {
            assert_vector(&block, &decode_hex(key), ciphertext);
        }
    }

    #[test]
    fn test_reference_every_valid_key_length() {
        for (index, ciphertext) in KEY_LENGTH_CIPHERTEXTS.iter().enumerate() {
            let key: Vec<u8> = (0x80..).take(index + 4).collect();
            assert_vector(&REFERENCE_BLOCK, &key, ciphertext);
        }
    }

    #[test]
    fn test_reference_binary_key_patterns() {
        for (key, ciphertext) in BINARY_KEY_VECTORS {
            assert_vector(&REFERENCE_BLOCK, &decode_hex(key), ciphertext);
        }
    }

    #[test]
    fn test_reference_every_key_byte_is_used() {
        let base_key: Vec<u8> = (0x80..).take(56).collect();
        for (index, ciphertext) in KEY_BYTE_CIPHERTEXTS.iter().enumerate() {
            let mut key = base_key.clone();
            key[index] ^= 1;
            assert_vector(&REFERENCE_BLOCK, &key, ciphertext);
        }
    }

    #[test]
    fn test_reference_every_byte_value_at_every_block_position() {
        let key: Vec<u8> = (0x80..).take(56).collect();
        for (value, ciphertext) in BYTE_VALUE_CIPHERTEXTS.iter().enumerate() {
            let block = std::array::from_fn(|index| (value as u8).wrapping_add((index as u8) * 17));
            assert_vector(&block, &key, ciphertext);
        }
    }

    #[test]
    fn test_calls_are_independent_and_preserve_inputs() {
        let block = decode_block("1111111111111111");
        let original_block = block;
        let keys = [
            decode_hex("0123456789ABCDEF"),
            decode_hex("1111111111111111"),
        ];
        let original_keys = keys.clone();
        let expected = ["61F9C3802281B096", "2466DD878B963C9D"];
        for index in [0, 1, 1, 0, 1, 0] {
            assert_vector(&block, &keys[index], expected[index]);
        }
        assert_eq!(block, original_block);
        assert_eq!(keys, original_keys);
    }

    #[test]
    fn test_invalid_key_lengths_panic() {
        let operations = [blowfish_encrypt, blowfish_decrypt];
        for length in [0, 1, 2, 3, 57, 256] {
            let key = vec![0xff; length];
            for block in [[0; 8], [0xff; 8]] {
                for operation in operations {
                    let panic = std::panic::catch_unwind(|| operation(&block, &key))
                        .expect_err("invalid key lengths must panic");
                    let message = panic
                        .downcast_ref::<&str>()
                        .copied()
                        .or_else(|| panic.downcast_ref::<String>().map(String::as_str));
                    assert_eq!(message, Some("`key` must contain between 4 and 56 bytes."));
                }
            }
        }
    }

    #[test]
    fn test_documented_examples() {
        assert_vector(b"Blowfish", b"mysecretkey", "359918FE6E7A64A0");
        assert_vector(&[0; 8], &[0; 8], "4EF997456198DD78");
    }
}
