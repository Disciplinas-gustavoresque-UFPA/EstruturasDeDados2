use super::helpers::{is_valid, select_with_next};

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
/// # Sem ordenar a lista inteira
///
/// A fórmula só precisa de **dois** valores: os das posições i e i + 1 da
/// lista ordenada. Não é preciso ordenar tudo para achá-los: o `quickselect`
/// da biblioteca acha o valor da posição i em tempo linear, e o da posição
/// i + 1 é o menor valor que ficou à direita dele (veja `select_with_next`).
///
/// # Valores perto do limite do `f64`
///
/// Para `[−f64::MAX, f64::MAX]`, a diferença `valor[i + 1] − valor[i]` passa do
/// maior `f64` e vira infinito; e infinito × 0 dá `NaN`. Por isso a conta é
/// feita por `interpolate`, que trata esse caso (veja lá).
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
/// - `p` fora do intervalo de 0 a 1 (ou `p` igual a `NaN`).
///
/// # Complexidade
///
/// - Tempo: **O(n)**. Copiar a lista, o `quickselect` (O(n) no pior caso) e
///   uma passada para o vizinho da direita. Depois, a conta é O(1).
/// - Espaço: O(n), o tamanho da cópia (a lista original não muda).
pub fn quantile(data: &[f64], p: f64) -> Option<f64> {
    // Passo 1: conferir os dados e conferir se p está no intervalo de 0 a 1,
    // `0.0..=1.0` (um p igual a NaN não está em intervalo nenhum: é recusado).
    if !is_valid(data) || !(0.0..=1.0).contains(&p) {
        return None;
    }

    // Passo 2: a posição procurada, separada em parte inteira (i) e decimal (f).
    let (i, f) = position(data.len(), p);

    // Passo 3: os valores das posições i e i + 1, sem ordenar a lista.
    let (low, high) = select_with_next(data, i);

    // Passo 4: andar a fração f do caminho entre os dois.
    Some(interpolate(low, high, f))
}

/// Versão interna: calcula o quantil numa lista **já ordenada**.
///
/// Quem chama garante que a lista está ordenada, não está vazia e que `p`
/// está entre 0 e 1. Assim, quem já precisa da lista ordenada por outro motivo
/// (como o `summary`, por causa da moda) aproveita a ordenação e chama esta
/// função quantas vezes quiser.
///
/// # Complexidade
///
/// - Tempo: O(1). É uma conta só, qualquer que seja o tamanho da lista.
/// - Espaço: O(1).
pub(super) fn quantile_sorted(sorted: &[f64], p: f64) -> f64 {
    // Passo 1: a posição procurada, separada em parte inteira (i) e decimal (f).
    let (i, f) = position(sorted.len(), p);

    // Passo 2: se i é a última posição, não há vizinho à direita.
    if i + 1 >= sorted.len() {
        return sorted[i];
    }

    // Passo 3: andar a fração f do caminho entre valor[i] e valor[i + 1].
    interpolate(sorted[i], sorted[i + 1], f)
}

/// A posição procurada, h = (n − 1) × p, separada em parte inteira (i) e
/// parte decimal (f).
fn position(n: usize, p: f64) -> (usize, f64) {
    let h = (n - 1) as f64 * p;

    // Como h nunca é negativo, converter para inteiro (`as usize`) descarta a
    // parte decimal: é o mesmo que arredondar para baixo.
    let i = h as usize;
    (i, h - i as f64)
}

/// Anda a fração `f` do caminho entre `low` e `high`, com cuidado nos
/// extremos do `f64`.
///
/// ```text
/// f = 0                       → low, sem conta nenhuma (inclui p = 0 e p = 1)
/// caso normal                 → low + f × (high − low)
/// high − low passa do limite  → (1 − f) × low + f × high
/// ```
///
/// As duas fórmulas são iguais na matemática. A segunda não calcula a
/// diferença `high − low`, que é o que estoura quando os vizinhos têm sinais
/// opostos e estão perto de ±f64::MAX. Ela só é usada nesse caso, porque a
/// primeira é a mais precisa (e devolve `low` exato quando `low = high`).
fn interpolate(low: f64, high: f64, f: f64) -> f64 {
    // A posição caiu exatamente sobre um valor: não há o que interpolar.
    if f == 0.0 {
        return low;
    }

    let gap = high - low;
    if gap.is_finite() {
        low + f * gap
    } else {
        (1.0 - f) * low + f * high
    }
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

    #[test]
    fn test_extreme_values_do_not_give_nan() {
        // Revisão do PR (P2): f64::MAX − (−f64::MAX) passa do limite do f64.
        // Mesmo assim, p = 0 é o mínimo e p = 1 é o máximo, sem NaN.
        let data = [-f64::MAX, f64::MAX];
        assert_eq!(quantile(&data, 0.0), Some(-f64::MAX));
        assert_eq!(quantile(&data, 1.0), Some(f64::MAX));
        // No meio do caminho entre −f64::MAX e f64::MAX fica o 0.
        assert_close(quantile(&data, 0.5), 0.0);
        // E o quantil continua entre os dois vizinhos, sem infinito.
        let q = quantile(&data, 0.25).unwrap();
        assert!(q.is_finite() && q < 0.0);
    }

    #[test]
    fn test_agrees_with_sorted_version() {
        // O quantil com quickselect tem que dar exatamente o mesmo que o
        // quantil calculado na lista ordenada pelo merge_sort.
        use super::quantile_sorted;
        use crate::statistics::helpers::sorted_copy;

        let data = [3.5, -2.25, 0.0, 3.5, 10.0, -7.75, 0.1, 3.5, -2.25, 8.0];
        let sorted = sorted_copy(&data);
        for step in 0..=20 {
            let p = step as f64 / 20.0;
            assert_eq!(quantile(&data, p), Some(quantile_sorted(&sorted, p)));
        }
    }
}
