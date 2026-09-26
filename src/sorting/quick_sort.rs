pub fn quick_sort<T: Ord>(elements: &mut [T]) {
    if(elements.len() <= 1){
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


fn partition<T: Ord>(elements: &mut [T]) -> usize{
    let last = elements.len() - 1;
    let mut i = 0;
    for j in 0..last{
        if(elements[j] <= elements[last]){
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
    fn partition_places_pivot_in_final_position() {
        let mut v = vec![3, 8, 1, 9, 5];
        let p = partition(&mut v);
        assert_eq!(p, 2);
        assert_eq!(v, vec![3, 1, 5, 9, 8]);
    }
} 