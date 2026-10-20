use super::mean::mean;

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
/// # Como a conta é feita: primeiro, normalizar
///
/// Na fórmula, o (n − 1) da covariância e dos desvios padrão se cancela:
///
/// ```text
/// r = Σ dxᵢ·dyᵢ ÷ √(Σ dxᵢ² × Σ dyᵢ²)     (dx = x − média de x; dy = y − média de y)
/// ```
///
/// Fazer essa conta direto nos dados pode estragar o resultado nos extremos
/// do `f64`:
///
/// ```text
/// x = [0, 1e-200]   → dx² ≈ 1e-401, pequeno demais: vira 0, e r = 0 ÷ 0 = NaN
/// x = [−1e308, 1e308] → dx² ≈ 1e616, grande demais: vira infinito, e r = NaN
/// ```
///
/// Por isso, antes das contas, cada lista é **dividida pelo seu maior valor
/// absoluto**. Os valores passam a ficar entre −1 e 1, e os quadrados e
/// produtos não estouram nem somem. A correlação não muda com essa divisão,
/// porque ela não depende da escala dos dados: a correlação entre altura em
/// metros e peso é a mesma que entre altura em centímetros e peso.
///
/// # Dois cuidados
///
/// 1. **Lista constante → `None`.** Se todos os valores de uma lista são
///    iguais, ela não varia, e não existe "variar junto". A função confere
///    diretamente se "todos os valores são iguais", e não se "desvio padrão
///    = 0". Motivo: para `[0.1, 0.1, 0.1]`, o computador calcula o desvio
///    padrão como 0,000000000000000017 em vez de 0, porque 0,1 não é guardado
///    de forma exata.
/// 2. **Resultado limitado ao intervalo de −1 a 1.** O arredondamento pode
///    dar um valor como 1,0000000000000002, o que é impossível na matemática.
///    O limite corrige esse erro.
///
/// # Quando a resposta é `None`
///
/// - Nos mesmos casos da covariância: tamanhos diferentes, menos de 2 pares,
///   `NaN` ou infinito.
/// - Uma das listas é constante (todos os valores iguais).
///
/// # Complexidade
///
/// - Tempo: O(n). Um número fixo de passadas: conferir os dados, achar o
///   maior valor absoluto, as médias e as três somas.
/// - Espaço: O(1). Nenhuma cópia das listas: a divisão é feita na hora, valor
///   por valor.
pub fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    // Passo 1: as listas precisam ter o mesmo tamanho e pelo menos 2 pares.
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }

    // Passo 2: as médias. Se alguma lista tiver NaN ou infinito, `mean`
    // responde None, e o `?` faz esta função responder None também.
    let mean_x = mean(x)?;
    let mean_y = mean(y)?;

    // Passo 3: lista constante → não existe correlação.
    if is_constant(x) || is_constant(y) {
        return None;
    }

    // Passo 4: normalizar. Como as listas não são constantes, cada uma tem
    // algum valor diferente de 0, e o maior valor absoluto nunca é 0.
    let scale_x = largest_absolute(x);
    let scale_y = largest_absolute(y);

    // O desvio de cada valor já normalizado: x ÷ escala − média ÷ escala.
    // (`|value| ...` é uma função curta, sem nome, guardada numa variável.)
    let deviation_x = |value: f64| value / scale_x - mean_x / scale_x;
    let deviation_y = |value: f64| value / scale_y - mean_y / scale_y;

    // Passo 5: as três somas da fórmula. `zip` anda pelas duas listas ao
    // mesmo tempo, formando os pares (x[0], y[0]), (x[1], y[1]), ...
    let sum_xy: f64 = x
        .iter()
        .zip(y)
        .map(|(&a, &b)| deviation_x(a) * deviation_y(b))
        .sum();
    let sum_xx: f64 = x.iter().map(|&a| deviation_x(a) * deviation_x(a)).sum();
    let sum_yy: f64 = y.iter().map(|&b| deviation_y(b) * deviation_y(b)).sum();

    // Passo 6: r, limitado ao intervalo de −1 a 1 (corrige o arredondamento).
    let r = sum_xy / (sum_xx * sum_yy).sqrt();
    Some(r.clamp(-1.0, 1.0))
}

/// Confere se todos os valores da lista são iguais ao primeiro.
fn is_constant(data: &[f64]) -> bool {
    data.iter().all(|&value| value == data[0])
}

/// O maior valor absoluto da lista (o maior |x|).
fn largest_absolute(data: &[f64]) -> f64 {
    data.iter()
        .fold(0.0, |largest, &value| largest.max(value.abs()))
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
        // y = 2x: relação linear perfeita crescente, r = 1 exato.
        let double: Vec<f64> = X.iter().map(|&value| 2.0 * value).collect();
        assert_eq!(correlation(&X, &double), Some(1.0));
    }

    #[test]
    fn test_perfect_negative() {
        // y = −x: relação linear perfeita decrescente, r = −1 exato.
        let minus: Vec<f64> = X.iter().map(|&value| -value).collect();
        assert_eq!(correlation(&X, &minus), Some(-1.0));
    }

    #[test]
    fn test_does_not_depend_on_scale() {
        // Multiplicar uma das listas por um número positivo não muda r.
        let scaled: Vec<f64> = Y.iter().map(|&value| 1000.0 * value).collect();
        assert_close(correlation(&X, &scaled), correlation(&X, &Y).unwrap());
    }

    #[test]
    fn test_huge_values_do_not_overflow() {
        // (1e308)² vira infinito; com a normalização, r continua certo.
        let x = [-1e308, 0.0, 1e308];
        let y = [1.0, 2.0, 3.0];
        assert_close(correlation(&x, &y), 1.0);
        assert_close(correlation(&x, &x), 1.0);
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

    #[test]
    fn test_tiny_values_do_not_underflow() {
        // Revisão do PR (P3): (1e-200)² = 1e-400 é pequeno demais para o f64 e
        // vira 0. Uma lista com ela mesma tem correlação 1, e não NaN.
        let data = [0.0, 1e-200];
        assert_close(correlation(&data, &data), 1.0);
    }
}
