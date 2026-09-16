pub fn bubble_sort<T: Ord>(elements: &mut Vec<T>) {    
    let n = elements.len();
    for i in 1..n {
        for j in 0..n - i {
            if elements[j] > elements[j + 1] {
                elements.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_empty() {
        let mut values: Vec<i32> = vec![];
        bubble_sort(&mut values);
        assert_eq!(values, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut values = vec![42];
        bubble_sort(&mut values);
        assert_eq!(values, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut values = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut values);
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut values);
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = vec![9, 1, 5, 3, 7, 2, 8, 4, 6];
        bubble_sort(&mut values);
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = vec![-3, 0, -10, 7, -1, 2];
        bubble_sort(&mut values);
        assert_eq!(values, vec![-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = vec![4, 2, 2, 1, 3, 4, 1];
        bubble_sort(&mut values);
        assert_eq!(values, vec![1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = vec![7, 7, 7, 7, 7];
        bubble_sort(&mut values);
        assert_eq!(values, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = vec![i32::MAX, i32::MIN, 0, -1, 1];
        bubble_sort(&mut values);
        assert_eq!(values, vec![i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_characters() {
        let mut values = vec!['d', 'a', 'c', 'b'];
        bubble_sort(&mut values);
        assert_eq!(values, vec!['a', 'b', 'c', 'd']);
    }
}