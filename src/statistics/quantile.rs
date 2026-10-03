use super::helpers::{is_valid, sorted_copy};

/// Calcula o **quantil p** de uma lista de números.
///
/// # O que é
///
/// É o valor abaixo do qual fica a fração `p` dos dados ordenados. Exemplos:
///
/// ```text
/// p = 0,25 → primeiro quartil (Q1): um quarto dos dados fica abaixo
/// p = 0,5  → mediana: metade dos dados fica abaixo
/// p = 0,75 → terceiro quartil (Q3): três quartos dos dados ficam abaixo
/// ```
///
/// # Método: interpolação linear entre vizinhos
///
/// ```text
/// 1. Ordene a lista. As posições são contadas a partir de 0.
/// 2. Calcule a posição procurada: h = (n − 1) × p
/// 3. Separe h em parte inteira (i) e parte decimal (f).
/// 4. Se i é a última posição, o quantil é o valor da posição i.
/// 5. Senão: quantil = valor[i] + f × (valor[i + 1] − valor[i])
///    (ande a fração f do caminho entre os dois vizinhos)
/// ```
///
/// # Exemplo, passo a passo (Q3, p = 0,75)
///
/// ```text
/// posição:  0   1   2   3   4   5
/// valor:    2   4   4   5   7   9
///
/// h = (6 − 1) × 0,75 = 3,75   →   i = 3,  f = 0,75
/// quantil = valor[3] + 0,75 × (valor[4] − valor[3])
///         = 5 + 0,75 × (7 − 5)
///         = 6,5
/// ```
///
/// Casos especiais: `p = 0` dá o mínimo, `p = 1` dá o máximo, e uma lista com
/// um único valor dá o próprio valor.
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
/// - `p` fora do intervalo de 0 a 1 (ou `p` igual a `NaN`).
///
/// # Complexidade
///
/// - Tempo: O(n log n), o custo de ordenar a cópia. Depois de ordenar, achar
///   o quantil é uma conta só: O(1).
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn quantile(data: &[f64], p: f64) -> Option<f64> {
    // Passo 1: conferir os dados e conferir se p está entre 0 e 1
    // (um p igual a NaN também é recusado).
    if !is_valid(data) || p.is_nan() || p < 0.0 || p > 1.0 {
        return None;
    }

    // Passo 2: ordenar uma cópia (a lista original não muda).
    let sorted = sorted_copy(data);

    // Passo 3: calcular o quantil na lista ordenada.
    Some(quantile_sorted(&sorted, p))
}

/// Versão interna: calcula o quantil numa lista **já ordenada**.
///
/// Quem chama garante que a lista está ordenada, não está vazia e que `p`
/// está entre 0 e 1. Assim, quem precisa de vários quantis (como os quartis)
/// ordena **uma vez só** e chama esta função quantas vezes quiser.
///
/// # Complexidade
///
/// - Tempo: O(1). É uma conta só, qualquer que seja o tamanho da lista.
/// - Espaço: O(1).
pub(super) fn quantile_sorted(sorted: &[f64], p: f64) -> f64 {
    // Passo 1: a posição procurada, h = (n − 1) × p.
    let h = (sorted.len() - 1) as f64 * p;

    // Passo 2: separar h em parte inteira (i) e parte decimal (f).
    // Como h nunca é negativo, converter para inteiro (`as usize`) descarta a
    // parte decimal: é o mesmo que arredondar para baixo.
    let i = h as usize;
    let f = h - i as f64;

    // Passo 3: se i é a última posição, não há vizinho à direita.
    if i + 1 >= sorted.len() {
        return sorted[i];
    }

    // Passo 4: andar a fração f do caminho entre valor[i] e valor[i + 1].
    sorted[i] + f * (sorted[i + 1] - sorted[i])
}

#[cfg(test)]
mod tests {
    use super::quantile;
    use crate::statistics::helpers::assert_close;

    const X: [f64; 6] = [2.0, 4.0, 4.0, 5.0, 7.0, 9.0];

    #[test]
    fn test_main_example() {
        // Q3: h = 5 × 0,75 = 3,75 → 5 + 0,75 × (7 − 5) = 6,5
        assert_close(quantile(&X, 0.75), 6.5);
        // Q1: h = 5 × 0,25 = 1,25 → 4 + 0,25 × (4 − 4) = 4
        assert_close(quantile(&X, 0.25), 4.0);
        // Mediana: h = 5 × 0,5 = 2,5 → 4 + 0,5 × (5 − 4) = 4,5
        assert_close(quantile(&X, 0.5), 4.5);
    }

    #[test]
    fn test_extremes_are_min_and_max() {
        // p = 0 → mínimo; p = 1 → máximo
        assert_close(quantile(&X, 0.0), 2.0);
        assert_close(quantile(&X, 1.0), 9.0);
    }

    #[test]
    fn test_unsorted_input() {
        // A função ordena uma cópia antes: o resultado é o mesmo.
        assert_close(quantile(&[9.0, 4.0, 2.0, 7.0, 4.0, 5.0], 0.75), 6.5);
    }

    #[test]
    fn test_even_length() {
        // Q1 de [1, 2, 3, 4]: h = 3 × 0,25 = 0,75 → 1 + 0,75 × (2 − 1) = 7/4
        assert_close(quantile(&[1.0, 2.0, 3.0, 4.0], 0.25), 7.0 / 4.0);
    }

    #[test]
    fn test_single_value() {
        // Um valor só: qualquer quantil é ele mesmo.
        assert_close(quantile(&[5.0], 0.3), 5.0);
    }

    #[test]
    fn test_p_out_of_range() {
        // p precisa estar entre 0 e 1.
        assert_eq!(quantile(&X, 1.5), None);
        assert_eq!(quantile(&X, -0.1), None);
        assert_eq!(quantile(&X, f64::NAN), None);
    }

    #[test]
    fn test_original_order_is_kept() {
        // A lista de quem chamou não muda de ordem.
        let data = vec![9.0, 2.0, 5.0];
        let _ = quantile(&data, 0.5);
        assert_eq!(data, vec![9.0, 2.0, 5.0]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(quantile(&[], 0.5), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(quantile(&[1.0, f64::NAN, 3.0], 0.5), None);
    }
}
