//! BMHS: Boyer-Moore-Horspool-Sunday (aula 7).

/// Monta a tabela de deslocamento do BMHS para um padrão de tamanho m.
///
/// Todas as entradas começam em m + 1. Depois, para cada posição k do
/// padrão, `d[P[k]] = m − k`. Como as posições são percorridas da esquerda
/// para a direita, vale a última ocorrência do byte, que é o menor
/// deslocamento: `d[x] = min { j | 1 ≤ j ≤ m e P[m − j] = x }`.
///
/// Exemplo: para `teste`, d[t] = 2, d[e] = 1, d[s] = 3 e os outros são 6.
fn build_shift_table(pattern: &[u8], m: usize) -> [usize; 256] {
    let mut d = [m + 1; 256];
    let mut k = 0;
    while k < m {
        d[pattern[k] as usize] = m - k;
        k += 1;
    }
    d
}

/// Procura **todas** as ocorrências de `pattern` em `text` com o algoritmo
/// BMHS.
///
/// # Parâmetros e resposta
///
/// Iguais aos de `brute_force`: `text`/`n`, `pattern`/`m`, `out`/`cap`.
/// Resposta `-1` para padrão vazio; senão, o número total de ocorrências, com
/// as posições (em bytes) das primeiras `cap` em `out[0..]`.
///
/// # Como funciona
///
/// 1. Pré-processa o padrão: monta a tabela `d` (256 entradas, uma por byte).
/// 2. Em cada janela, compara o padrão com o texto. Se casar, registra a
///    posição.
/// 3. Desloca a janela usando o byte do texto que vem **logo depois** da
///    janela: `i += d[text[i + m]]`. Se esse byte não está no padrão, o
///    deslocamento é m + 1.
///
/// # Exemplo
///
/// ```text
/// texto:  h b a d e c a e d c a d e       padrão: c a d e   (m = 4)
/// d[c] = 4, d[a] = 3, d[d] = 2, d[e] = 1, os outros = 5
/// resultado: 1 ocorrência, na posição 9
/// ```
///
/// # Complexidade
///
/// - Pré-processamento: O(m + c), com c = 256 (o tamanho do alfabeto de bytes).
/// - Busca: pior caso O(n·m); caso esperado O(n/m).
/// - Espaço extra: O(c), a tabela.
pub fn bmhs(
    text: &[u8],
    n: usize,
    pattern: &[u8],
    m: usize,
    out: &mut [usize],
    cap: usize,
) -> isize {
    if m == 0 {
        return -1;
    }
    if m > n {
        return 0;
    }

    let d = build_shift_table(pattern, m);
    let mut count: usize = 0;
    let mut i = 0;
    while i + m <= n {
        let mut j = 0;
        while j < m && text[i + j] == pattern[j] {
            j += 1;
        }
        if j == m {
            if count < cap {
                out[count] = i;
            }
            count += 1;
        }
        // Se a janela chegou ao fim do texto, não existe o byte seguinte.
        if i + m == n {
            break;
        }
        i += d[text[i + m] as usize];
    }
    count as isize
}

#[cfg(test)]
mod tests {
    use super::{bmhs, build_shift_table};

    #[test]
    fn test_shift_table_teste() {
        // Slide: para "teste", d[t] = 2, d[e] = 1, d[s] = 3 e os outros = m + 1 = 6.
        let d = build_shift_table(b"teste", 5);
        assert_eq!(d[b't' as usize], 2);
        assert_eq!(d[b'e' as usize], 1);
        assert_eq!(d[b's' as usize], 3);
        assert_eq!(d[b'x' as usize], 6);
    }

    #[test]
    fn test_shift_table_moore() {
        // Exercício 1a: d[M] = 5, d[O] = 3, d[R] = 2, d[E] = 1, d[B] = d[Y] = 6.
        let d = build_shift_table(b"MOORE", 5);
        assert_eq!(d[b'M' as usize], 5);
        assert_eq!(d[b'O' as usize], 3);
        assert_eq!(d[b'R' as usize], 2);
        assert_eq!(d[b'E' as usize], 1);
        assert_eq!(d[b'B' as usize], 6);
        assert_eq!(d[b'Y' as usize], 6);
    }

    #[test]
    fn test_window_at_the_end_of_the_text() {
        // O padrão está no fim do texto: não pode ler um byte além dele.
        let mut out = [0usize; 4];
        assert_eq!(bmhs(b"xxabc", 5, b"abc", 3, &mut out, 4), 1);
        assert_eq!(out[0], 2);
    }
}
