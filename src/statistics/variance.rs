use super::mean::mean;

/// Calcula a **variância amostral** de uma lista de números.
///
/// # O que é
///
/// A variância mede o quanto os dados se **espalham** em volta da média. Se
/// todos os números são iguais, a variância é 0. Quanto mais longe da média
/// os números estão, maior a variância.
///
/// # Fórmula
///
/// ```text
/// variância = Σ (xᵢ − média)² ÷ (n − 1)
/// ```
///
/// Divide-se por **n − 1**, e não por n, porque os dados são tratados como
/// uma **amostra**. É a correção usual para que a variância da amostra não
/// fique, em média, menor que a variância da população de onde ela veio.
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados     = [2, 4, 4, 5, 7, 9]
/// média     = 31 ÷ 6 = 5,1667
/// desvios   = −3,1667  −1,1667  −1,1667  −0,1667  1,8333  3,8333
/// ao quadrado e somados = 30,8333
/// variância = 30,8333 ÷ (6 − 1) = 6,1667   (exatamente 37/6)
/// ```
///
/// # Por que "duas passadas"
///
/// A função percorre a lista duas vezes: uma para a média e outra para os
/// desvios. Existe uma fórmula de atalho, `Σx² − n·média²`, que faria tudo
/// numa passada só. Mas o computador guarda só uns 16 dígitos de cada número,
/// e o atalho subtrai dois números enormes e quase iguais: os dígitos que
/// importam se perdem. Exemplo:
///
/// ```text
/// dados = [1000000001, 1000000002, 1000000003]   (variância verdadeira = 1)
/// fórmula de atalho → 0   (errado)
/// duas passadas     → 1   (certo)
/// ```
///
/// # Quando a resposta é `None`
///
/// - Menos de 2 números: com um número só não existe espalhamento para medir
///   (e a divisão seria por n − 1 = 0).
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n). Passadas pela lista: conferir os dados, calcular a média e
///   somar os desvios. Um número fixo de passadas continua sendo O(n).
/// - Espaço: O(1). Guarda só a média e a soma.
pub fn variance(data: &[f64]) -> Option<f64> {
    // Passo 1: são precisos pelo menos 2 números.
    if data.len() < 2 {
        return None;
    }

    // Passo 2 (1ª passada): calcular a média.
    // O `?` quer dizer: se `mean` responder None (dados inválidos), esta
    // função para aqui e também responde None.
    let average = mean(data)?;

    // Passo 3 (2ª passada): somar os desvios ao quadrado, (x − média)².
    let mut sum_of_squares = 0.0;
    for &value in data {
        let deviation = value - average;
        sum_of_squares += deviation * deviation;
    }

    // Passo 4: dividir por n − 1.
    Some(sum_of_squares / (data.len() - 1) as f64)
}

#[cfg(test)]
mod tests {
    use super::variance;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // Desvios ao quadrado somados = 185/6; dividido por (6 − 1) = 37/6
        assert_close(variance(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]), 37.0 / 6.0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(variance(&[]), None);
    }

    #[test]
    fn test_single_value() {
        // Um número só: não há espalhamento para medir.
        assert_eq!(variance(&[5.0]), None);
    }

    #[test]
    fn test_all_equal() {
        // Todos iguais: nenhum desvio, variância 0.
        assert_close(variance(&[7.0, 7.0, 7.0]), 0.0);
    }

    #[test]
    fn test_negative_values() {
        // Média −1/2; desvios −2,5 −0,5 0,5 2,5; quadrados somados = 13;
        // 13 ÷ (4 − 1) = 13/3
        assert_close(variance(&[-3.0, -1.0, 0.0, 2.0]), 13.0 / 3.0);
    }

    #[test]
    fn test_large_numbers_two_passes() {
        // Média = 1000000002; desvios −1, 0, 1; quadrados somados = 2;
        // 2 ÷ (3 − 1) = 1. A fórmula de atalho daria 0 aqui.
        let data = [1_000_000_001.0, 1_000_000_002.0, 1_000_000_003.0];
        assert_close(variance(&data), 1.0);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(variance(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn test_with_infinity() {
        assert_eq!(variance(&[1.0, f64::INFINITY, 3.0]), None);
    }
}
