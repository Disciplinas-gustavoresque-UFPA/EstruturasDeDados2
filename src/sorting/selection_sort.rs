pub fn selection_sort<T: Ord + Copy>(elements: &mut [T]) {
    for i in 0..elements.len() {
        let mut min_idx = i;
        for j in i..elements.len() {
            if elements[min_idx] > elements[j] {
                min_idx = j;
            }
        }
        elements.swap(min_idx, i);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        selection_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values = [42];
        selection_sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5];
        selection_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        selection_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
        selection_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = [-3, 0, -10, 7, -1, 2];
        selection_sort(&mut values);
        assert_eq!(values, [-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        selection_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7, 7, 7, 7, 7];
        selection_sort(&mut values);
        assert_eq!(values, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = [i32::MAX, i32::MIN, 0, -1, 1];
        selection_sort(&mut values);
        assert_eq!(values, [i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'a', 'c', 'b'];
        selection_sort(&mut values);
        assert_eq!(values, ['a', 'b', 'c', 'd']);
    }
}
