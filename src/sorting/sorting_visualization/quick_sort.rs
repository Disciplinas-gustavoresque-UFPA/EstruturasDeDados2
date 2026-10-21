
pub fn quick_sort<T: Ord>(elements: &mut [T]) {
    if elements.len() <= 1 {
        return;
    }

    let pivot_index = partition(elements, 0, elements.len() - 1);

    if pivot_index > 0 {
        quick_sort(&mut elements[..pivot_index]);
    }

    if pivot_index + 1 < elements.len() {
        quick_sort(&mut elements[pivot_index + 1..]);
    }
}

// Particionamento de Lomuto: utiliza o último elemento
// do intervalo como pivô.
fn partition<T: Ord>(
    elements: &mut [T],
    low: usize,
    high: usize,
) -> usize {
    let mut pivot_position = low;

    for current in low..high {
        if elements[current] <= elements[high] {
            elements.swap(pivot_position, current);
            pivot_position += 1;
        }
    }

    elements.swap(pivot_position, high);

    pivot_position
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
    fn test_two_sorted_elements() {
        let mut values = [1, 2];
        quick_sort(&mut values);
        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn test_two_reverse_sorted_elements() {
        let mut values = [2, 1];
        quick_sort(&mut values);
        assert_eq!(values, [1, 2]);
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

// Versão instrumentada para a visualização.
// O vetor completo é mantido em todas as etapas.
pub(super) fn quick_sort_with_trace(
    elements: &mut [i32],
    steps: &mut Vec<super::SortStep>,
    depth: usize,
) -> Result<(), String> {
    if elements.len() <= 1 {
        return Ok(());
    }

    quick_sort_range_with_trace(
        elements,
        0,
        elements.len() - 1,
        steps,
        depth,
    )
}

fn quick_sort_range_with_trace(
    elements: &mut [i32],
    low: usize,
    high: usize,
    steps: &mut Vec<super::SortStep>,
    depth: usize,
) -> Result<(), String> {
    if low >= high {
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

    let pivot_index =
        partition_with_trace(elements, low, high, steps);

    // Ordena a região à esquerda do pivô.
    if pivot_index > low {
        quick_sort_range_with_trace(
            elements,
            low,
            pivot_index - 1,
            steps,
            depth + 1,
        )?;
    }

    // Ordena a região à direita do pivô.
    if pivot_index < high {
        quick_sort_range_with_trace(
            elements,
            pivot_index + 1,
            high,
            steps,
            depth + 1,
        )?;
    }

    Ok(())
}

// Particionamento de Lomuto com registro das etapas.
// low e high são índices do vetor completo.
fn partition_with_trace(
    elements: &mut [i32],
    low: usize,
    high: usize,
    steps: &mut Vec<super::SortStep>,
) -> usize {
    let mut pivot_position = low;

    // Mostra o vetor completo e destaca o pivô.
    super::record(
        elements,
        &[high],
        &[],
        "Selecionando o último elemento do intervalo como pivô",
        steps,
    );

    for current in low..high {
        // Registra os índices absolutos da comparação.
        // elements continua sendo o vetor completo.
        super::record(
            elements,
            &[current, high],
            &[],
            "Comparando elemento com o pivô",
            steps,
        );

        if elements[current] <= elements[high] {
            elements.swap(pivot_position, current);

            super::record(
                elements,
                &[],
                &[pivot_position, current],
                "Troca realizada",
                steps,
            );

            pivot_position += 1;
        }
    }

    // Coloca o pivô na posição definitiva.
    elements.swap(pivot_position, high);

    super::record(
        elements,
        &[],
        &[pivot_position, high],
        "Pivô colocado em sua posição definitiva",
        steps,
    );

    pivot_position
}