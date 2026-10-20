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
/// # Números enormes: quando a soma estoura
///
/// O maior `f64` é cerca de 1,8 × 10³⁰⁸. Para `[1e308, 1e308]`, a soma
/// (2 × 10³⁰⁸) passa desse limite e vira infinito, embora a média (1e308)
/// caiba. Nesse caso, a função usa uma segunda forma da mesma conta:
///
/// ```text
/// média = x₁ ÷ n + x₂ ÷ n + ... + xₙ ÷ n
/// ```
///
/// Cada parcela é no máximo o maior valor absoluto dividido por n. Por isso,
/// até as somas parciais ficam no máximo do tamanho do maior valor absoluto,
/// que cabe no `f64`. Essa forma só é usada quando a primeira estoura, porque
/// dividir antes de somar arredonda n vezes em vez de uma.
///
/// Por fim, a média de verdade sempre fica entre o menor e o maior valor. Mas
/// esses n arredondamentos podem empurrar a conta para fora desse intervalo:
/// com `[1e306; 1000]`, ela daria 1,000000000000003 × 10³⁰⁶, mais que todos os
/// valores; com `[f64::MAX; 3]`, que não tem folga nenhuma até o limite, daria
/// infinito. Por isso, nesse caminho, o resultado é trazido de volta para
/// dentro do intervalo, como a correlação faz com −1 e 1.
///
/// # Quando a resposta é `None`
///
/// - Lista vazia: não há números para somar (seria 0 ÷ 0).
/// - Lista com `NaN` ou infinito: a média não teria sentido.
///
/// # Complexidade
///
/// - Tempo: O(n). Uma passada para conferir os dados e outra para somar (mais
///   três, só quando a soma estoura).
/// - Espaço: O(1). Guarda só a soma, qualquer que seja o tamanho da lista.
pub fn mean(data: &[f64]) -> Option<f64> {
    // Passo 1: conferir os dados. Lista vazia, NaN ou infinito → sem resposta.
    if !is_valid(data) {
        return None;
    }

    // A quantidade de números, como decimal (`as f64`).
    let n = data.len() as f64;

    // Passo 2 (caminho normal): somar todos os números e dividir por n.
    let sum: f64 = data.iter().sum();
    if sum.is_finite() {
        return Some(sum / n);
    }

    // Passo 3 (a soma estourou): dividir cada número por n antes de somar.
    let average: f64 = data.iter().map(|&value| value / n).sum();

    // Passo 4: manter o resultado entre o menor e o maior valor.
    let smallest = data.iter().copied().fold(f64::INFINITY, f64::min);
    let largest = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Some(average.clamp(smallest, largest))
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

    #[test]
    fn test_huge_values_do_not_overflow() {
        // Revisão do PR (P1): 1e308 + 1e308 passa do limite do f64 (≈ 1,8 × 10³⁰⁸),
        // mas a média, 1e308, cabe. A resposta não pode ser infinito.
        assert_eq!(mean(&[1e308, 1e308]), Some(1e308));
        // Mil valores iguais a 1e306: a soma também estoura. Dividir antes de
        // somar daria 1,000000000000003 × 10³⁰⁶, mais que todos os valores; o
        // resultado volta para o intervalo dos dados.
        assert_eq!(mean(&[1e306; 1000]), Some(1e306));
    }

    #[test]
    fn test_values_at_the_limit() {
        // Três vezes o maior f64: a média é ele mesmo, e não infinito.
        assert_eq!(mean(&[f64::MAX; 3]), Some(f64::MAX));
        assert_eq!(mean(&[-f64::MAX; 3]), Some(-f64::MAX));
        // A soma parcial estoura (MAX + MAX), mas o −MAX traz a média para
        // MAX ÷ 3. A comparação é relativa, porque os números são enormes.
        let m = mean(&[f64::MAX, f64::MAX, -f64::MAX]).unwrap();
        assert!((m / (f64::MAX / 3.0) - 1.0).abs() < 1e-12);
    }
}
