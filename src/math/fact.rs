pub fn fact(n: i64) -> i64 {
    if n < 0 {
        panic!("fact: parameter 'n' must be equal or greater than zero.");
    }
    let mut result: i64 = 1;
    for i in 1..n + 1 {
        result *= i;
    }
    return result;
}

pub fn fact_recursive(n: i64) -> i64 {
    if n < 0 {
        panic!("fact: parameter 'n' must be equal or greater than zero.");
    } else if n == 0 || n == 1 {
        return 1;
    }
    return n * fact_recursive(n - 1);
}

#[cfg(test)]
mod tests {
    use super::fact;
    use super::fact_recursive;

    #[test]
    #[should_panic]
    fn test_negative() {
        fact(-1);
    }

    #[test]
    #[should_panic]
    fn test_max_negative() {
        fact(-i64::MAX);
    }

    #[test]
    #[should_panic]
    fn test_negative_recursive() {
        fact_recursive(-1);
    }

    #[test]
    #[should_panic]
    fn test_max_negative_recursive() {
        fact_recursive(-i64::MAX);
    }

    #[test]
    fn test_first_ten_numbers() {
        let numbers: Vec<i64> = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];
        for (idx, number) in numbers.iter().enumerate() {
            assert_eq!(fact(idx as i64), *number);
            assert_eq!(fact_recursive(idx as i64), *number);
        }
    }

    #[test]
    fn test_twenty() {
        assert_eq!(fact(20), 2432902008176640000);
        assert_eq!(fact_recursive(20), 2432902008176640000);
    }
}
