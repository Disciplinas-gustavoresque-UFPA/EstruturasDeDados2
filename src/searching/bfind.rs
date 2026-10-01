pub fn bfind<T: Eq + Ord>(elements: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = elements.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if &elements[mid] == target {
            return Some(mid);
        }

        if &elements[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::bfind;

    #[test]
    fn test_empty() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(bfind(&[], &3), None);
        assert_eq!(bfind(&values, &0), None);
    }

    #[test]
    fn test_single_element() {
        let values = [42];
        assert_eq!(bfind(&values, &42), Some(0));
        assert_eq!(bfind(&values, &41), None);
    }

    #[test]
    fn test_first_element() {
        let values = [1, 3, 5, 7, 9, 11, 13];
        assert_eq!(bfind(&values, &1), Some(0));
    }

    #[test]
    fn test_middle_element() {
        let values = [1, 3, 5, 7, 9, 11, 13];
        assert_eq!(bfind(&values, &7), Some(3));
    }

    #[test]
    fn test_last_element() {
        let values = [1, 3, 5, 7, 9, 11, 13];
        assert_eq!(bfind(&values, &13), Some(6));
    }

    #[test]
    fn test_missing_target() {
        let values = [2, 4, 6, 8, 10, 12, 14];
        assert_eq!(bfind(&values, &9), None);
    }

    #[test]
    fn test_negative_numbers() {
        let values = [-10, -5, 0, 4, 9, 15];
        assert_eq!(bfind(&values, &15), Some(5));
        assert_eq!(bfind(&values, &-10), Some(0));
    }

    #[test]
    fn test_characters() {
        let values = ['a', 'c', 'e', 'g', 'i'];
        assert_eq!(bfind(&values, &'g'), Some(3));
    }
}
