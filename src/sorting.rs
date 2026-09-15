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