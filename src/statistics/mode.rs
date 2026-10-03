use super::helpers::{is_valid, sorted_copy};

/// Calcula a **moda**: o valor (ou os valores) que mais se repete(m).
///
/// # O que é
///
/// A moda é o valor mais frequente dos dados. Diferente da média e da mediana,
/// pode haver **mais de uma moda** (quando há empate) ou **nenhuma** (quando
/// nenhum valor se destaca).
///
/// # Como é calculada
///
/// ```text
/// 1. Ordena uma cópia: valores iguais ficam lado a lado, formando "blocos".
///    [2, 4, 4, 5, 7, 9] → [2 | 4 4 | 5 | 7 | 9]
/// 2. Conta o tamanho de cada bloco: 2→1  4→2  5→1  7→1  9→1
/// 3. A maior contagem é 2 → moda = [4]
/// ```
///
/// # Regras
///
/// ```text
/// [2, 4, 4, 5, 7, 9] → [4]      um valor ganha
/// [1, 1, 2, 2, 3]    → [1, 2]   empate entre os mais frequentes (bimodal)
/// [1, 2, 3]          → []       todos aparecem 1 vez: nenhuma moda
/// [1, 1, 2, 2]       → []       todos empatam: nenhuma moda
/// [7, 7, 7]          → [7]      só existe um valor
/// ```
///
/// Regra geral: se há mais de um valor distinto e **todos** aparecem o mesmo
/// número de vezes, não há moda (a resposta é a lista vazia). Senão, a moda são
/// os valores de maior frequência, em ordem crescente.
///
/// # `None` e lista vazia são coisas diferentes
///
/// - `None` quer dizer "não há dados" (lista vazia, `NaN` ou infinito).
/// - `Some(vec![])` quer dizer "há dados, mas nenhuma moda".
///
/// # Cuidado com números decimais
///
/// A moda compara os valores de forma **exata**. O computador guarda
/// `0.1 + 0.2` como `0.30000000000000004`, que é diferente de `0.3`. Por isso
/// a moda é indicada para dados **discretos**, como contagens e notas.
///
/// # Complexidade
///
/// - Tempo: O(n log n). Ordenar custa O(n log n); contar os blocos é uma
///   passada, O(n). A parcela maior é a que manda.
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn mode(data: &[f64]) -> Option<Vec<f64>> {
    // Passo 1: conferir os dados.
    if !is_valid(data) {
        return None;
    }

    // Passo 2: ordenar uma cópia, para que valores iguais fiquem lado a lado.
    let sorted = sorted_copy(data);

    // Passo 3: calcular a moda na lista ordenada.
    Some(mode_sorted(&sorted))
}

/// Versão interna: calcula a moda numa lista **já ordenada** e não vazia.
///
/// # Complexidade
///
/// - Tempo: O(n). Cada posição da lista é visitada uma vez.
/// - Espaço: O(n) no pior caso (quando todos os valores são diferentes, há
///   um bloco para cada valor).
pub(super) fn mode_sorted(sorted: &[f64]) -> Vec<f64> {
    // Passo 1: montar os blocos de valores iguais, guardando
    // (valor, quantas vezes ele aparece).
    let mut blocks: Vec<(f64, usize)> = Vec::new();
    let mut i = 0;
    while i < sorted.len() {
        let value = sorted[i];
        let mut count = 0;
        // Avança enquanto o valor se repete.
        while i < sorted.len() && sorted[i] == value {
            count += 1;
            i += 1;
        }
        blocks.push((value, count));
    }

    // Passo 2: descobrir a maior contagem.
    let mut highest = 0;
    for &(_, count) in &blocks {
        if count > highest {
            highest = count;
        }
    }

    // Passo 3: se há mais de um bloco e todos têm o mesmo tamanho,
    // nenhum valor se destaca → nenhuma moda.
    let mut all_tied = true;
    for &(_, count) in &blocks {
        if count != highest {
            all_tied = false;
        }
    }
    if blocks.len() > 1 && all_tied {
        return Vec::new();
    }

    // Passo 4: a moda são os valores dos blocos com a maior contagem.
    // Eles já saem em ordem crescente, porque a lista estava ordenada.
    let mut modes: Vec<f64> = Vec::new();
    for &(value, count) in &blocks {
        if count == highest {
            modes.push(value);
        }
    }
    modes
}

#[cfg(test)]
mod tests {
    use super::mode;

    #[test]
    fn test_main_example() {
        // [2 | 4 4 | 5 | 7 | 9] → o 4 aparece 2 vezes
        assert_eq!(mode(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]), Some(vec![4.0]));
    }

    #[test]
    fn test_bimodal() {
        // 1 e 2 aparecem 2 vezes cada; 3 aparece 1 vez
        assert_eq!(mode(&[1.0, 1.0, 2.0, 2.0, 3.0]), Some(vec![1.0, 2.0]));
    }

    #[test]
    fn test_all_different_has_no_mode() {
        // Todos aparecem 1 vez: nenhuma moda
        assert_eq!(mode(&[1.0, 2.0, 3.0]), Some(vec![]));
    }

    #[test]
    fn test_full_tie_has_no_mode() {
        // 1 e 2 aparecem 2 vezes cada: todos empatam, nenhuma moda
        assert_eq!(mode(&[1.0, 1.0, 2.0, 2.0]), Some(vec![]));
    }

    #[test]
    fn test_all_equal() {
        // Só existe um valor: ele é a moda
        assert_eq!(mode(&[7.0, 7.0, 7.0]), Some(vec![7.0]));
    }

    #[test]
    fn test_single_value() {
        assert_eq!(mode(&[5.0]), Some(vec![5.0]));
    }

    #[test]
    fn test_unsorted_input() {
        // [4, 1, 4, 2] ordenado é [1 | 2 | 4 4] → 4
        assert_eq!(mode(&[4.0, 1.0, 4.0, 2.0]), Some(vec![4.0]));
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(mode(&[-1.0, 2.0, -1.0]), Some(vec![-1.0]));
    }

    #[test]
    fn test_empty() {
        // Sem dados: None (diferente de "nenhuma moda")
        assert_eq!(mode(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(mode(&[1.0, f64::NAN, 1.0]), None);
    }
}
