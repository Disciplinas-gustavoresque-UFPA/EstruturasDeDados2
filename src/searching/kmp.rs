/// Array LPS (Longest Prefix Suffix) para o padrão fornecido.
/// Otimiza os saltos em caso de incompatibilidade durante a busca.
fn build_lps(pattern: &[char]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut length = 0; // tamanho do maior prefixo-sufixo anterior
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else if length != 0 {
            // Volta para o último prefixo válido sem incrementar `i`
            length = lps[length - 1];
        } else { // Nenhuma correspondência de prefixo/sufixo
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

/// Algoritmo Knuth-Morris-Pratt (KMP).
/// Realiza a busca de um padrão de texto em uma string maior.
/// Retorna um vetor contendo os índices iniciais de todas as ocorrências encontradas.
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut occurrences = Vec::new();

    if pattern.is_empty() || text.is_empty() || pattern.len() > text.len() {
        return occurrences;
    }

    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let lps = build_lps(&pattern_chars);

    let mut i = 0; // Índice para percorrer o texto
    let mut j = 0; // Índice para percorrer o padrão

    while i < text_chars.len() {
        if pattern_chars[j] == text_chars[i] {
            i += 1;
            j += 1;
        }

        if j == pattern_chars.len() {
            // Ocorrência inteira encontrada; armazena o índice inicial
            occurrences.push(i - j);
            j = lps[j - 1]; // Prepara para buscar próximas ocorrências
        } else if i < text_chars.len() && pattern_chars[j] != text_chars[i] {
            // Incompatibilidade após j correspondências
            if j != 0 {
                j = lps[j - 1]; // Pula os caracteres já avaliados usando o LPS
            } else {
                i += 1; // Nenhuma correspondência inicial, avança o texto
            }
        }
    }

    occurrences
}