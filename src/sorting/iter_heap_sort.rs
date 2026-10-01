pub fn iter_heap_sort<T: Ord + Copy>(elements: &mut [T]) {
    let mut n = elements.len();
    iter_max_heap(elements);
    for i in (1..n).rev() {
        elements.swap(0, i);
        n = n - 1;
        iter_sieve(&mut elements[0..n], 0);
    }
}

fn iter_max_heap<T: Ord + Copy>(elements: &mut [T]) {
    for depth in (0..elements.len()/2).rev() {
        iter_sieve(elements, depth);
    }
}

fn iter_sieve<T: Ord + Copy>(elements: &mut [T], mut depth: usize) {
    let n = elements.len();
    let mut left = 2 * depth + 1;
    let mut right = 2 * depth + 2;
    while left < n {
        let mut largest_idx = left;
        if right < n && elements[left] < elements[right] {
            largest_idx = right;
        }
        if elements[depth] >= elements[largest_idx] {
            break;
        }
        else {
            elements.swap(depth, largest_idx);
            depth = largest_idx;
            left = 2 * depth + 1;
            right = 2 * depth + 2;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::iter_heap_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        iter_heap_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values: [i32; 1] = [42];
        iter_heap_sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        iter_heap_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

        #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        iter_heap_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
        iter_heap_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = [-3, 0, -10, 7, -1, 2];
        iter_heap_sort(&mut values);
        assert_eq!(values, [-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        iter_heap_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7, 7, 7, 7, 7];
        iter_heap_sort(&mut values);
        assert_eq!(values, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = [i32::MAX, i32::MIN, 0, -1, 1];
        iter_heap_sort(&mut values);
        assert_eq!(values, [i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'e', 'a', 'c', 'b'];
        iter_heap_sort(&mut values);
        assert_eq!(values, ['a', 'b', 'c', 'd', 'e']);
    }
}