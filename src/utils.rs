pub fn max<T: Ord + Copy>(elements: &[T]) -> Option<T> {

    let mut max_element: Option<&T> = elements.first();

    for elem in elements.iter().skip(1) {
        if max_element < Some(elem) {
            max_element = Some(elem);
        }
    }
    return max_element.copied();
}

pub fn min<T: Ord + Copy>(elements: &[T]) -> Option<T> {

    let mut min_element: Option<&T> = elements.first();

    for num in elements.iter().skip(1) {
        if min_element > Some(num) {
            min_element = Some(num);
        }
    }
    return min_element.copied();
}

pub fn find<T: Eq + Copy>(elements: &[T], target: &T) -> Option<usize> {
    for (idx, num) in elements.iter().enumerate() {
        if target == num {
            return Some(idx);
        }
    }
    return None;
}

pub fn insertion_sort<T: Ord + Copy>(elements: &Vec<T>) -> Vec<T> {
    let mut elements_copy = elements.clone();
    
    let len = elements_copy.len();
    for i in 1..len {
        let mut j = i;
        let num = elements_copy[i];

        while j > 0 && num < elements_copy[j - 1] {
            elements_copy[j] = elements_copy[j - 1];
            j -= 1;
        }

        elements_copy[j] = num;
    }

    return elements_copy;
}

pub fn bubble_sort<T: Ord + Copy>(elements: &Vec<T>) -> Vec<T> {
    let mut elements_copy = elements.clone();
    let n = elements_copy.len();

    for i in 1..n {
        for j in 0..n - i {
            if elements_copy[j] > elements_copy[j + 1] {
                elements_copy.swap(j, j + 1);
            }
        }
    }
    return elements_copy;
}