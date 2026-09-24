pub fn min<T: Ord + Copy>(elements: &[T]) -> Option<T> {

    let mut min_element: Option<&T> = elements.first();

    for num in elements.iter().skip(1) {
        if min_element > Some(num) {
            min_element = Some(num);
        }
    }
    return min_element.copied();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(min(&empty_vec), None);
    }

    #[test]
    fn test_single_element_vec() {
        let single_element_vec = vec![42];
        assert_eq!(min(&single_element_vec), Some(42));
    }

    #[test]
    fn test_first_element_is_min() {
        let vec = vec![1, 3, 5, 4, 2];
        assert_eq!(min(&vec), Some(1));
    }

    #[test]
    fn test_last_element_is_min() {
        let vec = vec![5, 4, 3, 2, 1];
        assert_eq!(min(&vec), Some(1));
    }

    #[test]
    fn test_middle_element_is_min() {
        let vec = vec![5, 1, 3, 4, 2];
        assert_eq!(min(&vec), Some(1));
    }

    #[test]
    fn test_negative_numbers() {
        let vec = vec![-1, -5, -3, -4, -2];
        assert_eq!(min(&vec), Some(-5));
    }

    #[test]
    fn test_mixed_numbers() {
        let vec = vec![-1, 5, -3, 4, 2];
        assert_eq!(min(&vec), Some(-3));
    }

    #[test]
    fn test_duplicate_min_values() {
        let vec = vec![5, 3, 1, 2, 1];
        assert_eq!(min(&vec), Some(1));
    }

    #[test]
    fn test_characters() {
        let vec = vec!['a', 'z', 'm', 'b', 'y'];
        assert_eq!(min(&vec), Some('a'));
    }

    #[test]
    fn test_integer_boundaries() {
        let vec = vec![i32::MIN, 0, i32::MAX];
        assert_eq!(min(&vec), Some(i32::MIN));
    }
}