use super::huffman::{
    CHROMINANCE_AC_TABLE, CHROMINANCE_DC_TABLE, LUMINANCE_AC_TABLE, LUMINANCE_DC_TABLE,
};

use super::common::{Bits, bit};
use super::jpeg::Mode;

pub fn encode(squence: &[i32], bits: &mut Bits, prev_dc: i32, mode: Mode) -> Vec<u8> {
    let mut result = Vec::new();
    let mut run_length = 0;
    for (index, num) in squence.iter().enumerate() {
        let mut encode = Bits::new(0, 0);
        if index == 0 {
            encode = encode_dc(*num - prev_dc, mode);
        } else {
            // Do not record when encounter 0
            // Only to increase run_length
            if *num == 0 {
                run_length += 1;
            } else {
                // Emit (run_size, 0) when encounter non-zero number
                // Note the run_size is up to 15
                // So if there is more than 15 zeros, emit multiple (15, 0) pairs
                while run_length > 15 {
                    let encode = encode_ac(15, 0, mode);
                    *bits += encode;
                    run_length -= 16;
                }
                // After encode zeros, we can now encode this non-zero number
                encode = encode_ac(run_length, *num, mode);
                run_length = 0;
            }
        }
        *bits += encode;
        let mut bytes = bits.dump();
        result.append(&mut bytes);
    }
    // End of Block: rl/size = 0/0
    if run_length != 0 {
        *bits += encode_ac(0, 0, mode);
        let mut last_byte = bits.dump();
        result.append(&mut last_byte);
    }

    result
}

fn encode_dc(dc: i32, mode: Mode) -> Bits {
    // Huffman-coded sysmbol1
    let amplitude = bit::get_bit_conut(dc.abs()) as u8;
    let codeword = if mode == Mode::Luminance {
        LUMINANCE_DC_TABLE.get(&amplitude)
    } else {
        CHROMINANCE_DC_TABLE.get(&amplitude)
    };
    // Row sysmbol2
    let ones_complements = bit::get_ones_complements(dc);
    if let Some(codeword) = codeword {
        *codeword + Bits::new(amplitude, ones_complements as u32)
    } else {
        panic!("No such DC value!");
    }
}

fn encode_ac(run_length: u8, ac: i32, mode: Mode) -> Bits {
    // Huffman-coded sysmbo1
    let size = bit::get_bit_conut(ac.abs()) as u8;
    let symbol1 = run_length << 4 | size;
    let codeword = if mode == Mode::Luminance {
        LUMINANCE_AC_TABLE.get(&symbol1)
    } else {
        CHROMINANCE_AC_TABLE.get(&symbol1)
    };
    // Row sysmbo2
    let ones_complements = bit::get_ones_complements(ac);
    if let Some(codeword) = codeword {
        *codeword + Bits::new(size, ones_complements as u32)
    } else {
        panic!("No such AC value!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_dc() {
        assert_eq!(
            encode_dc(2, Mode::Luminance),
            Bits::new(5, 0b01110)
        );
    }

    #[test]
    fn test_encode_ac() {
        assert_eq!(
            encode_ac(0, 16, Mode::Luminance),
            Bits::new(10, 0b1101010000)
        );
    }

    #[test]
    fn test_encode_sequence() {
        let test_sequence = [2, 16, -21, 10, -15, 0, 0, 0, 3, -2, 0];
        encode(&test_sequence, &mut Bits::new(0, 0), 0, Mode::Luminance);
    }
}
