pub fn byte_to_char(u8_array: &Vec<u8>) -> Vec<u8> {
    u8_array.iter().map(|x| x + 48).collect::<Vec<u8>>()
}

// Convert u8 array to bit array
pub fn u8_to_bits(u8_array: &Vec<u8>) -> Vec<u8> {
    let mut bits_array = Vec::with_capacity((u8_array.len() + 7) / 8);

    let count = u8_array.len();
    let mut index = 0;
    while index + 8 <= count {
        let converted_u8 = to_bit(&u8_array[index..(index + 8)], 8);
        bits_array.push(converted_u8);
        index += 8;
    }
    if index != count {
        bits_array.push(to_bit(&u8_array[index..count], (count - index) as u8));
    }

    bits_array
}

fn to_bit(u8_slice: &[u8], calc_num: u8) -> u8 {
    let mut result = 0_u8;
    let mut index: u8 = 0;
    while index < calc_num {
        result |= u8_slice[index as usize] << (7 - index);
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::u8_to_bits;

    #[test]
    fn test_u8_to_bits_8bit() {
        let u8_array: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result = u8_to_bits(&u8_array);
        assert_eq!(1, result.len());
        assert_eq!(255, result[0]);
    }

    #[test]
    fn test_u8_bits_less_than_8_bit() {
        let u8_array: Vec<u8> = vec![0, 1];
        let result = u8_to_bits(&u8_array);
        assert_eq!(1, result.len());
        assert_eq!(64, result[0]);
    }

    #[test]
    fn test_u8_bits_more_than_8_bit() {
        let u8_array: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 1];
        let result = u8_to_bits(&u8_array);
        assert_eq!(2, result.len());
        assert_eq!(1, result[0]);
        assert_eq!(128, result[1]);
    }
}


