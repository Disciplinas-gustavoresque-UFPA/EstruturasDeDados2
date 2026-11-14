use super::quartiles::quartiles;

/// Calcula o **intervalo interquartílico** (IQR): Q3 − Q1.
///
/// # O que é
///
/// É a largura da faixa onde ficam os **50% centrais** dos dados. Como ignora
/// os 25% de cada ponta, não é afetado por valores extremos, ao contrário da
/// amplitude.
///
/// # Fórmula
///
/// ```text
/// IQR = Q3 − Q1
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados = [2, 4, 4, 5, 7, 9]
/// Q1    = 4
/// Q3    = 6,5
/// IQR   = 6,5 − 4 = 2,5
/// ```
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n log n), o custo dos quartis (uma ordenação).
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn iqr(data: &[f64]) -> Option<f64> {
    // Passo 1: calcular os quartis (None se os dados forem inválidos).
    let q = quartiles(data)?;

    // Passo 2: subtrair.
    Some(q.q3 - q.q1)
}

#[cfg(test)]
mod tests {
    use super::iqr;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // 6,5 − 4 = 2,5
        assert_close(iqr(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]), 2.5);
    }

    #[test]
    fn test_even_length() {
        // [1, 2, 3, 4]: 13/4 − 7/4 = 6/4 = 1,5
        assert_close(iqr(&[1.0, 2.0, 3.0, 4.0]), 1.5);
    }

    #[test]
    fn test_all_equal() {
        // Todos iguais: Q1 = Q3 → IQR = 0
        assert_close(iqr(&[7.0, 7.0, 7.0]), 0.0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(iqr(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(iqr(&[1.0, f64::NAN, 3.0]), None);
    }
}
