pub fn heap_sort<T: Ord + Copy>(elements: &mut [T]) {
    let mut n = elements.len();
    max_heap(elements);
    for i in (1..n).rev() {
        elements.swap(0, i);
        n = n - 1;
        sieve(&mut elements[0..n], 0);
    }
}

fn max_heap<T: Ord + Copy>(elements: &mut [T]) {
    for depth in (0..elements.len()/2).rev() {
        sieve(elements, depth);
    }
}

fn sieve<T: Ord + Copy>(elements: &mut [T], depth: usize) {
    let left = 2 * depth + 1;
    let right = 2 * depth + 2;
    let mut largest = depth;

    if left < elements.len() && elements[left] > elements[largest] {
        largest = left;
    }

    if right < elements.len() && elements[right] > elements[largest] {
        largest = right;
    }

    if largest != depth {
        elements.swap(depth, largest);
        sieve(elements, largest);
    }
}

#[cfg(test)]
mod tests {

    use super::heap_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        heap_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values: [i32; 1] = [42];
        heap_sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        heap_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

        #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        heap_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
        heap_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = [-3, 0, -10, 7, -1, 2];
        heap_sort(&mut values);
        assert_eq!(values, [-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        heap_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7, 7, 7, 7, 7];
        heap_sort(&mut values);
        assert_eq!(values, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = [5, -5, 0, -1, 1];
        heap_sort(&mut values);
        assert_eq!(values, [-5, -1, 0, 1, 5]);
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'a', 'c', 'b'];
        heap_sort(&mut values);
        assert_eq!(values, ['a', 'b', 'c', 'd']);
    }
}