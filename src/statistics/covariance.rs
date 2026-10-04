use super::mean::mean;

/// Calcula a **covariância amostral** entre duas listas pareadas.
///
/// # O que é
///
/// Mede se dois conjuntos de dados **variam juntos**. As listas são pareadas:
/// `x[0]` anda com `y[0]`, `x[1]` com `y[1]`, e assim por diante (por exemplo,
/// a altura e o peso da mesma pessoa).
///
/// ```text
/// positiva → quando x está acima da média dele, y tende a estar acima da dele
/// negativa → quando um está acima da média, o outro tende a estar abaixo
/// perto de 0 → não há relação linear entre x e y
/// ```
///
/// # Fórmula
///
/// ```text
/// covariância = Σ (xᵢ − média de x) × (yᵢ − média de y) ÷ (n − 1)
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// x = [2, 4, 4, 5, 7, 9]    média de x = 31/6 = 5,1667
/// y = [1, 3, 2, 5, 6, 8]    média de y = 25/6 = 4,1667
///
/// soma dos produtos dos desvios = 31,8333
/// covariância = 31,8333 ÷ (6 − 1) = 6,3667   (exatamente 191/30)
/// ```
///
/// Curiosidade: a covariância de uma lista com ela mesma é a variância.
///
/// # Quando a resposta é `None`
///
/// - As listas têm tamanhos diferentes (não dá para formar os pares).
/// - Menos de 2 pares.
/// - `NaN` ou infinito em qualquer uma das listas.
///
/// # Complexidade
///
/// - Tempo: O(n). Passadas: conferir os dados, as duas médias e a soma dos
///   produtos dos desvios.
/// - Espaço: O(1). Guarda só as duas médias e a soma.
pub fn covariance(x: &[f64], y: &[f64]) -> Option<f64> {
    // Passo 1: as listas precisam ter o mesmo tamanho e pelo menos 2 pares.
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }

    // Passo 2 (1ª passada): as médias das duas listas.
    // Se alguma lista tiver NaN ou infinito, `mean` responde None, e o `?`
    // faz esta função responder None também.
    let mean_x = mean(x)?;
    let mean_y = mean(y)?;

    // Passo 3 (2ª passada): somar os produtos dos desvios, par a par:
    // (x[0], y[0]), (x[1], y[1]), ... É o Σ da fórmula, com i indo de 0 a n − 1.
    let mut sum_of_products = 0.0;
    for i in 0..x.len() {
        sum_of_products += (x[i] - mean_x) * (y[i] - mean_y);
    }

    // Passo 4: dividir por n − 1.
    Some(sum_of_products / (x.len() - 1) as f64)
}

#[cfg(test)]
mod tests {
    use super::covariance;
    use crate::statistics::helpers::assert_close;

    const X: [f64; 6] = [2.0, 4.0, 4.0, 5.0, 7.0, 9.0];
    const Y: [f64; 6] = [1.0, 3.0, 2.0, 5.0, 6.0, 8.0];

    #[test]
    fn test_main_example() {
        // Soma dos produtos dos desvios = 191/6; ÷ (6 − 1) = 191/30
        assert_close(covariance(&X, &Y), 191.0 / 30.0);
    }

    #[test]
    fn test_with_itself_is_variance() {
        // cov(x, x) = variância de x = 37/6
        assert_close(covariance(&X, &X), 37.0 / 6.0);
    }

    #[test]
    fn test_opposite_direction_is_negative() {
        // cov(x, −x) = −(variância de x) = −37/6
        let mut minus_x: Vec<f64> = Vec::new();
        for &value in &X {
            minus_x.push(-value);
        }
        assert_close(covariance(&X, &minus_x), -37.0 / 6.0);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(covariance(&[1.0, 2.0, 3.0], &[1.0, 2.0]), None);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(covariance(&[1.0], &[2.0]), None);
    }

    #[test]
    fn test_empty() {
        assert_eq!(covariance(&[], &[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(covariance(&[1.0, 2.0, 3.0], &[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn test_with_infinity() {
        assert_eq!(
            covariance(&[1.0, f64::INFINITY, 3.0], &[1.0, 2.0, 3.0]),
            None
        );
    }
}
