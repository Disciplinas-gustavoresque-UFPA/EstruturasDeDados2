pub fn hanoi<T>(start: &mut Vec<T>, end: &mut Vec<T>) {
    let mut aux: Vec<T> = vec![];
    inner_hanoi(start.len(), start, end, &mut aux);
}

fn inner_hanoi<T>(n: usize, start: &mut Vec<T>, end: &mut Vec<T>, aux: &mut Vec<T>) {
    if n == 1 {
        let top = start.remove(0);
        end.insert(0, top);
    }
    else {
        inner_hanoi(n - 1, start, aux, end);
        let top = start.remove(0);
        end.insert(0, top);
        inner_hanoi(n - 1, aux, end, start);
    }
}

#[cfg(test)]
mod tests {

    use super::hanoi; 

    #[test]
    fn test_one_to_three_discs() {
        let mut start: Vec<i32> = (1..=1).step_by(1).collect();
        let mut end: Vec<i32> = vec![];
        hanoi(&mut start, &mut end);
        
    }
}

