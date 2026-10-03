use super::variance::variance;

/// Calcula o **desvio padrão** (amostral) de uma lista de números.
///
/// # O que é
///
/// É a raiz quadrada da variância. A variância fica em "unidades ao
/// quadrado": se os dados estão em metros, a variância está em metros². A raiz
/// traz a medida de volta para a **mesma unidade dos dados**, e por isso o
/// desvio padrão é mais fácil de interpretar: é um valor típico do quanto os
/// números se afastam da média.
///
/// # Fórmula
///
/// ```text
/// desvio padrão = √variância
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados         = [2, 4, 4, 5, 7, 9]
/// variância     = 37/6 = 6,1667
/// desvio padrão = √6,1667 = 2,4833
/// ```
///
/// # Quando a resposta é `None`
///
/// Nos mesmos casos da variância: menos de 2 números, `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n), o custo da variância. A raiz quadrada é uma conta só, O(1).
/// - Espaço: O(1).
pub fn std_dev(data: &[f64]) -> Option<f64> {
    // Passo 1: calcular a variância (None se não for possível).
    let var = variance(data)?;

    // Passo 2: tirar a raiz quadrada.
    Some(var.sqrt())
}

#[cfg(test)]
mod tests {
    use super::std_dev;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // √(37/6)
        assert_close(
            std_dev(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]),
            (37.0_f64 / 6.0).sqrt(),
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(std_dev(&[]), None);
    }

    #[test]
    fn test_single_value() {
        assert_eq!(std_dev(&[5.0]), None);
    }

    #[test]
    fn test_all_equal() {
        // Variância 0 → desvio padrão √0 = 0
        assert_close(std_dev(&[7.0, 7.0, 7.0]), 0.0);
    }

    #[test]
    fn test_negative_values() {
        // √(13/3)
        assert_close(std_dev(&[-3.0, -1.0, 0.0, 2.0]), (13.0_f64 / 3.0).sqrt());
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(std_dev(&[1.0, f64::NAN, 3.0]), None);
    }
}
