//! Algoritmo Knuth-Morris-Pratt (KMP) para String Matching.
//!
//! - Pré-processamento (LPS): O(m)
//! - Busca: O(n)
//! - Tempo total: O(n + m) no pior caso
//! - Espaço auxiliar: O(m) para a tabela LPS (desconsiderando o vetor de saída O(k))

/// Constrói a tabela LPS (Longest Proper Prefix which is also Suffix)
/// para a sequência de bytes do padrão.
fn compute_lps(p: &[u8]) -> Vec<usize> {
    let m = p.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if p[i] == p[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}

/// Busca todas as ocorrências (incluindo sobrepostas) de `pattern` em `text`.
///
/// # Convenções
/// - Opera diretamente sobre bytes UTF-8 (`as_bytes()`), mantendo o espaço auxiliar
///   estritamente em O(m) sem copiar o texto inteiro para `Vec<char>`.
/// - Os índices retornados correspondem às posições em **bytes** (UTF-8).
/// - Se `pattern` for vazio (`""`), retorna um vetor vazio.
pub fn search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();

    if pattern.is_empty() {
        return result;
    }

    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m > n {
        return result;
    }

    let lps = compute_lps(p);
    let mut i = 0;
    let mut j = 0;

    while i < n {
        if t[i] == p[j] {
            i += 1;
            j += 1;

            if j == m {
                result.push(i - j);
                j = lps[j - 1];
            }
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::compute_lps;

    #[test]
    fn test_compute_lps_classic() {
        assert_eq!(compute_lps(b"ABABAC"), vec![0, 0, 1, 2, 3, 0]);
        assert_eq!(compute_lps(b"AAAA"), vec![0, 1, 2, 3]);
        assert_eq!(compute_lps(b"ABCDE"), vec![0, 0, 0, 0, 0]);
        assert_eq!(
            compute_lps(b"AABAACAABAA"),
            vec![0, 1, 0, 1, 2, 0, 1, 2, 3, 4, 5]
        );
    }
}