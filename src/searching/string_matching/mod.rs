//! Módulo de Casamento de Padrões em Textos (String Matching).

pub mod kmp;
pub mod naive;

pub use kmp::search as kmp_search;
pub use naive::search as naive_search;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test_case(text: &str, pattern: &str, expected: &[usize]) {
        let res_naive = naive_search(text, pattern);
        let res_kmp = kmp_search(text, pattern);

        assert_eq!(
            res_naive, expected,
            "Naive falhou. Texto: '{}', Padrão: '{}'",
            text, pattern
        );
        assert_eq!(
            res_kmp, expected,
            "KMP falhou. Texto: '{}', Padrão: '{}'",
            text, pattern
        );
        assert_eq!(
            res_naive, res_kmp,
            "Divergência entre Naive e KMP. Texto: '{}', Padrão: '{}'",
            text, pattern
        );
    }

    #[test]
    fn test_pattern_at_start() {
        run_test_case("abcde", "abc", &[0]);
    }

    #[test]
    fn test_pattern_at_end() {
        run_test_case("abcde", "cde", &[2]);
    }

    #[test]
    fn test_pattern_in_middle() {
        run_test_case("abcdef", "cd", &[2]);
    }

    #[test]
    fn test_multiple_occurrences() {
        run_test_case("ab x ab y ab", "ab", &[0, 5, 10]);
    }

    #[test]
    fn test_overlapping_occurrences() {
        run_test_case("ABABABA", "ABA", &[0, 2, 4]);
        run_test_case("aaaa", "aa", &[0, 1, 2]);
    }

    #[test]
    fn test_pattern_not_found() {
        run_test_case("abcdef", "xyz", &[]);
    }

    #[test]
    fn test_pattern_larger_than_text() {
        run_test_case("abc", "abcdef", &[]);
    }

    #[test]
    fn test_empty_text() {
        run_test_case("", "abc", &[]);
    }

    #[test]
    fn test_empty_pattern() {
        run_test_case("abc", "", &[]);
    }

    #[test]
    fn test_single_char() {
        run_test_case("a", "a", &[0]);
        run_test_case("a", "b", &[]);
    }

    #[test]
    fn test_utf8_characters() {
        // "á" (2 bytes) + "é" (2 bytes) -> "í" inicia no byte 4
        run_test_case("áéíóú", "íó", &[4]);
        // "A" (1 byte) -> "ç" inicia no byte 1
        run_test_case("Ação", "çã", &[1]);
    }

    #[test]
    fn test_repetitive_worst_case() {
        run_test_case("AAAAAAAAAAAAAAAAAAAAB", "AAAAAAAAAC", &[]);
    }
}