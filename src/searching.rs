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