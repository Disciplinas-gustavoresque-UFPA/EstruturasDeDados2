//! Algoritmo Ingênuo (Naive String Matching).
//!
//! - Melhor caso: O(n)
//! - Pior caso: O(n * m)
//! - Espaço auxiliar: O(1) (desconsiderando o vetor de saída O(k))

/// Busca todas as ocorrências (incluindo sobrepostas) de `pattern` em `text`.
///
/// # Convenções
/// - Os índices retornados correspondem às posições em **bytes** (UTF-8),
///   e não a posições de caracteres Unicode.
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

    for i in 0..=(n - m) {
        let mut match_found = true;
        for j in 0..m {
            if t[i + j] != p[j] {
                match_found = false;
                break;
            }
        }
        if match_found {
            result.push(i);
        }
    }

    result
}