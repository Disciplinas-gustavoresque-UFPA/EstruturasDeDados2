pub fn find<T: Eq>(elements: &[T], target: &T) -> Option<usize> {
    for (idx, num) in elements.iter().enumerate() {
        if target == num {
            return Some(idx);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::find;

    #[test]
    fn test_empty() {
        let values: [i32; 0] = [];
        assert_eq!(find(&values, &10), None);
    }

    #[test]
    fn test_first_element() {
        let values = [3, 1, 4, 1, 5];
        assert_eq!(find(&values, &3), Some(0));
    }

    #[test]
    fn test_middle_element() {
        let values = [10, 20, 30, 40, 50];
        assert_eq!(find(&values, &30), Some(2));
    }

    #[test]
    fn test_last_element() {
        let values = [8, 6, 4, 2];
        assert_eq!(find(&values, &2), Some(3));
    }

    #[test]
    fn test_first_occurrence_when_duplicates_exist() {
        let values = [7, 2, 7, 9, 7];
        assert_eq!(find(&values, &7), Some(0));
    }

    #[test]
    fn test_target_missing() {
        let values = [1, 2, 3, 4];
        assert_eq!(find(&values, &99), None);
    }

    #[test]
    fn test_negative_numbers() {
        let values = [-5, -2, -9, -1];
        assert_eq!(find(&values, &-9), Some(2));
    }

    #[test]
    fn test_characters() {
        let values = ['a', 'b', 'c'];
        assert_eq!(find(&values, &'b'), Some(1));
        assert_eq!(find(&values, &'x'), None);
    }
}
