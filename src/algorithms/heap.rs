/// Ordena os elementos de `data` em ordem crescente usando Heap Sort.
///
/// # Complexidade
///
/// - Melhor caso: O(n log n)
/// - Caso médio: O(n log n)
/// - Pior caso: O(n log n)
/// - Espaço adicional: O(1)
pub fn heap_sort(data: &mut [i32]) {
    let len = data.len();

    if len <= 1 {
        return;
    }

    // Constrói o Max-Heap.
    for index in (0..len / 2).rev() {
        sift_down(data, index, len);
    }

    // Move o maior elemento para o final a cada iteração.
    for end in (1..len).rev() {
        data.swap(0, end);
        sift_down(data, 0, end);
    }
}

fn sift_down(data: &mut [i32], mut root: usize, heap_size: usize) {
    loop {
        let left = 2 * root + 1;
        let right = left + 1;

        if left >= heap_size {
            break;
        }

        let mut largest = left;

        if right < heap_size && data[right] > data[left] {
            largest = right;
        }

        if data[root] >= data[largest] {
            break;
        }

        data.swap(root, largest);
        root = largest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_data() {
        let mut data = [5, 2, 4, 6, 1, 3];

        heap_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut data: [i32; 0] = [];

        heap_sort(&mut data);

        assert_eq!(data, []);
    }

    #[test]
    fn handles_single_element() {
        let mut data = [42];

        heap_sort(&mut data);

        assert_eq!(data, [42]);
    }

    #[test]
    fn handles_already_sorted_data() {
        let mut data = [1, 2, 3, 4, 5];

        heap_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_reverse_sorted_data() {
        let mut data = [5, 4, 3, 2, 1];

        heap_sort(&mut data);

        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut data = [4, 2, 4, 1, 2, 1];

        heap_sort(&mut data);

        assert_eq!(data, [1, 1, 2, 2, 4, 4]);
    }
}
