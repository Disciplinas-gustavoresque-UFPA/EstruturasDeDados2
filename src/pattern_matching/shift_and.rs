//! Shift-And: casamento exato com paralelismo de bit (aula 7).

/// Maior padrão aceito: cabe em uma palavra de 64 bits.
const MAX_PATTERN: usize = 64;

/// Procura **todas** as ocorrências de `pattern` em `text` com o algoritmo
/// Shift-And.
///
/// # Parâmetros e resposta
///
/// Iguais aos de `brute_force`: `text`/`n`, `pattern`/`m`, `out`/`cap`.
/// Resposta `-1` para padrão vazio ou com mais de 64 bytes (não cabe em um
/// `u64`); senão, o número total de ocorrências, com as posições (do começo
/// de cada ocorrência, em bytes) das primeiras `cap` em `out[0..]`.
///
/// # Como funciona
///
/// O conjunto de prefixos do padrão que casam com o texto lido é guardado
/// em uma máscara de bits `r` (um `u64`). O bit j de `r` vale 1 quando os
/// primeiros j + 1 bytes do padrão casam com os últimos j + 1 bytes lidos.
///
/// 1. Pré-processamento: para cada byte `c`, `masks[c]` tem o bit j ligado
///    quando `pattern[j] == c`.
/// 2. A cada byte do texto: `r = ((r << 1) | 1) & masks[byte]`.
/// 3. Se o bit m − 1 de `r` está ligado, o padrão inteiro casou, e a
///    ocorrência começa em `i + 1 − m`.
///
/// Nos slides, o bit 0 fica à esquerda e o deslocamento é `>> 1`. Aqui o bit
/// 0 é o menos significativo e o deslocamento é `<< 1`. É a mesma conta, com
/// a ordem dos bits invertida.
///
/// # Exemplo
///
/// ```text
/// padrão: t e s t e    masks[t] = 01001, masks[e] = 10010, masks[s] = 00100
/// texto:  o s _ t e s t e s ...   -> casa terminando na posição 7 -> começa em 3
/// ```
///
/// # Complexidade
///
/// - Pré-processamento: O(m + c), com c = 256.
/// - Busca: O(n), porque cada byte do texto custa um número fixo de operações.
/// - Espaço extra: O(c), a tabela de máscaras.
pub fn shift_and(
    text: &[u8],
    n: usize,
    pattern: &[u8],
    m: usize,
    out: &mut [usize],
    cap: usize,
) -> isize {
    if m == 0 || m > MAX_PATTERN {
        return -1;
    }

    let mut masks = [0u64; 256];
    let mut j = 0;
    while j < m {
        masks[pattern[j] as usize] |= 1u64 << j;
        j += 1;
    }

    let accept = 1u64 << (m - 1);
    let mut count: usize = 0;
    let mut r = 0u64;
    let mut i = 0;
    while i < n {
        r = ((r << 1) | 1) & masks[text[i] as usize];
        if (r & accept) != 0 {
            if count < cap {
                out[count] = i + 1 - m;
            }
            count += 1;
        }
        i += 1;
    }
    count as isize
}

#[cfg(test)]
mod tests {
    use super::shift_and;

    #[test]
    fn test_longest_pattern_that_fits() {
        let pattern = [b'a'; 64];
        let text = [b'a'; 65];
        let mut out = [0usize; 4];
        assert_eq!(shift_and(&text, 65, &pattern, 64, &mut out, 4), 2);
        assert_eq!(out[0], 0);
        assert_eq!(out[1], 1);
    }

    #[test]
    fn test_pattern_too_long_is_invalid() {
        let pattern = [b'a'; 65];
        let text = [b'a'; 100];
        let mut out = [0usize; 4];
        assert_eq!(shift_and(&text, 100, &pattern, 65, &mut out, 4), -1);
    }

    #[test]
    fn test_single_byte_pattern() {
        let mut out = [0usize; 8];
        assert_eq!(shift_and(b"banana", 6, b"a", 1, &mut out, 8), 3);
        assert_eq!(out[0], 1);
        assert_eq!(out[1], 3);
        assert_eq!(out[2], 5);
    }
}
