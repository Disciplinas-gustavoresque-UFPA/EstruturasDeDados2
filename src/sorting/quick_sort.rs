pub fn quick_sort<T: Ord>(elements: &mut [T]) {
    if (elements.len() <= 1) {
        return;
    }

    median_of_three(elements);
    let p = partition(elements);

    quick_sort(&mut elements[..p]);
    quick_sort(&mut elements[p + 1..]);
}

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

fn partition<T: Ord>(elements: &mut [T]) -> usize {
    let last = elements.len() - 1;
    let mut i = 0;
    for j in 0..last {
        if (elements[j] <= elements[last]) {
            elements.swap(i, j);
            i += 1;
        }
    }

    elements.swap(i, last);
    i
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
        // os outros elementos podem ter mudado de lugar, mas o pivô está certo
    }

    #[test]
    fn partition_places_pivot_in_final_position() {
        let mut v = vec![3, 8, 1, 9, 5];
        let p = partition(&mut v);
        assert_eq!(p, 2);
        assert_eq!(v, vec![3, 1, 5, 9, 8]);
    }

    #[test]
    fn partition_pivot_is_smallest() {
        // Pivô (último elemento) é o menor — deve ir para índice 0
        let mut v = vec![5, 9, 3, 7, 1]; // pivô = 1
        let p = partition(&mut v);
        assert_eq!(p, 0);
        assert_eq!(v[p], 1);
        // Todos os outros devem ser >= 1
        assert!(v[1..].iter().all(|&x| x >= 1));
    }

    #[test]
    fn partition_pivot_is_largest() {
        // Pivô (último elemento) é o maior — deve ir para o último índice
        let mut v = vec![3, 1, 4, 2, 9]; // pivô = 9
        let p = partition(&mut v);
        assert_eq!(p, 4);
        assert_eq!(v[p], 9);
        // Todos os anteriores devem ser <= 9
        assert!(v[..p].iter().all(|&x| x <= 9));
    }

    #[test]
    fn partition_with_all_equal_elements() {
        let mut v = vec![4, 4, 4, 4];
        let p = partition(&mut v);
        // Com todos iguais, i avança para cada elemento → pivô vai para o final
        assert_eq!(v[p], 4);
        assert!(v.iter().all(|&x| x == 4));
    }

    #[test]
    fn partition_two_elements() {
        let mut v = vec![2, 1]; // pivô = 1, menor que 2 → vai para índice 0
        let p = partition(&mut v);
        assert_eq!(v[p], 1);
        assert_eq!(p, 0);

        let mut v = vec![1, 2]; // pivô = 2, maior → vai para índice 1
        let p = partition(&mut v);
        assert_eq!(v[p], 2);
        assert_eq!(p, 1);
    }

    #[test]
    fn partition_invariant_holds() {
        // Propriedade principal: tudo à esquerda <= pivô, tudo à direita >= pivô
        let mut v = vec![7, 2, 9, 4, 6, 1, 3];
        let p = partition(&mut v);
        let pivot = v[p];
        assert!(v[..p].iter().all(|&x| x <= pivot));
        assert!(v[p + 1..].iter().all(|&x| x >= pivot));
    }
}
