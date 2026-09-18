pub fn shift_char(c: char, shift: i32) -> char {
    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
    let offset = ((c as i32 - base as i32 + shift).rem_euclid(26)) as u8;
    return (base + offset) as char;
}
