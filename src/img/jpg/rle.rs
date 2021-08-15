use super::huffman::{LUMINANCE_AC_TABLE, LUMINANCE_DC_TABLE, CHROMINANCE_AC_TABLE, CHROMINANCE_DC_TABLE};

use std::fmt;
use std::ops::{Add, AddAssign};
use super::jpeg::Mode;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bits {
    // Since some bits' length may less than 8
    // So the length field is introduced
    length: u8,
    bits: u32,
}

impl Bits {
    pub fn new(length: u8, bits: u32) -> Self {
        Self {
            length,
            bits: to_highest_pos(length, bits),
        }
    }

    pub fn dump(&mut self) -> Vec<u8> {
        let mut bytes = Vec::new();
        while self.length >= 8 {
            bytes.push((self.bits >> 24) as u8);

            self.length -= 8;
            self.bits = ((self.bits as u64) << 8) as u32;
        }

        bytes
    }

    // Must call at the end
    pub fn complete(&self) -> (u8, bool) {
        let is_complete = if self.length == 0 { true } else { false };
        let mut last_byte = (self.bits >> 24) as u8;
        last_byte |= get_lowest_n_bits(8 - self.length, 0b1111111);
        (last_byte, is_complete)
    }
}

impl Add for Bits {
    type Output = Self;

    // The later operand is setting on the least significant
    fn add(self, rhs: Self) -> Self {
        Self {
            length: self.length + rhs.length,
            bits: self.bits | (rhs.bits >> self.length),
        }
    }
}

impl AddAssign for Bits {
    fn add_assign(&mut self, rhs: Bits) {
        *self = Self {
            length: self.length + rhs.length,
            bits: self.bits | (rhs.bits >> self.length),
        }
    }
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bits {{ length: {:?}, bits: {:b} }}",
            self.length, self.bits
        )?;
        Ok(())
    }
}

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
    let amplitude = get_abs_bit_conut(dc) as u8;
    let codeword = if mode == Mode::Luminance {LUMINANCE_DC_TABLE.get(&amplitude)} else {CHROMINANCE_DC_TABLE.get(&amplitude)};
    // Row sysmbol2
    let ones_complements = get_ones_complements(dc);
    if let Some(codeword) = codeword {
        *codeword + Bits::new(amplitude, ones_complements as u32)
    } else {
        panic!("No such DC value!");
    }
}

fn encode_ac(run_length: u8, ac: i32, mode: Mode) -> Bits {
    // Huffman-coded sysmbo1
    let size = get_abs_bit_conut(ac) as u8;
    let symbol1 = run_length << 4 | size;
    let codeword = if mode == Mode::Luminance {LUMINANCE_AC_TABLE.get(&symbol1)} else {CHROMINANCE_AC_TABLE.get(&symbol1)};
    // Row sysmbo2
    let ones_complements = get_ones_complements(ac);
    if let Some(codeword) = codeword {
        *codeword + Bits::new(size, ones_complements as u32)
    } else {
        panic!("No such AC value!");
    }
}

fn to_highest_pos(length: u8, bits: u32) -> u32 {
    ((bits as u64) << (32 - length) as u64) as u32
}

fn get_lowest_n_bits(length: u8, bits: u8) -> u8 {
    bits & (2_u16.pow(length as u32) - 1) as u8
}

fn get_abs_bit_conut(num: i32) -> u32 {
    let num = num.abs();
    std::mem::size_of::<i32>() as u32 * 8 - num.leading_zeros()
}

fn get_ones_complements(num: i32) -> i32 {
    if num < 0 { num - 1 } else { num }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_dc() {
        assert_eq!(
            encode_dc(2, Mode::Luminance),
            Bits {
                length: 5,
                bits: 0b01110000000000000000000000000000
            }
        );
    }

    #[test]
    fn test_encode_ac() {
        assert_eq!(
            encode_ac(0, 16, Mode::Luminance),
            Bits {
                length: 10,
                bits: 0b11010100000000000000000000000000
            }
        );
    }

    #[test]
    fn test_encode_sequence() {
        let test_sequence = [2, 16, -21, 10, -15, 0, 0, 0, 3, -2, 0];
        encode(&test_sequence, &mut Bits::new(0, 0), 0, Mode::Luminance);
    }
}
