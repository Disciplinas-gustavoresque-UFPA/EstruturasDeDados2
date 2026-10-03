use super::covariance::covariance;
use super::std_dev::std_dev;

/// Calcula o **coeficiente de correlação de Pearson** entre duas listas
/// pareadas.
///
/// # O que é
///
/// É a covariância "padronizada": um número sempre entre −1 e 1, que mede a
/// força e a direção da relação **linear** entre x e y.
///
/// ```text
///  1  → relação linear perfeita crescente    (exemplo: y = 2x)
///  0  → nenhuma relação linear
/// −1  → relação linear perfeita decrescente  (exemplo: y = −x)
/// ```
///
/// # Fórmula
///
/// ```text
/// r = covariância(x, y) ÷ (desvio padrão de x × desvio padrão de y)
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// x = [2, 4, 4, 5, 7, 9]
/// y = [1, 3, 2, 5, 6, 8]
///
/// covariância        = 6,3667
/// desvio padrão de x = 2,4833
/// desvio padrão de y = 2,6394
/// r = 6,3667 ÷ (2,4833 × 2,6394) = 0,9713   (relação forte e crescente)
/// ```
///
/// # Dois cuidados
///
/// 1. **Lista constante → `None`.** Se todos os valores de uma lista são
///    iguais, ela não varia, e não existe "variar junto". A função confere
///    diretamente se "todos os valores são iguais", e não se "desvio padrão
///    = 0". Motivo: para `[0.1, 0.1, 0.1]`, o computador calcula o desvio
///    padrão como 0,000000000000000017 em vez de 0, porque 0,1 não é guardado
///    de forma exata.
/// 2. **Resultado limitado ao intervalo de −1 a 1.** Com y = 2x, a conta dá
///    1,0000000000000002 (e, com y = −x, −1,0000000000000002), o que é
///    impossível na matemática. O limite corrige esse erro de arredondamento.
///
/// # Quando a resposta é `None`
///
/// - Nos mesmos casos da covariância: tamanhos diferentes, menos de 2 pares,
///   `NaN` ou infinito.
/// - Uma das listas é constante (todos os valores iguais).
///
/// # Complexidade
///
/// - Tempo: O(n). Usa a covariância e os dois desvios padrão (todos O(n)),
///   mais a conferência de lista constante (uma passada).
/// - Espaço: O(1).
pub fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    // Passo 1: a covariância. Ela já confere os tamanhos, se há pelo menos
    // 2 pares e se há NaN ou infinito.
    let cov = covariance(x, y)?;

    // Passo 2: lista constante → não existe correlação.
    if is_constant(x) || is_constant(y) {
        return None;
    }

    // Passo 3: dividir pela multiplicação dos desvios padrão.
    let r = cov / (std_dev(x)? * std_dev(y)?);

    // Passo 4: limitar ao intervalo de −1 a 1 (corrige o arredondamento).
    if r > 1.0 {
        return Some(1.0);
    }
    if r < -1.0 {
        return Some(-1.0);
    }
    Some(r)
}

/// Confere se todos os valores da lista são iguais ao primeiro.
fn is_constant(data: &[f64]) -> bool {
    for &value in data {
        if value != data[0] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::correlation;
    use crate::statistics::helpers::assert_close;

    const X: [f64; 6] = [2.0, 4.0, 4.0, 5.0, 7.0, 9.0];
    const Y: [f64; 6] = [1.0, 3.0, 2.0, 5.0, 6.0, 8.0];

    #[test]
    fn test_main_example() {
        // (191/30) ÷ √(37/6 × 209/30)
        let expected = (191.0 / 30.0) / (37.0_f64 / 6.0 * (209.0 / 30.0)).sqrt();
        assert_close(correlation(&X, &Y), expected);
    }

    #[test]
    fn test_perfect_positive() {
        // y = 2x: a conta daria 1,0000000000000002; o limite devolve 1 exato.
        let mut double: Vec<f64> = Vec::new();
        for &value in &X {
            double.push(2.0 * value);
        }
        assert_eq!(correlation(&X, &double), Some(1.0));
    }

    #[test]
    fn test_perfect_negative() {
        // y = −x: a conta daria −1,0000000000000002; o limite devolve −1 exato.
        let mut minus: Vec<f64> = Vec::new();
        for &value in &X {
            minus.push(-value);
        }
        assert_eq!(correlation(&X, &minus), Some(-1.0));
    }

    #[test]
    fn test_constant_list() {
        // [0.1, 0.1, 0.1] não varia: não existe correlação.
        assert_eq!(correlation(&[0.1, 0.1, 0.1], &[1.0, 2.0, 3.0]), None);
        assert_eq!(correlation(&[1.0, 2.0, 3.0], &[5.0, 5.0, 5.0]), None);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(correlation(&[1.0, 2.0, 3.0], &[1.0, 2.0]), None);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(correlation(&[1.0], &[2.0]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(correlation(&[1.0, 2.0, 3.0], &[1.0, f64::NAN, 3.0]), None);
    }
}
