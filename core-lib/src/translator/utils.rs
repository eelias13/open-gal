pub fn bool_to_byte(byte: &Vec<bool>) -> u8 {
    let mut result = 0;
    for i in 0..8 {
        if byte[i] {
            result |= 1 << (7 - i);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn bool_to_byte() {
        assert_eq!(super::bool_to_byte(&vec![true; 8]), 0xff);
        assert_eq!(super::bool_to_byte(&vec![false; 8]), 0x00);
    }
}
