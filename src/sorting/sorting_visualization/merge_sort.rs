pub fn merge_sort<T: Ord + Clone>(elements: &mut [T]) {
    if elements.len() <= 1 {
        return;
    }

    let middle = elements.len() / 2;

    merge_sort(&mut elements[..middle]);
    merge_sort(&mut elements[middle..]);

    let left = elements[..middle].to_vec();
    let right = elements[middle..].to_vec();

    let mut left_index = 0;
    let mut right_index = 0;
    let mut elements_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            elements[elements_index] = left[left_index].clone();
            left_index += 1;
        } else {
            elements[elements_index] = right[right_index].clone();
            right_index += 1;
        }

        elements_index += 1;
    }

    while left_index < left.len() {
        elements[elements_index] = left[left_index].clone();
        left_index += 1;
        elements_index += 1;
    }

    while right_index < right.len() {
        elements[elements_index] = right[right_index].clone();
        right_index += 1;
        elements_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        merge_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values = [42];
        merge_sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5];
        merge_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        merge_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
        merge_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = [-3, 0, -10, 7, -1, 2];
        merge_sort(&mut values);
        assert_eq!(values, [-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        merge_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7, 7, 7, 7, 7];
        merge_sort(&mut values);
        assert_eq!(values, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = [i32::MAX, i32::MIN, 0, -1, 1];
        merge_sort(&mut values);
        assert_eq!(values, [i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'a', 'c', 'b'];
        merge_sort(&mut values);
        assert_eq!(values, ['a', 'b', 'c', 'd']);
    }
}