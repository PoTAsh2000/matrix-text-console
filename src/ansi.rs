/// Appends `value` as decimal digits.
pub fn push_decimal(out: &mut Vec<u8>, value: usize) {
    if value >= 10 {
        push_decimal(out, value / 10);
    }
    out.push(b'0' + (value % 10) as u8);
}

/// Moves the cursor to a zero-based position.
pub fn move_to(out: &mut Vec<u8>, column: usize, row: usize) {
    out.extend_from_slice(b"\x1b[");
    push_decimal(out, row + 1);
    out.push(b';');
    push_decimal(out, column + 1);
    out.push(b'H');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_digits() {
        for value in [0, 7, 10, 255, 1024] {
            let mut out = Vec::new();
            push_decimal(&mut out, value);
            assert_eq!(out, value.to_string().into_bytes());
        }
    }

    #[test]
    fn move_is_one_based() {
        let mut out = Vec::new();
        move_to(&mut out, 4, 2);
        assert_eq!(out, b"\x1b[3;5H");
    }
}
