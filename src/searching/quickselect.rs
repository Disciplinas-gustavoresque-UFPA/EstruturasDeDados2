use crate::sorting::insertion_sort;

/// Acha o **k-ésimo menor** elemento (contando a partir de 0) sem ordenar a
/// lista inteira. Responde `None` quando `k` não é uma posição da lista.
///
/// Depois da chamada, a lista fica reorganizada assim:
///
/// ```text
/// [ valores ≤ elements[k] | elements[k] | valores ≥ elements[k] ]
///          posições 0..k          k          posições k+1..n
/// ```
///
/// Ou seja, `elements[k]` é exatamente o valor que estaria na posição `k` se a
/// lista fosse ordenada. As duas partes, porém, ficam fora de ordem.
///
/// # Como funciona (Quickselect com a "mediana das medianas")
///
/// 1. Escolhe um **pivô** pela mediana das medianas (veja `median_of_medians`).
/// 2. Separa a lista em três partes: menores que o pivô, iguais e maiores.
/// 3. Continua **só na parte onde está a posição k**. As outras partes nunca
///    mais são visitadas. É isso que o diferencia do Quicksort, que ordena as
///    duas metades.
///
/// # Complexidade
///
/// - Tempo: **O(n) no pior caso.** A mediana das medianas garante que cada
///   rodada descarta pelo menos 3/10 da lista, o que dá a recorrência
///   T(n) ≤ T(n/5) + T(7n/10) + O(n). Como 1/5 + 7/10 = 9/10 < 1, o trabalho
///   de cada nível da recursão encolhe 10%: n + 0,9n + 0,81n + ... ≤ 10n.
/// - Espaço: O(log n), a pilha das chamadas recursivas da mediana das
///   medianas (cada uma recebe 1/5 da lista anterior).
pub fn quickselect<T: Ord + Copy>(elements: &mut [T], k: usize) -> Option<T> {
    if k >= elements.len() {
        return None;
    }

    // A posição k está sempre dentro da faixa elements[lo..hi].
    let mut lo = 0;
    let mut hi = elements.len();

    loop {
        // Faixa pequena: ordenar direto é mais barato (no máximo 5 elementos).
        if hi - lo <= 5 {
            insertion_sort(&mut elements[lo..hi]);
            return Some(elements[k]);
        }

        // Escolher o pivô e separar a faixa em  < pivô | = pivô | > pivô.
        let part = &mut elements[lo..hi];
        let pivot = median_of_medians(part);
        let (less_end, greater_start) = partition_three_ways(part, pivot);

        // Continuar só na parte que contém a posição k.
        if k < lo + less_end {
            hi = lo + less_end;
        } else if k < lo + greater_start {
            return Some(pivot);
        } else {
            lo += greater_start;
        }
    }
}

/// Escolhe um pivô que, com certeza, não fica perto das pontas.
///
/// Divide a lista em grupos de 5, acha a mediana de cada grupo e devolve a
/// mediana dessas medianas (achada pelo próprio `quickselect`, recursivamente).
/// Pelo menos 3/10 dos elementos são ≤ a ela, e pelo menos 3/10 são ≥ a ela.
///
/// As medianas dos grupos são levadas para o começo da lista, para que a
/// chamada recursiva trabalhe em `elements[..count]`, sem lista extra.
fn median_of_medians<T: Ord + Copy>(elements: &mut [T]) -> T {
    let mut count = 0;
    let mut start = 0;
    while start < elements.len() {
        let end = (start + 5).min(elements.len());
        insertion_sort(&mut elements[start..end]);
        let group_median = start + (end - start - 1) / 2;
        elements.swap(count, group_median);
        count += 1;
        start = end;
    }

    let middle = (count - 1) / 2;
    quickselect(&mut elements[..count], middle).expect("middle < count")
}

/// Separa a lista em três partes, em torno do pivô (a "bandeira holandesa"):
///
/// ```text
/// [ < pivô | = pivô | > pivô ]
///           ^        ^
///      less_end  greater_start
/// ```
///
/// Separar os iguais numa parte própria garante que a lista encolhe a cada
/// rodada, mesmo quando há muitos valores repetidos.
fn partition_three_ways<T: Ord + Copy>(elements: &mut [T], pivot: T) -> (usize, usize) {
    let mut less_end = 0;
    let mut i = 0;
    let mut greater_start = elements.len();

    while i < greater_start {
        if elements[i] < pivot {
            elements.swap(less_end, i);
            less_end += 1;
            i += 1;
        } else if elements[i] > pivot {
            greater_start -= 1;
            elements.swap(i, greater_start);
        } else {
            i += 1;
        }
    }

    (less_end, greater_start)
}

#[cfg(test)]
mod tests {
    use super::quickselect;
    use crate::sorting::merge_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        assert_eq!(quickselect(&mut values, 0), None);
    }

    #[test]
    fn test_k_out_of_range() {
        let mut values = [3, 1, 2];
        assert_eq!(quickselect(&mut values, 3), None);
    }

    #[test]
    fn test_single_element() {
        let mut values = [42];
        assert_eq!(quickselect(&mut values, 0), Some(42));
    }

    #[test]
    fn test_every_position() {
        // Ordenado seria [1, 2, 3, 4, 5, 6, 7, 8, 9]: a posição k guarda k + 1.
        for k in 0..9 {
            let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
            assert_eq!(quickselect(&mut values, k), Some(k as i32 + 1));
        }
    }

    #[test]
    fn test_duplicates_and_negatives() {
        let mut values = [4, -2, 2, 4, 1, -2, 3, 4, 1, 0, 4, -7];
        // Ordenado: [-7, -2, -2, 0, 1, 1, 2, 3, 4, 4, 4, 4]
        assert_eq!(quickselect(&mut values, 0), Some(-7));
        assert_eq!(quickselect(&mut values, 5), Some(1));
        assert_eq!(quickselect(&mut values, 11), Some(4));
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7; 50];
        assert_eq!(quickselect(&mut values, 25), Some(7));
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'a', 'c', 'b'];
        assert_eq!(quickselect(&mut values, 2), Some('c'));
    }

    #[test]
    fn test_agrees_with_merge_sort() {
        // Listas "aleatórias" (sempre as mesmas, para o teste ser repetível) de
        // vários tamanhos: o resultado tem que bater com a lista ordenada, e a
        // lista tem que ficar separada em  ≤ elements[k] | elements[k] | ≥.
        let mut seed: u64 = 12345;
        for n in 1..120 {
            let mut original: Vec<i64> = Vec::new();
            for _ in 0..n {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                original.push((seed >> 33) as i64 % 50 - 25);
            }
            let mut sorted = original.clone();
            merge_sort(&mut sorted);

            for k in 0..n {
                let mut values = original.clone();
                assert_eq!(quickselect(&mut values, k), Some(sorted[k]));
                assert!(values[..k].iter().all(|&v| v <= values[k]));
                assert!(values[k + 1..].iter().all(|&v| v >= values[k]));
            }
        }
    }
}
