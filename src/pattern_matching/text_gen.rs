//! Gerador de texto "Lorem Ipsum" com semente fixa.

/// Quantidade de palavras da tabela.
const NUM_WORDS: usize = 19;

/// As palavras e o tamanho de cada uma.
const WORDS: [(&[u8], usize); NUM_WORDS] = [
    (b"lorem", 5),
    (b"ipsum", 5),
    (b"dolor", 5),
    (b"sit", 3),
    (b"amet", 4),
    (b"consectetur", 11),
    (b"adipiscing", 10),
    (b"elit", 4),
    (b"sed", 3),
    (b"do", 2),
    (b"eiusmod", 7),
    (b"tempor", 6),
    (b"incididunt", 10),
    (b"ut", 2),
    (b"labore", 6),
    (b"et", 2),
    (b"dolore", 6),
    (b"magna", 5),
    (b"aliqua", 6),
];

/// Preenche `buf[0..size]` com um texto de exatamente `size` bytes, feito de
/// palavras sorteadas e separadas por espaço. Quem chama garante que
/// `buf` tem pelo menos `size` posições.
///
/// O sorteio usa um gerador congruencial linear, escrito aqui (sem pacotes
/// externos). Com a mesma `seed`, o texto é sempre o mesmo. O estado fica
/// sempre abaixo de 2³¹, então a conta cabe em um `u64` sem estourar.
///
/// # Complexidade
///
/// - Tempo: O(size).
/// - Espaço extra: O(1).
pub fn fill_lorem(buf: &mut [u8], size: usize, seed: u64) {
    let mut state = seed % 2_147_483_648;
    let mut pos = 0;
    while pos < size {
        state = (state * 1_103_515_245 + 12_345) % 2_147_483_648;
        let (word, word_len) = WORDS[((state / 65_536) as usize) % NUM_WORDS];
        let mut k = 0;
        while k < word_len && pos < size {
            buf[pos] = word[k];
            pos += 1;
            k += 1;
        }
        if pos < size {
            buf[pos] = b' ';
            pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fill_lorem;

    #[test]
    fn test_fills_exactly_size_bytes() {
        let mut buf = [b'#'; 10];
        fill_lorem(&mut buf, 6, 1);
        let mut k = 0;
        while k < 6 {
            assert!(buf[k] != b'#');
            k += 1;
        }
        // Os bytes além de `size` não são tocados.
        while k < 10 {
            assert_eq!(buf[k], b'#');
            k += 1;
        }
    }

    #[test]
    fn test_same_seed_same_text() {
        let mut a = [0u8; 500];
        let mut b = [0u8; 500];
        fill_lorem(&mut a, 500, 2026);
        fill_lorem(&mut b, 500, 2026);
        let mut k = 0;
        while k < 500 {
            assert_eq!(a[k], b[k]);
            k += 1;
        }
    }
}
