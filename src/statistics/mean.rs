use super::helpers::is_valid;

/// Calcula a **média aritmética** de uma lista de números.
///
/// # O que é
///
/// A média é o "ponto de equilíbrio" dos dados. Imagine cada número como um
/// peso pendurado numa régua, na posição do seu valor: a média é o ponto onde
/// a régua fica equilibrada.
///
/// # Fórmula
///
/// ```text
/// média = (x₁ + x₂ + ... + xₙ) ÷ n
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados = [2, 4, 4, 5, 7, 9]
/// soma  = 2 + 4 + 4 + 5 + 7 + 9 = 31
/// n     = 6
/// média = 31 ÷ 6 = 5,1667
/// ```
///
/// # Quando a resposta é `None`
///
/// - Lista vazia: não há números para somar (seria 0 ÷ 0).
/// - Lista com `NaN` ou infinito: a média não teria sentido.
///
/// # Complexidade
///
/// - Tempo: O(n). Uma passada para conferir os dados e outra para somar.
/// - Espaço: O(1). Guarda só a soma, qualquer que seja o tamanho da lista.
pub fn mean(data: &[f64]) -> Option<f64> {
    // Passo 1: conferir os dados. Lista vazia, NaN ou infinito → sem resposta.
    if !is_valid(data) {
        return None;
    }

    // Passo 2: somar todos os números, um por um.
    let mut sum = 0.0;
    for &value in data {
        sum += value;
    }

    // Passo 3: dividir a soma pela quantidade de números.
    // (`as f64` transforma a quantidade, que é um número inteiro, em decimal.)
    Some(sum / data.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::mean;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // (2 + 4 + 4 + 5 + 7 + 9) ÷ 6 = 31 ÷ 6
        assert_close(mean(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]), 31.0 / 6.0);
    }

    #[test]
    fn test_empty() {
        // Lista vazia: não existe média.
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn test_single_value() {
        // Um único número: a média é ele mesmo. 5 ÷ 1 = 5
        assert_close(mean(&[5.0]), 5.0);
    }

    #[test]
    fn test_all_equal() {
        // (7 + 7 + 7) ÷ 3 = 21 ÷ 3 = 7
        assert_close(mean(&[7.0, 7.0, 7.0]), 7.0);
    }

    #[test]
    fn test_negative_values() {
        // (−3 − 1 + 0 + 2) ÷ 4 = −2 ÷ 4 = −1/2
        assert_close(mean(&[-3.0, -1.0, 0.0, 2.0]), -0.5);
    }

    #[test]
    fn test_with_nan() {
        // Um NaN na lista: sem resposta.
        assert_eq!(mean(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn test_with_infinity() {
        // Um infinito na lista: sem resposta.
        assert_eq!(mean(&[1.0, f64::INFINITY, 3.0]), None);
    }
}
