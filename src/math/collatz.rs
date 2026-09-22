/*
   The conjecture asks whether repeating two simple arithmetic operations will eventually transform every positive integer into 1.
   It concerns sequences of integers in which each term is obtained from the previous term as follows:
   if a term is even, the next term is one half of it. If a term is odd, the next term is 3 times the previous term plus 1.
   The conjecture is that these sequences always reach 1, no matter which positive integer is chosen to start the sequence.
   Source: https://en.wikipedia.org/wiki/Collatz_conjecture
   e.g.:
   - 1 = [1],
   - 2 = [2, 1],
   - 3 = [3, 10, 5, 16, 8, 4, 2, 1],
   - 4 = [4, 2, 1],
   - 5 = [5, 16, 8, 4, 2, 1]
*/
pub fn collatz(n: i64, sequence: &mut Vec<i64>) {
    assert!(n >= 1, "fact: parameter 'n' must be greater than zero.");
    let mut number = n;
    while number > 1 {
        sequence.push(number);
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = 3 * number + 1;
        }
    }
    sequence.push(number);
}

/*
   This is the same conjecture but in a recursive form.
*/
pub fn collatz_recursive(n: i64, sequence: &mut Vec<i64>) {
    assert!(n >= 1, "fact: parameter 'n' must be greater than zero.");
    if n == 1 {
        sequence.push(1);
    } else {
        sequence.push(n);
        if n % 2 == 0 {
            collatz_recursive(n / 2, sequence);
        } else {
            collatz_recursive(3 * n + 1, sequence);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::collatz;
    use super::collatz_recursive;

    #[test]
    #[should_panic]
    fn test_zero() {
        collatz(0, &mut vec![]);
    }

    #[test]
    #[should_panic]
    fn test_negative() {
        collatz(-1, &mut vec![]);
    }

    #[test]
    #[should_panic]
    fn test_max_negative() {
        collatz(-i64::MAX, &mut vec![]);
    }

    #[test]
    #[should_panic]
    fn test_zero_recursive() {
        collatz_recursive(0, &mut vec![]);
    }

    #[test]
    #[should_panic]
    fn test_negative_recursive() {
        collatz_recursive(-1, &mut vec![]);
    }

    #[test]
    #[should_panic]
    fn test_max_negative_recursive() {
        collatz_recursive(-i64::MAX, &mut vec![]);
    }

    #[test]
    fn test_one_recursive() {
        let mut sequence: Vec<i64> = vec![];
        collatz_recursive(1, &mut sequence);
        assert_eq!(sequence, vec![1]);
    }

    #[test]
    fn test_three_to_six_recursive() {
        let mut sequence: Vec<i64> = vec![];

        collatz_recursive(3, &mut sequence);
        assert_eq!(sequence, vec![3, 10, 5, 16, 8, 4, 2, 1]);
        sequence.clear();

        collatz_recursive(4, &mut sequence);
        assert_eq!(sequence, vec![4, 2, 1]);
        sequence.clear();

        collatz_recursive(5, &mut sequence);
        assert_eq!(sequence, vec![5, 16, 8, 4, 2, 1]);
        sequence.clear();

        collatz_recursive(6, &mut sequence);
        assert_eq!(sequence, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
        sequence.clear();
    }

    #[test]
    fn test_one() {
        let mut sequence: Vec<i64> = vec![];
        collatz(1, &mut sequence);
        assert_eq!(sequence, vec![1]);
    }

    #[test]
    fn test_three_to_six() {
        let mut sequence: Vec<i64> = vec![];
        collatz(3, &mut sequence);
        assert_eq!(sequence, vec![3, 10, 5, 16, 8, 4, 2, 1]);
        sequence.clear();

        collatz(4, &mut sequence);
        assert_eq!(sequence, vec![4, 2, 1]);
        sequence.clear();

        collatz(5, &mut sequence);
        assert_eq!(sequence, vec![5, 16, 8, 4, 2, 1]);
        sequence.clear();

        collatz(6, &mut sequence);
        assert_eq!(sequence, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
        sequence.clear();
    }
}
