#[test]
fn test_every_ascii_letter_and_shift() {
    for base in *b"Aa" {
        for letter in base..base + 26 {
            for shift in (-104..=104).chain([i32::MIN, i32::MAX]) {
                let offset = (i64::from(letter - base) + i64::from(shift)).rem_euclid(26);
                let expected = char::from(base + offset as u8);
                assert_eq!(super::shift_char(char::from(letter), shift), expected);
            }
        }
    }
}
