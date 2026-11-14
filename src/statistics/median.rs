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
/// Apesar de o exemplo mostrar a lista ordenada, a função **não** ordena a
/// lista inteira: o `quantile` usa o `quickselect` da biblioteca para achar
/// só os valores do meio.
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: **O(n)**, o custo do `quantile` (quickselect, O(n) no pior caso).
/// - Espaço: O(n), o tamanho da cópia (a lista original não muda).
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

    #[test]
    fn test_huge_values() {
        // (f64::MAX + f64::MAX) ÷ 2 estouraria; a mediana certa é f64::MAX.
        assert_eq!(median(&[f64::MAX, f64::MAX]), Some(f64::MAX));
        // Entre −f64::MAX e f64::MAX, o meio é 0.
        assert_close(median(&[f64::MAX, -f64::MAX]), 0.0);
    }

    #[test]
    fn test_long_list_with_repeats() {
        // 1001 valores: os dígitos de 0 a 9, repetidos e fora de ordem.
        // O 0 aparece 101 vezes (posições 0 a 100 da lista ordenada) e cada
        // outro dígito 100 vezes; o 4 ocupa as posições 401 a 500. O meio é a
        // posição 500, então a mediana é 4.
        let data: Vec<f64> = (0..1001).map(|i| ((i * 7) % 10) as f64).collect();
        assert_close(median(&data), 4.0);
    }
}
