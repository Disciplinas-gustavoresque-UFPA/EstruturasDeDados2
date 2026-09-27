/// Ordena os elementos de `data` em ordem crescente usando Merge Sort.
///
/// # Complexidade
///
/// - Melhor caso: O(n log n)
/// - Caso médio: O(n log n)
/// - Pior caso: O(n log n)
/// - Espaço adicional: O(n)
pub fn merge_sort(data: &mut [i32]) {
    let len = data.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge_sort(&mut data[..mid]);
    merge_sort(&mut data[mid..]);

    merge(data, mid);
}

fn merge(data: &mut [i32], mid: usize) {
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();

    let mut left_index = 0;
    let mut right_index = 0;
    let mut data_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            data[data_index] = left[left_index];
            left_index += 1;
        } else {
            data[data_index] = right[right_index];
            right_index += 1;
        }

        data_index += 1;
    }

    while left_index < left.len() {
        data[data_index] = left[left_index];
        left_index += 1;
        data_index += 1;
    }

    while right_index < right.len() {
        data[data_index] = right[right_index];
        right_index += 1;
        data_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_data() {
        let mut data = [5, 2, 4, 6, 1, 3];

        merge_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut data: [i32; 0] = [];

        merge_sort(&mut data);

        assert_eq!(data, []);
    }

    #[test]
    fn handles_single_element() {
        let mut data = [42];

        merge_sort(&mut data);

        assert_eq!(data, [42]);
    }

    #[test]
    fn handles_already_sorted_data() {
        let mut data = [1, 2, 3, 4, 5];

        merge_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_reverse_sorted_data() {
        let mut data = [5, 4, 3, 2, 1];

        merge_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut data = [4, 2, 4, 1, 2, 1];

        merge_sort(&mut data);

        assert_eq!(data, [1, 1, 2, 2, 4, 4]);
    }
}
