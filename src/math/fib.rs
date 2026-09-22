pub fn fib(n: i64) -> i64 {
    assert!(
        n >= 0,
        "fact: parameter 'n' must be equal or greater than zero."
    );
    if n == 0 || n == 1 {
        return n;
    }

    let mut f1 = 0;
    let mut f2 = 1;
    for _ in 1..n {
        (f1, f2) = (f2, f1 + f2);
    }
    return f2;
}

pub fn fib_recursive(n: i64) -> i64 {
    assert!(
        n >= 0,
        "fact: parameter 'n' must be equal or greater than zero."
    );
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

#[cfg(test)]
mod tests {

    use super::fib;
    use super::fib_recursive;

    #[test]
    #[should_panic]
    fn test_negative() {
        fib(-1);
    }

    #[test]
    #[should_panic]
    fn test_max_negative() {
        fib(-i64::MAX);
    }

    #[test]
    #[should_panic]
    fn test_negative_recursive() {
        fib_recursive(-1);
    }

    #[test]
    #[should_panic]
    fn test_max_negative_recursive() {
        fib_recursive(-i64::MAX);
    }

    #[test]
    fn test_first_twenty_numbers() {
        let numbers: Vec<i64> = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765,
        ];
        for (idx, number) in numbers.iter().enumerate() {
            assert_eq!(fib(idx as i64), *number);
            assert_eq!(fib_recursive(idx as i64), *number);
        }
    }

    #[test]
    fn test_ninety() {
        // source: https://r-knott.surrey.ac.uk/fibonacci/fibtable.html
        assert_eq!(fib(90), 2880067194370816120);
        assert_eq!(fib_recursive(90), 2880067194370816120);
    }
}
