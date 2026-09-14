pub fn max(elements: &Vec<i32>) -> Option<i32> {

    let mut max_element: Option<&i32> = elements.first();

    for num in elements.iter().skip(1) {
        if max_element < Some(num) {
            max_element = Some(num);
        }
    }
    return max_element.copied();
}

pub fn min(elements: &Vec<i32>) -> Option<i32> {

    let mut min_element: Option<&i32> = elements.first();

    for num in elements.iter().skip(1) {
        if min_element > Some(num) {
            min_element = Some(num);
        }
    }
    return min_element.copied();
}