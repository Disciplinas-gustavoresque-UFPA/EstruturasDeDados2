/// Ordena um slice in-place com o algoritmo Quick Sort.
///
/// O pivô é escolhido pela mediana de três (primeiro, meio e último elemento)
/// e o slice é dividido por uma partição de Lomuto em três vias: menores,
/// iguais e maiores que o pivô. Os elementos iguais ao pivô já ficam na
/// posição final e não entram nas chamadas seguintes.
///
/// A recursão é feita apenas sobre a parte menor; a parte maior é tratada
/// pelo próprio laço. Como a parte menor tem no máximo metade dos elementos,
/// a profundidade da pilha fica limitada a O(log n) para qualquer entrada.
///
/// Se as partições se mostrarem ruins por muitos níveis seguidos (mais de
/// 2·log₂ n), o trecho restante é ordenado com Heap Sort (estratégia do
/// Introsort). Isso evita o pior caso O(n²) em entradas que enganam a
/// mediana de três, como vetores em formato de "montanha".
///
/// Complexidade de tempo: O(n log n) em todos os casos. Vetores com todos os
/// valores iguais são ordenados em O(n).
/// Espaço: O(log n) na pilha de recursão. O algoritmo não é estável.
pub fn quick_sort<T: Ord>(elements: &mut [T]) {
    // 2·⌊log₂ n⌋ + 2: o número de bits de n é ⌊log₂ n⌋ + 1
    let depth_limit = 2 * (usize::BITS - elements.len().leading_zeros()) as usize;
    quick_sort_with_limit(elements, depth_limit);
}

/// Quick Sort com limite de níveis de partição; ao atingir o limite, o trecho
/// restante é ordenado com Heap Sort.
fn quick_sort_with_limit<T: Ord>(mut elements: &mut [T], mut depth_limit: usize) {
    while elements.len() > 1 {
        if depth_limit == 0 {
            heap_sort(elements);
            return;
        }
        depth_limit -= 1;

        median_of_three(elements);
        let (lt, gt) = partition(elements);

        // elements[..lt] < pivô, elements[lt..gt] == pivô, elements[gt..] > pivô
        let (left, rest) = std::mem::take(&mut elements).split_at_mut(lt);
        let right = &mut rest[gt - lt..];

        if left.len() < right.len() {
            quick_sort_with_limit(left, depth_limit);
            elements = right;
        } else {
            quick_sort_with_limit(right, depth_limit);
            elements = left;
        }
    }
}

/// Ordena o slice com Heap Sort, usado como alternativa quando o Quick Sort
/// atinge o limite de profundidade. Complexidade: O(n log n) em qualquer caso,
/// sem recursão.
fn heap_sort<T: Ord>(elements: &mut [T]) {
    let n = elements.len();

    // transforma o slice em um max-heap
    for i in (0..n / 2).rev() {
        sift_down(elements, i, n);
    }

    // move o maior elemento para o fim e restaura o heap no restante
    for end in (1..n).rev() {
        elements.swap(0, end);
        sift_down(elements, 0, end);
    }
}

/// Desce o elemento da posição `root` no heap `elements[..end]` até que ele
/// seja maior ou igual aos seus filhos.
fn sift_down<T: Ord>(elements: &mut [T], mut root: usize, end: usize) {
    loop {
        let mut child = 2 * root + 1;
        if child >= end {
            break;
        }
        if child + 1 < end && elements[child + 1] > elements[child] {
            child += 1;
        }
        if elements[root] >= elements[child] {
            break;
        }
        elements.swap(root, child);
        root = child;
    }
}

/// Move para a última posição do slice a mediana entre o primeiro, o elemento
/// do meio e o último elemento, para que ela seja usada como pivô.
///
/// Evita o pior caso do Quick Sort em vetores já ordenados ou em ordem inversa.
/// Requer um slice não vazio.
fn median_of_three<T: Ord>(elements: &mut [T]) {
    let first = 0;
    let middle = (elements.len() - 1) / 2;
    let last = elements.len() - 1;

    let (a, b, c) = (&elements[first], &elements[middle], &elements[last]);

    if (a <= b && b <= c) || (c <= b && b <= a) {
        elements.swap(middle, last);
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        elements.swap(first, last);
    }
}

/// Particiona o slice em três regiões em relação ao pivô, que deve estar na
/// última posição (partição de Lomuto em três vias).
///
/// A primeira passada move os elementos menores que o pivô para o início; a
/// segunda move os iguais ao pivô para logo depois deles. Os maiores mantêm
/// sua ordem relativa, o que preserva o bom desempenho da mediana de três em
/// entradas já ordenadas ou em ordem inversa.
///
/// Retorna `(lt, gt)` tal que:
/// - `elements[..lt]` contém os elementos menores que o pivô;
/// - `elements[lt..gt]` contém os elementos iguais ao pivô (nunca é vazio);
/// - `elements[gt..]` contém os elementos maiores que o pivô.
///
/// Complexidade: O(n). Requer um slice não vazio.
fn partition<T: Ord>(elements: &mut [T]) -> (usize, usize) {
    // o pivô permanece em elements[last] durante as duas passadas
    let last = elements.len() - 1;

    let mut lt = 0;
    for j in 0..last {
        if elements[j] < elements[last] {
            elements.swap(lt, j);
            lt += 1;
        }
    }

    let mut gt = lt;
    for j in lt..last {
        if elements[j] == elements[last] {
            elements.swap(gt, j);
            gt += 1;
        }
    }

    // coloca o pivô logo após os demais elementos iguais a ele
    elements.swap(gt, last);
    (lt, gt + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_empty_vector() {
        let mut v: Vec<i32> = vec![];
        quick_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn sorts_single_element() {
        let mut v = vec![42];
        quick_sort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn sorts_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sorts_reverse_order() {
        let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sorts_mixed_negatives_and_duplicates() {
        let mut v = vec![5, -3, 0, 12, -3, 7, 5, -10, 0, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![-10, -3, -3, 0, 0, 1, 5, 5, 7, 12]);
    }

    #[test]
    fn sorts_all_equal() {
        let mut v = vec![7; 10];
        quick_sort(&mut v);
        assert_eq!(v, vec![7; 10]);
    }

    #[test]
    fn sorts_integer_limits() {
        let mut v = vec![0, i32::MAX, -1, i32::MIN, 1, i32::MAX, i32::MIN];
        quick_sort(&mut v);
        assert_eq!(v, vec![i32::MIN, i32::MIN, -1, 0, 1, i32::MAX, i32::MAX]);
    }

    #[test]
    fn sorts_chars() {
        let mut v = vec!['r', 'u', 's', 't', 'a', 'c', 'e'];
        quick_sort(&mut v);
        assert_eq!(v, vec!['a', 'c', 'e', 'r', 's', 't', 'u']);
    }

    #[test]
    fn sorts_large_sorted_vector_without_worst_case() {
        let mut v: Vec<i32> = (0..100_000).collect();
        let expected = v.clone();
        quick_sort(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn median_of_three_moves_median_to_last() {
        // mediana no meio
        let mut v = vec![1, 2, 3];
        median_of_three(&mut v);
        assert_eq!(v[2], 2);

        // mediana no início
        let mut v = vec![2, 3, 1];
        median_of_three(&mut v);
        assert_eq!(v[2], 2);

        // mediana já no final
        let mut v = vec![3, 1, 2];
        median_of_three(&mut v);
        assert_eq!(v[2], 2);
    }

    #[test]
    fn median_of_three_with_equal_elements() {
        // Todos iguais — nenhum swap necessário, pivô já é a mediana
        let mut v = vec![2, 2, 2];
        median_of_three(&mut v);
        assert_eq!(v[2], 2);

        // Dois iguais no início
        let mut v = vec![1, 1, 3];
        median_of_three(&mut v);
        assert_eq!(v[2], 1);

        // Dois iguais no fim
        let mut v = vec![1, 3, 3];
        median_of_three(&mut v);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn median_of_three_with_larger_slices() {
        // len=7, middle=(7-1)/2=3
        // elementos considerados: v[0]=10, v[3]=5, v[6]=8
        // mediana entre 10, 5 e 8 é 8 → deve ir para v[6]
        let mut v = vec![10, 99, 99, 5, 99, 99, 8];
        median_of_three(&mut v);
        assert_eq!(v[6], 8);
    }

    #[test]
    fn median_of_three_already_in_last() {
        // mediana já está na última posição — nenhum swap deve ocorrer
        let mut v = vec![3, 1, 2]; // mediana é 2, já está em v[2]
        let before = v.clone();
        median_of_three(&mut v);
        assert_eq!(v[2], 2);
        assert_eq!(v, before);
    }

    /// Verifica as três regiões da partição: < pivô, == pivô e > pivô.
    fn assert_partitioned(v: &[i32], lt: usize, gt: usize, pivot: i32) {
        assert!(lt < gt && gt <= v.len(), "limites inválidos: ({lt}, {gt})");
        assert!(v[..lt].iter().all(|&x| x < pivot));
        assert!(v[lt..gt].iter().all(|&x| x == pivot));
        assert!(v[gt..].iter().all(|&x| x > pivot));
    }

    #[test]
    fn partition_places_pivot_in_final_position() {
        let mut v = vec![3, 8, 1, 9, 5]; // pivô = 5
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (2, 3));
        assert_eq!(v, vec![3, 1, 5, 9, 8]);
    }

    #[test]
    fn partition_pivot_is_smallest() {
        // Pivô (último elemento) é o menor — deve ir para o índice 0
        let mut v = vec![5, 9, 3, 7, 1]; // pivô = 1
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (0, 1));
        assert_partitioned(&v, lt, gt, 1);
    }

    #[test]
    fn partition_pivot_is_largest() {
        // Pivô (último elemento) é o maior — deve ir para o último índice
        let mut v = vec![3, 1, 4, 2, 9]; // pivô = 9
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (4, 5));
        assert_partitioned(&v, lt, gt, 9);
    }

    #[test]
    fn partition_with_all_equal_elements() {
        // Todos iguais ao pivô: a região dos iguais ocupa o slice inteiro,
        // e não sobra nada para as chamadas recursivas
        let mut v = vec![4, 4, 4, 4];
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (0, 4));
    }

    #[test]
    fn partition_groups_duplicates_of_pivot() {
        let mut v = vec![5, 2, 5, 8, 5, 1, 9, 5]; // pivô = 5
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (2, 6));
        assert_partitioned(&v, lt, gt, 5);
    }

    #[test]
    fn partition_two_elements() {
        let mut v = vec![2, 1]; // pivô = 1, menor que 2 → vai para o índice 0
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (0, 1));
        assert_eq!(v, vec![1, 2]);

        let mut v = vec![1, 2]; // pivô = 2, maior → vai para o índice 1
        let (lt, gt) = partition(&mut v);
        assert_eq!((lt, gt), (1, 2));
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn partition_invariant_holds() {
        // Propriedade principal: < pivô à esquerda, == no meio, > à direita
        let mut v = vec![7, 2, 9, 4, 6, 1, 3, 6, 6]; // pivô = 6
        let (lt, gt) = partition(&mut v);
        assert_partitioned(&v, lt, gt, 6);
    }

    #[test]
    fn heap_sort_sorts_various_inputs() {
        let casos: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2, 1],
            vec![5, -3, 0, 12, -3, 7, 5, -10, 0, 1],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![4; 6],
        ];
        for mut v in casos {
            let mut esperado = v.clone();
            esperado.sort();
            heap_sort(&mut v);
            assert_eq!(v, esperado);
        }
    }

    #[test]
    fn depth_limit_zero_falls_back_to_heap_sort() {
        // com limite 0, nenhuma partição é feita e o Heap Sort ordena tudo
        let mut v = vec![3, 8, 1, 9, 5, 2, 7];
        quick_sort_with_limit(&mut v, 0);
        assert_eq!(v, vec![1, 2, 3, 5, 7, 8, 9]);
    }

    // ── entradas que causavam estouro de pilha (revisão do PR) ──────────────

    fn assert_sorted(v: &[i32]) {
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn sorts_many_equal_values() {
        let mut v = vec![7; 100_000];
        quick_sort(&mut v);
        assert_eq!(v, vec![7; 100_000]);
    }

    #[test]
    fn sorts_many_zeros_and_ones() {
        let mut v: Vec<i32> = (0..100_000).map(|i| (i * 7 % 3 == 0) as i32).collect();
        let zeros = v.iter().filter(|&&x| x == 0).count();
        quick_sort(&mut v);
        assert_sorted(&v);
        assert!(v[..zeros].iter().all(|&x| x == 0));
    }

    #[test]
    fn sorts_mountain_shaped_vector() {
        // sobe e depois desce, sem valores repetidos: 0, 2, 4, ..., 5, 3, 1
        let n = 100_000;
        let mut v: Vec<i32> = (0..n).step_by(2).chain((1..n).step_by(2).rev()).collect();
        quick_sort(&mut v);
        assert_eq!(v, (0..n).collect::<Vec<i32>>());
    }

    #[test]
    fn sorts_few_distinct_values() {
        // notas de 0 a 10, como no teste feito no navegador
        let mut v: Vec<i32> = (0..100_000).map(|i| (i * 37 + 11) % 11).collect();
        quick_sort(&mut v);
        assert_sorted(&v);
    }

    #[test]
    fn sorts_strings() {
        let mut v: Vec<String> = ["pera", "uva", "abacaxi", "uva", "banana"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        quick_sort(&mut v);
        assert_eq!(v, vec!["abacaxi", "banana", "pera", "uva", "uva"]);
    }
}
