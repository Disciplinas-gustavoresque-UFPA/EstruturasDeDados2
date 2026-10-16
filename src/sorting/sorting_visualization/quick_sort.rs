pub fn quick_sort<T: Ord>(elements: &mut [T]) {
    if elements.len() <= 1 {
        return;
    }

    let pivot_index = partition(elements);

    let (left, right) = elements.split_at_mut(pivot_index + 1);

    quick_sort(left);
    quick_sort(right);
}

fn partition<T: Ord>(elements: &mut [T]) -> usize {
    let mut left = 0;
    let mut right = elements.len() - 1;
    let pivot_index = elements.len() / 2;

    loop {
        while elements[left] < elements[pivot_index] {
            left += 1;
        }

        while elements[right] > elements[pivot_index] {
            right -= 1;
        }

        if left >= right {
            return right;
        }

        elements.swap(left, right);

        left += 1;

        if right == 0 {
            return 0;
        }

        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn test_empty() {
        let mut values: [i32; 0] = [];
        quick_sort(&mut values);
        assert_eq!(values, []);
    }

    #[test]
    fn test_single_element() {
        let mut values = [42];
        quick_sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut values = [1, 2, 3, 4, 5];
        quick_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut values = [5, 4, 3, 2, 1];
        quick_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_mixed_values() {
        let mut values = [9, 1, 5, 3, 7, 2, 8, 4, 6];
        quick_sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut values = [-3, 0, -10, 7, -1, 2];
        quick_sort(&mut values);
        assert_eq!(values, [-10, -3, -1, 0, 2, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let mut values = [4, 2, 2, 1, 3, 4, 1];
        quick_sort(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn test_all_equal_values() {
        let mut values = [7, 7, 7, 7, 7];
        quick_sort(&mut values);
        assert_eq!(values, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_integer_boundaries() {
        let mut values = [i32::MAX, i32::MIN, 0, -1, 1];
        quick_sort(&mut values);
        assert_eq!(values, [i32::MIN, -1, 0, 1, i32::MAX]);
    }

    #[test]
    fn test_characters() {
        let mut values = ['d', 'a', 'c', 'b'];
        quick_sort(&mut values);
        assert_eq!(values, ['a', 'b', 'c', 'd']);
    }
}

pub(super) fn quick_sort_with_trace(
    elements: &mut [i32],
    steps: &mut Vec<super::SortStep>,
    depth: usize,
) -> Result<(), String> {
    if elements.len() <= 1 {
        return Ok(());
    }

    if depth >= 32 {
        super::record(
            elements,
            &[],
            &[],
            "Limite de recursão atingido; demonstração interrompida",
            steps,
        );

        return Err(
            "O Quick Sort atingiu o limite de recursão da demonstração."
                .to_string(),
        );
    }

    let pivot_index = partition_with_trace(elements, steps)?;

    let (left, right) = elements.split_at_mut(pivot_index + 1);

    quick_sort_with_trace(left, steps, depth + 1)?;
    quick_sort_with_trace(right, steps, depth + 1)?;

    Ok(())
}

fn partition_with_trace(
    elements: &mut [i32],
    steps: &mut Vec<super::SortStep>,
) -> Result<usize, String> {
    let mut left = 0;
    let mut right = elements.len() - 1;
    let pivot_index = elements.len() / 2;
    let max_iterations = elements.len().saturating_mul(4).saturating_add(4);
    let mut iterations = 0;

    loop {
        iterations += 1;

        if iterations > max_iterations {
            super::record(
                elements,
                &[],
                &[],
                "Particionamento excedeu o limite da demonstração",
                steps,
            );

            return Err("O particionamento não progrediu como esperado.".to_string());
        }

        while left < elements.len()
            && elements[left] < elements[pivot_index]
        {
            left += 1;
        }

        if left >= elements.len() {
            return Err(
                "O índice esquerdo ultrapassou os limites da lista."
                    .to_string(),
            );
        }

        while elements[right] > elements[pivot_index] {
            if right == 0 {
                return Err(
                    "O índice direito atingiu o limite da lista."
                        .to_string(),
                );
            }

            right -= 1;
        }

        super::record(
            elements,
            &[left, right, pivot_index],
            &[],
            "Comparando elementos com o pivô",
            steps,
        );

        if left >= right {
            return Ok(right);
        }

        elements.swap(left, right);

        super::record(
            elements,
            &[pivot_index],
            &[left, right],
            "Troca durante o particionamento",
            steps,
        );

        left += 1;

        if right == 0 {
            return Ok(0);
        }

        right -= 1;
    }
}