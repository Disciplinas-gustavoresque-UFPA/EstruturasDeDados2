pub fn max<T: Ord + Copy>(elements: &[T]) -> Option<T> {

    let mut max_element: Option<&T> = elements.first();

    for elem in elements.iter().skip(1) {
        if max_element < Some(elem) {
            max_element = Some(elem);
        }
    }
    return max_element.copied();
}

#[cfg(test)]
mod tests {
    use super::max;

    #[test]
    fn test_empty_vec() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(max(&empty_vec), None);
    }

    #[test]
    fn test_single_element_vec() {
        let single_element_vec = vec![42];
        assert_eq!(max(&single_element_vec), Some(42));
    }

    #[test]
    fn test_first_element_is_max() {
        let vec = vec![5, 3, 1, 4, 2];
        assert_eq!(max(&vec), Some(5));
    }

    #[test]
    fn test_last_element_is_max() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(max(&vec), Some(5));
    }

    #[test]
    fn test_middle_element_is_max() {
        let vec = vec![1, 5, 3, 4, 2];
        assert_eq!(max(&vec), Some(5));
    }

    #[test]
    fn test_negative_numbers() {
        let vec = vec![-1, -5, -3, -4, -2];
        assert_eq!(max(&vec), Some(-1));
    }

    #[test]
    fn test_mixed_numbers() {
        let vec = vec![-1, 5, -3, 4, 2];
        assert_eq!(max(&vec), Some(5));
    }

    #[test]
    fn test_duplicate_max_elements() {
        let vec = vec![1, 3, 5, 5, 2];
        assert_eq!(max(&vec), Some(5));
    }
    
    #[test]
    fn test_characters() {
        let vec = vec!['a', 'z', 'm', 'b', 'y'];
        assert_eq!(max(&vec), Some('z'));
    }

    #[test]
    fn test_integer_boundaries() {
        let vec = vec![i32::MIN, 0, i32::MAX];
        assert_eq!(max(&vec), Some(i32::MAX));
    }
}