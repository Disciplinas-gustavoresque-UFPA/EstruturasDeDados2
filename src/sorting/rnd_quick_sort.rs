pub fn rnd_quick_sort<T: Ord + Copy>(elements: &mut [T]) {
    
    if elements.len() <=1 {   
        return;
    }

    let pivot = partition(elements);
    rnd_quick_sort(&mut elements[..pivot]);
    rnd_quick_sort(&mut elements[pivot + 1..]);
}

fn partition<T: Ord + Copy>(elements: &mut [T]) -> usize {
    
    let last_idx = elements.len() - 1;
    let mut i = 0;
    for j in 0..last_idx {
        if elements[j] <= elements[last_idx] {
            elements.swap(i, j);
            i = i + 1;
        }
    }
    elements.swap(i,last_idx);
    return i;
}

#[cfg(test)]
mod tests {
    use super::rnd_quick_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        rnd_quick_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values = [42];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [42]);
    }

     #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = [-3, 0, -10, 7, -1, 2];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7, 7, 7, 7, 7];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = [i32::MAX, i32::MIN, 0, -1, 1];
        rnd_quick_sort(&mut values);
        assert_eq!(values, [i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'a', 'c', 'b'];
        rnd_quick_sort(&mut values);
        assert_eq!(values, ['a', 'b', 'c', 'd']);
    }
}