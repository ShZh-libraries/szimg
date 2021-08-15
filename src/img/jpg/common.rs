use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bits {
    // Since some bits' length may less than 8
    // So the length field is introduced
    pub length: u8,
    pub bits: u32,
}

impl Bits {
    pub fn new(length: u8, bits: u32) -> Self {
        Self {
            length,
            bits: bit::to_highest_pos(length, bits),
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
        last_byte |= bit::get_lowest_n_bits(8 - self.length, 0b1111111);
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

// Utility bit operations
pub mod bit {
    pub fn to_highest_pos(length: u8, bits: u32) -> u32 {
        ((bits as u64) << (32 - length) as u64) as u32
    }

    pub fn get_lowest_n_bits(length: u8, bits: u8) -> u8 {
        bits & (2_u16.pow(length as u32) - 1) as u8
    }

    pub fn get_bit_conut(num: i32) -> u32 {
        std::mem::size_of::<i32>() as u32 * 8 - num.leading_zeros()
    }

    pub fn get_ones_complements(num: i32) -> i32 {
        if num < 0 {
            num - 1
        } else {
            num
        }
    }
}