// https://mathworld.wolfram.com/HofstadtersQ-Sequence.html
pub fn hofstadter_q(n: i64) -> i64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let q1 = n - hofstadter_q(n - 1);
    let q2 = n - hofstadter_q(n - 2);
    return hofstadter_q(q1) + hofstadter_q(q2);
}

#[cfg(test)]
mod tests {
    use super::hofstadter_q;

    #[test]
    fn test_first_numbers() {
        let numbers: Vec<i64> = vec![
            1, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 8, 8, 8, 10, 9, 10, 11, 11, 12, 12, 12, 12, 16
        ];
        for (idx, number) in (1i64..).zip(numbers.iter()) {
            assert_eq!(hofstadter_q(idx), *number);
        }
    }
}
