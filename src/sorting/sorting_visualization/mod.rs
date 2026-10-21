
mod bubble_sort;
mod merge_sort;
mod quick_sort;

pub use bubble_sort::bubble_sort;
pub use merge_sort::merge_sort;
pub use quick_sort::quick_sort;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SortStep {
    pub values: Vec<i32>,
    pub compared: Vec<usize>,
    pub swapped: Vec<usize>,
    pub message: String,

    // Índice absoluto do pivô, quando aplicável.
    pub pivot: Option<usize>,

    // Intervalo inclusivo [início, fim] que está sendo processado.
    pub active_range: Option<[usize; 2]>,
}

#[derive(Serialize)]
pub struct SortResult {
    pub algorithm: String,
    pub initial: Vec<i32>,
    pub steps: Vec<SortStep>,
    pub sorted: Vec<i32>,
    pub completed: bool,
    pub error: Option<String>,
}

pub(super) fn record(
    values: &[i32],
    compared: &[usize],
    swapped: &[usize],
    message: &str,
    steps: &mut Vec<SortStep>,
) {
    steps.push(SortStep {
        values: values.to_vec(),
        compared: compared.to_vec(),
        swapped: swapped.to_vec(),
        message: message.to_string(),
        pivot: None,
        active_range: None,
    });
}

// Registro específico do Quick Sort.
// Recebe sempre o vetor completo, nunca apenas um subvetor.
pub(super) fn record_quick(
    values: &[i32],
    compared: &[usize],
    swapped: &[usize],
    pivot: Option<usize>,
    active_range: Option<[usize; 2]>,
    message: &str,
    steps: &mut Vec<SortStep>,
) {
    steps.push(SortStep {
        values: values.to_vec(),
        compared: compared.to_vec(),
        swapped: swapped.to_vec(),
        message: message.to_string(),
        pivot,
        active_range,
    });
}

pub fn sort_with_trace(algorithm: &str, mut values: Vec<i32>) -> SortResult {
    let initial = values.clone();
    let mut steps = Vec::new();

    record(
        &values,
        &[],
        &[],
        "Estado inicial",
        &mut steps,
    );

    let result = match algorithm {
        "bubble" => {
            bubble_sort::bubble_sort_with_trace(&mut values, &mut steps);
            Ok(())
        }
        "merge" => {
            merge_sort::merge_sort_with_trace(&mut values, &mut steps);
            Ok(())
        }
        "quick" => {
            quick_sort::quick_sort_with_trace(&mut values, &mut steps, 0)
        }
        _ => Err("Algoritmo desconhecido".to_string()),
    };

    let completed = result.is_ok();
    let error = result.err();

    record(
        &values,
        &[],
        &[],
        if completed {
            "Ordenação concluída"
        } else {
            "A demonstração foi interrompida"
        },
        &mut steps,
    );

    SortResult {
        algorithm: algorithm.to_string(),
        initial,
        steps,
        sorted: values,
        completed,
        error,
    }
}