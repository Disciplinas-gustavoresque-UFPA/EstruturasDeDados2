pub fn bubble_sort<T: Ord>(elements: &mut [T]) {
    let n = elements.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n.saturating_sub(i + 1) {
            if elements[j] > elements[j + 1] {
                elements.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        bubble_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values = [42];
        bubble_sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5];
        bubble_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        bubble_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        bubble_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }
}