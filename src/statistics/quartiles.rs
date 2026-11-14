use super::helpers::{is_valid, sorted_copy};
use super::quantile::quantile_sorted;

/// Os três **quartis** de uma lista: os valores que dividem os dados
/// ordenados em quatro partes com a mesma quantidade de valores.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quartiles {
    /// Primeiro quartil (Q1): 25% dos dados ficam abaixo dele.
    pub q1: f64,
    /// Segundo quartil (Q2): é a mediana; 50% dos dados ficam abaixo dele.
    pub q2: f64,
    /// Terceiro quartil (Q3): 75% dos dados ficam abaixo dele.
    pub q3: f64,
}

/// Calcula os **quartis** Q1, Q2 (a mediana) e Q3.
///
/// # Como são calculados
///
/// São os quantis 0,25, 0,5 e 0,75, pelo método da interpolação linear (veja
/// a função `quantile`). A lista é ordenada **uma vez só**, e os três quartis
/// são lidos da mesma lista ordenada.
///
/// # Exemplo, passo a passo
///
/// ```text
/// posição:  0   1   2   3   4   5
/// valor:    2   4   4   5   7   9
///
/// Q1: h = 5 × 0,25 = 1,25 → 4 + 0,25 × (4 − 4) = 4
/// Q2: h = 5 × 0,5  = 2,5  → 4 + 0,5  × (5 − 4) = 4,5
/// Q3: h = 5 × 0,75 = 3,75 → 5 + 0,75 × (7 − 5) = 6,5
/// ```
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n log n). Uma ordenação, mais três contas de O(1).
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn quartiles(data: &[f64]) -> Option<Quartiles> {
    // Passo 1: conferir os dados.
    if !is_valid(data) {
        return None;
    }

    // Passo 2: ordenar uma cópia, uma única vez.
    let sorted = sorted_copy(data);

    // Passo 3: ler os três quartis da mesma lista ordenada.
    Some(Quartiles {
        q1: quantile_sorted(&sorted, 0.25),
        q2: quantile_sorted(&sorted, 0.5),
        q3: quantile_sorted(&sorted, 0.75),
    })
}

#[cfg(test)]
mod tests {
    use super::quartiles;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        let q = quartiles(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]).unwrap();
        assert_close(Some(q.q1), 4.0);
        assert_close(Some(q.q2), 4.5);
        assert_close(Some(q.q3), 6.5);
    }

    #[test]
    fn test_even_length() {
        // [1, 2, 3, 4]: Q1 = 7/4, Q2 = 5/2, Q3 = 13/4
        let q = quartiles(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_close(Some(q.q1), 1.75);
        assert_close(Some(q.q2), 2.5);
        assert_close(Some(q.q3), 3.25);
    }

    #[test]
    fn test_single_value() {
        // Um valor só: os três quartis são ele mesmo.
        let q = quartiles(&[5.0]).unwrap();
        assert_close(Some(q.q1), 5.0);
        assert_close(Some(q.q2), 5.0);
        assert_close(Some(q.q3), 5.0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(quartiles(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(quartiles(&[1.0, f64::NAN, 3.0]), None);
    }
}
