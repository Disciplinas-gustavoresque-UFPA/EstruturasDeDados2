use super::mean::mean;
use super::std_dev::std_dev;

/// Calcula o **coeficiente de variação** (CV) de uma lista de números.
///
/// # O que é
///
/// É o desvio padrão dividido pela média. Ele mede o espalhamento **em
/// relação ao tamanho dos valores**. Um desvio padrão de 2 é muito para dados
/// com média 5 (CV = 0,4), mas é pouco para dados com média 1000
/// (CV = 0,002). Por isso o CV permite comparar o espalhamento de dados que
/// estão em escalas diferentes.
///
/// O resultado é uma **proporção**: 0,4806 quer dizer 48,06%.
///
/// # Fórmula
///
/// ```text
/// CV = desvio padrão ÷ média
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados         = [2, 4, 4, 5, 7, 9]
/// desvio padrão = 2,4833
/// média         = 5,1667
/// CV            = 2,4833 ÷ 5,1667 = 0,4806   (48,06%)
/// ```
///
/// # Cuidado
///
/// O CV só faz sentido para dados **positivos**. Se a média for negativa, o CV
/// sai negativo (exemplo: `[-3, -1, 0, 2]` dá CV = −4,1633). Se a média estiver
/// perto de zero, o CV fica enorme e instável.
///
/// # Quando a resposta é `None`
///
/// - Menos de 2 números (o desvio padrão não existe).
/// - Lista com `NaN` ou infinito.
/// - Média exatamente zero: a divisão seria por zero.
///
/// # Complexidade
///
/// - Tempo: O(n). Usa a média e o desvio padrão, que são O(n).
/// - Espaço: O(1).
pub fn coefficient_of_variation(data: &[f64]) -> Option<f64> {
    // Passo 1: calcular o desvio padrão
    // (None se houver menos de 2 números ou dados inválidos).
    let deviation = std_dev(data)?;

    // Passo 2: calcular a média.
    let average = mean(data)?;

    // Passo 3: média zero → a divisão seria por zero → sem resposta.
    if average == 0.0 {
        return None;
    }

    // Passo 4: dividir o desvio padrão pela média.
    Some(deviation / average)
}

#[cfg(test)]
mod tests {
    use super::coefficient_of_variation;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // √(37/6) ÷ (31/6)
        assert_close(
            coefficient_of_variation(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]),
            (37.0_f64 / 6.0).sqrt() / (31.0 / 6.0),
        );
    }

    #[test]
    fn test_zero_mean() {
        // Média de [−1, 1] = 0 → divisão por zero → None
        assert_eq!(coefficient_of_variation(&[-1.0, 1.0]), None);
    }

    #[test]
    fn test_negative_mean() {
        // Desvio padrão √(13/3); média −1/2 → CV = √(13/3) ÷ (−1/2) = −2·√(13/3)
        assert_close(
            coefficient_of_variation(&[-3.0, -1.0, 0.0, 2.0]),
            -2.0 * (13.0_f64 / 3.0).sqrt(),
        );
    }

    #[test]
    fn test_single_value() {
        assert_eq!(coefficient_of_variation(&[5.0]), None);
    }

    #[test]
    fn test_empty() {
        assert_eq!(coefficient_of_variation(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(coefficient_of_variation(&[1.0, f64::NAN, 3.0]), None);
    }
}
