/// Ordena os elementos de `data` em ordem crescente usando Insertion Sort.
///
/// # Complexidade
///
/// - Melhor caso: O(n)
/// - Caso médio: O(n²)
/// - Pior caso: O(n²)
/// - Espaço adicional: O(1)
pub fn insertion_sort(data: &mut [i32]) {
    for i in 1..data.len() {
        let mut j = i;

        while j > 0 && data[j] < data[j - 1] {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_data() {
        let mut data = [5, 2, 4, 6, 1, 3];

        insertion_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut data: [i32; 0] = [];

        insertion_sort(&mut data);

        assert_eq!(data, []);
    }

    #[test]
    fn handles_single_element() {
        let mut data = [42];

        insertion_sort(&mut data);

        assert_eq!(data, [42]);
    }

    #[test]
    fn handles_already_sorted_data() {
        let mut data = [1, 2, 3, 4, 5];

        insertion_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_reverse_sorted_data() {
        let mut data = [5, 4, 3, 2, 1];

        insertion_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut data = [4, 2, 4, 1, 2, 1];

        insertion_sort(&mut data);

        assert_eq!(data, [1, 1, 2, 2, 4, 4]);
    }
}
