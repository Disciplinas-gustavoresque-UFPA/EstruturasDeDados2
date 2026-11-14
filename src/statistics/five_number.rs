use super::helpers::{is_valid, sorted_copy};
use super::quantile::quantile_sorted;

/// O **resumo de cinco números** de uma lista: mínimo, Q1, mediana, Q3 e
/// máximo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FiveNumberSummary {
    /// O menor valor.
    pub min: f64,
    /// Primeiro quartil: 25% dos dados ficam abaixo dele.
    pub q1: f64,
    /// Mediana: 50% dos dados ficam abaixo dela.
    pub median: f64,
    /// Terceiro quartil: 75% dos dados ficam abaixo dele.
    pub q3: f64,
    /// O maior valor.
    pub max: f64,
}

/// Calcula o **resumo de cinco números**.
///
/// # O que é
///
/// São cinco valores que descrevem os dados de uma vez: onde começam (mínimo),
/// onde terminam (máximo), onde fica o meio (mediana) e onde ficam os 50%
/// centrais (de Q1 a Q3). É a base do gráfico de caixa (*boxplot*).
///
/// # Como é calculado
///
/// A lista é ordenada **uma vez só**. O mínimo e o máximo são as pontas da
/// lista ordenada (os quantis 0 e 1), e Q1, mediana e Q3 são os quantis 0,25,
/// 0,5 e 0,75 (veja a função `quantile`).
///
/// # Exemplo
///
/// ```text
/// dados = [2, 4, 4, 5, 7, 9]
///
/// mínimo  Q1   mediana   Q3    máximo
///   2     4      4,5     6,5     9
/// ```
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n log n). Uma ordenação, mais cinco leituras de O(1).
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn five_number_summary(data: &[f64]) -> Option<FiveNumberSummary> {
    // Passo 1: conferir os dados.
    if !is_valid(data) {
        return None;
    }

    // Passo 2: ordenar uma cópia, uma única vez.
    let sorted = sorted_copy(data);

    // Passo 3: ler os cinco números da mesma lista ordenada.
    Some(FiveNumberSummary {
        min: sorted[0],
        q1: quantile_sorted(&sorted, 0.25),
        median: quantile_sorted(&sorted, 0.5),
        q3: quantile_sorted(&sorted, 0.75),
        max: sorted[sorted.len() - 1],
    })
}

#[cfg(test)]
mod tests {
    use super::five_number_summary;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        let s = five_number_summary(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]).unwrap();
        assert_close(Some(s.min), 2.0);
        assert_close(Some(s.q1), 4.0);
        assert_close(Some(s.median), 4.5);
        assert_close(Some(s.q3), 6.5);
        assert_close(Some(s.max), 9.0);
    }

    #[test]
    fn test_negative_values() {
        // [−3, −1, 0, 2]:
        // Q1: h = 3 × 0,25 = 0,75 → −3 + 0,75 × (−1 − (−3)) = −1,5
        // Q3: h = 3 × 0,75 = 2,25 →  0 + 0,25 × (2 − 0)     =  0,5
        let s = five_number_summary(&[-3.0, -1.0, 0.0, 2.0]).unwrap();
        assert_close(Some(s.min), -3.0);
        assert_close(Some(s.q1), -1.5);
        assert_close(Some(s.median), -0.5);
        assert_close(Some(s.q3), 0.5);
        assert_close(Some(s.max), 2.0);
    }

    #[test]
    fn test_unsorted_input() {
        let s = five_number_summary(&[9.0, 4.0, 2.0, 7.0, 4.0, 5.0]).unwrap();
        assert_close(Some(s.min), 2.0);
        assert_close(Some(s.max), 9.0);
    }

    #[test]
    fn test_single_value() {
        let s = five_number_summary(&[5.0]).unwrap();
        assert_close(Some(s.min), 5.0);
        assert_close(Some(s.median), 5.0);
        assert_close(Some(s.max), 5.0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(five_number_summary(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(five_number_summary(&[1.0, f64::NAN, 3.0]), None);
    }
}
