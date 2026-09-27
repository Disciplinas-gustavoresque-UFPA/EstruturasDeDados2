/// Ordena os elementos de `data` em ordem crescente usando Quick Sort.
///
/// # Complexidade
///
/// - Melhor caso: O(n log n)
/// - Caso médio: O(n log n)
/// - Pior caso: O(n²)
/// - Espaço adicional: O(log n) em média devido à recursão
pub fn quick_sort(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition(data);

    let (left, right) = data.split_at_mut(pivot_index);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(data: &mut [i32]) -> usize {
    let pivot_index = data.len() - 1;
    let pivot = data[pivot_index];

    let mut smaller_index = 0;

    for current_index in 0..pivot_index {
        if data[current_index] <= pivot {
            data.swap(smaller_index, current_index);
            smaller_index += 1;
        }
    }

    data.swap(smaller_index, pivot_index);

    smaller_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_data() {
        let mut data = [5, 2, 4, 6, 1, 3];

        quick_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut data: [i32; 0] = [];

        quick_sort(&mut data);

        assert_eq!(data, []);
    }

    #[test]
    fn handles_single_element() {
        let mut data = [42];

        quick_sort(&mut data);

        assert_eq!(data, [42]);
    }

    #[test]
    fn handles_already_sorted_data() {
        let mut data = [1, 2, 3, 4, 5];

        quick_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_reverse_sorted_data() {
        let mut data = [5, 4, 3, 2, 1];

        quick_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut data = [4, 2, 4, 1, 2, 1];

        quick_sort(&mut data);

        assert_eq!(data, [1, 1, 2, 2, 4, 4]);
    }
}
