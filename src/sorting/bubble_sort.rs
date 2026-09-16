pub fn bubble_sort<T: Ord + Copy>(elements: &Vec<T>) -> Vec<T> {
    let mut elements_copy = elements.clone();
    let n = elements_copy.len();

    for i in 1..n {
        for j in 0..n - i {
            if elements_copy[j] > elements_copy[j + 1] {
                elements_copy.swap(j, j + 1);
            }
        }
    }
    return elements_copy;
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_empty() {
        let values: Vec<i32> = vec![];
        assert_eq!(bubble_sort(&values), vec![]);
    }

    #[test]
    fn test_single_element() {
        let values = vec![42];
        assert_eq!(bubble_sort(&values), vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let values = vec![1, 2, 3, 4, 5];
        assert_eq!(bubble_sort(&values), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let values = vec![5, 4, 3, 2, 1];
        assert_eq!(bubble_sort(&values), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let values = vec![9, 1, 5, 3, 7, 2, 8, 4, 6];
        assert_eq!(bubble_sort(&values), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let values = vec![-3, 0, -10, 7, -1, 2];
        assert_eq!(bubble_sort(&values), vec![-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let values = vec![4, 2, 2, 1, 3, 4, 1];
        assert_eq!(bubble_sort(&values), vec![1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let values = vec![7, 7, 7, 7, 7];
        assert_eq!(bubble_sort(&values), vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let values = vec![i32::MAX, i32::MIN, 0, -1, 1];
        assert_eq!(bubble_sort(&values), vec![i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_keeps_original_vector_unchanged() {
        let values = vec![3, 1, 2];
        let sorted = bubble_sort(&values);

        assert_eq!(values, vec![3, 1, 2]);
        assert_eq!(sorted, vec![1, 2, 3]);
    }

    #[test]
    fn test_characters() {
        let values = vec!['d', 'a', 'c', 'b'];
        assert_eq!(bubble_sort(&values), vec!['a', 'b', 'c', 'd']);
    }
}