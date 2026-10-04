use super::quantile::quantile;

/// Calcula a **mediana**: o valor do meio dos dados ordenados.
///
/// # O que é
///
/// Metade dos valores fica abaixo da mediana e metade fica acima. Diferente da
/// média, a mediana não é "puxada" por valores extremos: para `[1, 2, 1000]`,
/// a mediana é 2, enquanto a média é 334,3.
///
/// # Como é calculada
///
/// É o quantil 0,5 (veja a função `quantile`). Na prática:
///
/// ```text
/// n ímpar → o valor do meio:              [1, 2, 3]    → 2
/// n par   → a média dos dois do meio:     [1, 2, 3, 4] → (2 + 3) ÷ 2 = 2,5
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados (ordenados) = [2, 4, 4, 5, 7, 9]   (n = 6, par)
/// os dois do meio   = 4 e 5
/// mediana           = (4 + 5) ÷ 2 = 4,5
/// ```
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n log n), o custo de ordenar a cópia.
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn median(data: &[f64]) -> Option<f64> {
    // A mediana é o quantil 0,5.
    quantile(data, 0.5)
}

#[cfg(test)]
mod tests {
    use super::median;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // (4 + 5) ÷ 2 = 9/2
        assert_close(median(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]), 4.5);
    }

    #[test]
    fn test_odd_length() {
        // n ímpar: o valor do meio.
        assert_close(median(&[1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn test_even_length() {
        // n par: (2 + 3) ÷ 2 = 5/2
        assert_close(median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }

    #[test]
    fn test_unsorted_input() {
        // [3, 1, 2] ordenado é [1, 2, 3] → 2
        assert_close(median(&[3.0, 1.0, 2.0]), 2.0);
    }

    #[test]
    fn test_not_pulled_by_extremes() {
        // A média seria 334,3; a mediana continua 2.
        assert_close(median(&[1.0, 2.0, 1000.0]), 2.0);
    }

    #[test]
    fn test_negative_values() {
        // [−3, −1, 0, 2]: os dois do meio são −1 e 0 → −1/2
        assert_close(median(&[-3.0, -1.0, 0.0, 2.0]), -0.5);
    }

    #[test]
    fn test_single_value() {
        assert_close(median(&[5.0]), 5.0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(median(&[1.0, f64::NAN, 3.0]), None);
    }
}
