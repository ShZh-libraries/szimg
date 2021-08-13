use lazy_static::lazy_static;

use std::fmt;
use std::ops::{Add, AddAssign};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bits {
    // Since some bits' length may less than 8
    // So the length field is introduced
    length: u8,
    bits: u32,
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bits {{ length: {:?}, bits: {:b} }}", self.length, self.bits)?;
        Ok(())
    }
}

impl AddAssign for Bits {
    fn add_assign(&mut self, rhs: Bits) {
        *self = Self {
            length: self.length + rhs.length,
            bits: self.bits | (rhs.bits >> self.length)
        }
    }
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
            // println!("{:08b}", self.bits >> 24);
            bytes.push((self.bits >> 24) as u8);

            self.length -= 8;
            self.bits = ((self.bits as u64) << 8) as u32;
        }

        bytes
    }

    // Must call at the end
    pub fn complete(&self) -> (u8, bool) {
        let is_complete = if self.length == 0 { true } else { false };
        println!("{}", self.length);
        let mut last_byte = (self.bits >> 24) as u8;
        last_byte |= 0b1111111 & (2_u8.pow(8 - self.length as u32) - 1);
        (last_byte, is_complete)
    }
}

fn to_highest_pos(length: u8, bits: u32) -> u32 {
    ((bits as u64) << (32 - length) as u64) as u32
}

impl Add for Bits {
    type Output = Self;

    // The later operand is setting on the least significant
    fn add(self, rhs: Self) -> Self {
        Self {
            length: self.length + rhs.length,
            bits: self.bits | (rhs.bits >> self.length)
        }
    }
}

pub struct HuffmanSpec {
    pub count: [u8; 16],
    pub value: Vec<u8>
}

// JPEG general purpose hash table
lazy_static! {
    pub static ref LUMINANCE_DC_SPEC: HuffmanSpec = HuffmanSpec {
        count: [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        value: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    };

    pub static ref LUMINANCE_AC_SPEC: HuffmanSpec = HuffmanSpec {
        count: [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 125],
        value: vec![
			0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12,
			0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07,
			0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08,
			0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0,
			0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16,
			0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28,
			0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
			0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
			0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
			0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
			0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79,
			0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
			0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98,
			0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
			0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6,
			0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5,
			0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4,
			0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2,
			0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea,
			0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
			0xf9, 0xfa,
        ],
    };

    pub static ref CHROMINANCE_DC_SPEC: HuffmanSpec = HuffmanSpec {
        count: [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        value: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    };

    pub static ref CHROMINANCE_AC_SPEC: HuffmanSpec = HuffmanSpec {
        count: [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 119],
        value: vec![
            0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21,
			0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71,
			0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91,
			0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0,
			0x15, 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34,
			0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26,
			0x27, 0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38,
			0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
			0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
			0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
			0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
			0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
			0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96,
			0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5,
			0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4,
			0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3,
			0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2,
			0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda,
			0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9,
			0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
			0xf9, 0xfa,
        ],
    };

    pub static ref LUMINANCE_DC_TABLE: HashMap<u8, Bits> = {
        let mut table = HashMap::new();
        table.insert(0, Bits::new(2, 0b00));
        table.insert(1, Bits::new(3, 0b010));
        table.insert(2, Bits::new(3, 0b011));
        table.insert(3, Bits::new(3, 0b100));
        table.insert(4, Bits::new(3, 0b101));
        table.insert(5, Bits::new(3, 0b110));
        table.insert(6, Bits::new(4, 0b1110));
        table.insert(7, Bits::new(5, 0b11110));
        table.insert(8, Bits::new(6, 0b111110));
        table.insert(9, Bits::new(7, 0b1111110));
        table.insert(10, Bits::new(8, 0b11111110));
        table.insert(11, Bits::new(9, 0b111111110));

        table
    };

    pub static ref CHROMINANCE_DC_TABLE: HashMap<u8, Bits> = {
        let mut table = HashMap::new();
        table.insert(0, Bits::new(2, 0b00));
        table.insert(1, Bits::new(2, 0b01));
        table.insert(2, Bits::new(2, 0b10));
        table.insert(3, Bits::new(3, 0b110));
        table.insert(4, Bits::new(4, 0b1110));
        table.insert(5, Bits::new(5, 0b11110));
        table.insert(6, Bits::new(6, 0b111110));
        table.insert(7, Bits::new(7, 0b1111110));
        table.insert(8, Bits::new(8, 0b11111110));
        table.insert(9, Bits::new(9, 0b111111110));
        table.insert(10, Bits::new(10, 0b1111111110));
        table.insert(11, Bits::new(11, 0b11111111110));

        table
    };

    pub static ref LUMINANCE_AC_TABLE: HashMap<u8, Bits> = {
        let mut table = HashMap::new();

        table.insert(0, Bits::new(4, 0b1010));
        table.insert(1, Bits::new(2, 0b00));
        table.insert(2, Bits::new(2, 0b01));
        table.insert(3, Bits::new(3, 0b100));
        table.insert(4, Bits::new(4, 0b1011));
        table.insert(5, Bits::new(5, 0b11010));
        table.insert(6, Bits::new(7, 0b1111000));
        table.insert(7, Bits::new(8, 0b11111000));
        table.insert(8, Bits::new(10, 0b1111110110));
        table.insert(9, Bits::new(16, 0b1111111110000010));
        table.insert(10, Bits::new(16, 0b1111111110000011));
        table.insert(17, Bits::new(4, 0b1100));
        table.insert(18, Bits::new(5, 0b11011));
        table.insert(19, Bits::new(7, 0b1111001));
        table.insert(20, Bits::new(9, 0b111110110));
        table.insert(21, Bits::new(11, 0b11111110110));
        table.insert(22, Bits::new(16, 0b1111111110000100));
        table.insert(23, Bits::new(16, 0b1111111110000101));
        table.insert(24, Bits::new(16, 0b1111111110000110));
        table.insert(25, Bits::new(16, 0b1111111110000111));
        table.insert(26, Bits::new(16, 0b1111111110001000));
        table.insert(33, Bits::new(5, 0b11100));
        table.insert(34, Bits::new(8, 0b11111001));
        table.insert(35, Bits::new(10, 0b1111110111));
        table.insert(36, Bits::new(12, 0b111111110100));
        table.insert(37, Bits::new(16, 0b1111111110001001));
        table.insert(38, Bits::new(16, 0b1111111110001010));
        table.insert(39, Bits::new(16, 0b1111111110001011));
        table.insert(40, Bits::new(16, 0b1111111110001100));
        table.insert(41, Bits::new(16, 0b1111111110001101));
        table.insert(42, Bits::new(16, 0b1111111110001110));
        table.insert(49, Bits::new(6, 0b111010));
        table.insert(50, Bits::new(9, 0b111110111));
        table.insert(51, Bits::new(12, 0b111111110101));
        table.insert(52, Bits::new(16, 0b1111111110001111));
        table.insert(53, Bits::new(16, 0b1111111110010000));
        table.insert(54, Bits::new(16, 0b1111111110010001));
        table.insert(55, Bits::new(16, 0b1111111110010010));
        table.insert(56, Bits::new(16, 0b1111111110010011));
        table.insert(57, Bits::new(16, 0b1111111110010100));
        table.insert(58, Bits::new(16, 0b1111111110010101));
        table.insert(65, Bits::new(6, 0b111011));
        table.insert(66, Bits::new(10, 0b1111111000));
        table.insert(67, Bits::new(16, 0b1111111110010110));
        table.insert(68, Bits::new(16, 0b1111111110010111));
        table.insert(69, Bits::new(16, 0b1111111110011000));
        table.insert(70, Bits::new(16, 0b1111111110011001));
        table.insert(71, Bits::new(16, 0b1111111110011010));
        table.insert(72, Bits::new(16, 0b1111111110011011));
        table.insert(73, Bits::new(16, 0b1111111110011100));
        table.insert(74, Bits::new(16, 0b1111111110011101));
        table.insert(81, Bits::new(7, 0b1111010));
        table.insert(82, Bits::new(11, 0b11111110111));
        table.insert(83, Bits::new(16, 0b1111111110011110));
        table.insert(84, Bits::new(16, 0b1111111110011111));
        table.insert(85, Bits::new(16, 0b1111111110100000));
        table.insert(86, Bits::new(16, 0b1111111110100001));
        table.insert(87, Bits::new(16, 0b1111111110100010));
        table.insert(88, Bits::new(16, 0b1111111110100011));
        table.insert(89, Bits::new(16, 0b1111111110100100));
        table.insert(90, Bits::new(16, 0b1111111110100101));
        table.insert(97, Bits::new(7, 0b1111011));
        table.insert(98, Bits::new(12, 0b111111110110));
        table.insert(99, Bits::new(16, 0b1111111110100110));
        table.insert(100, Bits::new(16, 0b1111111110100111));
        table.insert(101, Bits::new(16, 0b1111111110101000));
        table.insert(102, Bits::new(16, 0b1111111110101001));
        table.insert(103, Bits::new(16, 0b1111111110101010));
        table.insert(104, Bits::new(16, 0b1111111110101011));
        table.insert(105, Bits::new(16, 0b1111111110101100));
        table.insert(106, Bits::new(16, 0b1111111110101101));
        table.insert(113, Bits::new(8, 0b11111010));
        table.insert(114, Bits::new(12, 0b111111110111));
        table.insert(115, Bits::new(16, 0b1111111110101110));
        table.insert(116, Bits::new(16, 0b1111111110101111));
        table.insert(117, Bits::new(16, 0b1111111110110000));
        table.insert(118, Bits::new(16, 0b1111111110110001));
        table.insert(119, Bits::new(16, 0b1111111110110010));
        table.insert(120, Bits::new(16, 0b1111111110110011));
        table.insert(121, Bits::new(16, 0b1111111110110100));
        table.insert(122, Bits::new(16, 0b1111111110110101));
        table.insert(129, Bits::new(9, 0b111111000));
        table.insert(130, Bits::new(15, 0b111111111000000));
        table.insert(131, Bits::new(16, 0b1111111110110110));
        table.insert(132, Bits::new(16, 0b1111111110110111));
        table.insert(133, Bits::new(16, 0b1111111110111000));
        table.insert(134, Bits::new(16, 0b1111111110111001));
        table.insert(135, Bits::new(16, 0b1111111110111010));
        table.insert(136, Bits::new(16, 0b1111111110111011));
        table.insert(137, Bits::new(16, 0b1111111110111100));
        table.insert(138, Bits::new(16, 0b1111111110111101));
        table.insert(145, Bits::new(9, 0b111111001));
        table.insert(146, Bits::new(16, 0b1111111110111110));
        table.insert(147, Bits::new(16, 0b1111111110111111));
        table.insert(148, Bits::new(16, 0b1111111111000000));
        table.insert(149, Bits::new(16, 0b1111111111000001));
        table.insert(150, Bits::new(16, 0b1111111111000010));
        table.insert(151, Bits::new(16, 0b1111111111000011));
        table.insert(152, Bits::new(16, 0b1111111111000100));
        table.insert(153, Bits::new(16, 0b1111111111000101));
        table.insert(154, Bits::new(16, 0b1111111111000110));
        table.insert(161, Bits::new(9, 0b111111010));
        table.insert(162, Bits::new(16, 0b1111111111000111));
        table.insert(163, Bits::new(16, 0b1111111111001000));
        table.insert(164, Bits::new(16, 0b1111111111001001));
        table.insert(165, Bits::new(16, 0b1111111111001010));
        table.insert(166, Bits::new(16, 0b1111111111001011));
        table.insert(167, Bits::new(16, 0b1111111111001100));
        table.insert(168, Bits::new(16, 0b1111111111001101));
        table.insert(169, Bits::new(16, 0b1111111111001110));
        table.insert(170, Bits::new(16, 0b1111111111001111));
        table.insert(177, Bits::new(10, 0b1111111001));
        table.insert(178, Bits::new(16, 0b1111111111010000));
        table.insert(179, Bits::new(16, 0b1111111111010001));
        table.insert(180, Bits::new(16, 0b1111111111010010));
        table.insert(181, Bits::new(16, 0b1111111111010011));
        table.insert(182, Bits::new(16, 0b1111111111010100));
        table.insert(183, Bits::new(16, 0b1111111111010101));
        table.insert(184, Bits::new(16, 0b1111111111010110));
        table.insert(185, Bits::new(16, 0b1111111111010111));
        table.insert(186, Bits::new(16, 0b1111111111011000));
        table.insert(193, Bits::new(10, 0b1111111010));
        table.insert(194, Bits::new(16, 0b1111111111011001));
        table.insert(195, Bits::new(16, 0b1111111111011010));
        table.insert(196, Bits::new(16, 0b1111111111011011));
        table.insert(197, Bits::new(16, 0b1111111111011100));
        table.insert(198, Bits::new(16, 0b1111111111011101));
        table.insert(199, Bits::new(16, 0b1111111111011110));
        table.insert(200, Bits::new(16, 0b1111111111011111));
        table.insert(201, Bits::new(16, 0b1111111111100000));
        table.insert(202, Bits::new(16, 0b1111111111100001));
        table.insert(209, Bits::new(11, 0b11111111000));
        table.insert(210, Bits::new(16, 0b1111111111100010));
        table.insert(211, Bits::new(16, 0b1111111111100011));
        table.insert(212, Bits::new(16, 0b1111111111100100));
        table.insert(213, Bits::new(16, 0b1111111111100101));
        table.insert(214, Bits::new(16, 0b1111111111100110));
        table.insert(215, Bits::new(16, 0b1111111111100111));
        table.insert(216, Bits::new(16, 0b1111111111101000));
        table.insert(217, Bits::new(16, 0b1111111111101001));
        table.insert(218, Bits::new(16, 0b1111111111101010));
        table.insert(225, Bits::new(16, 0b1111111111101011));
        table.insert(226, Bits::new(16, 0b1111111111101100));
        table.insert(227, Bits::new(16, 0b1111111111101101));
        table.insert(228, Bits::new(16, 0b1111111111101110));
        table.insert(229, Bits::new(16, 0b1111111111101111));
        table.insert(230, Bits::new(16, 0b1111111111110000));
        table.insert(231, Bits::new(16, 0b1111111111110001));
        table.insert(232, Bits::new(16, 0b1111111111110010));
        table.insert(233, Bits::new(16, 0b1111111111110011));
        table.insert(234, Bits::new(16, 0b1111111111110100));
        table.insert(240, Bits::new(11, 0b11111111001));
        table.insert(241, Bits::new(16, 0b1111111111110101));
        table.insert(242, Bits::new(16, 0b1111111111110110));
        table.insert(243, Bits::new(16, 0b1111111111110111));
        table.insert(244, Bits::new(16, 0b1111111111111000));
        table.insert(245, Bits::new(16, 0b1111111111111001));
        table.insert(246, Bits::new(16, 0b1111111111111010));
        table.insert(247, Bits::new(16, 0b1111111111111011));
        table.insert(248, Bits::new(16, 0b1111111111111100));
        table.insert(249, Bits::new(16, 0b1111111111111101));
        table.insert(250, Bits::new(16, 0b1111111111111110));

        table
    };

    pub static ref CHROMINANCE_AC_TABLE: HashMap<u8, Bits> = {
        let mut table = HashMap::new();

        table.insert(0, Bits::new(2, 0b00));
        table.insert(1, Bits::new(2, 0b01));
        table.insert(2, Bits::new(3, 0b100));
        table.insert(3, Bits::new(4, 0b1010));
        table.insert(4, Bits::new(5, 0b11000));
        table.insert(5, Bits::new(5, 0b11001));
        table.insert(6, Bits::new(6, 0b111000));
        table.insert(7, Bits::new(7, 0b1111000));
        table.insert(8, Bits::new(9, 0b111110100));
        table.insert(9, Bits::new(10, 0b1111110110));
        table.insert(10, Bits::new(12, 0b111111110100));
        table.insert(17, Bits::new(4, 0b1011));
        table.insert(18, Bits::new(6, 0b111001));
        table.insert(19, Bits::new(8, 0b11110110));
        table.insert(20, Bits::new(9, 0b111110101));
        table.insert(21, Bits::new(11, 0b11111110110));
        table.insert(22, Bits::new(12, 0b111111110101));
        table.insert(23, Bits::new(16, 0b1111111110001000));
        table.insert(24, Bits::new(16, 0b1111111110001001));
        table.insert(25, Bits::new(16, 0b1111111110001010));
        table.insert(26, Bits::new(16, 0b1111111110001011));
        table.insert(33, Bits::new(5, 0b11010));
        table.insert(34, Bits::new(8, 0b11110111));
        table.insert(35, Bits::new(10, 0b1111110111));
        table.insert(36, Bits::new(12, 0b111111110110));
        table.insert(37, Bits::new(15, 0b111111111000010));
        table.insert(38, Bits::new(16, 0b1111111110001100));
        table.insert(39, Bits::new(16, 0b1111111110001101));
        table.insert(40, Bits::new(16, 0b1111111110001110));
        table.insert(41, Bits::new(16, 0b1111111110001111));
        table.insert(42, Bits::new(16, 0b1111111110010000));
        table.insert(49, Bits::new(5, 0b11011));
        table.insert(50, Bits::new(8, 0b11111000));
        table.insert(51, Bits::new(10, 0b1111111000));
        table.insert(52, Bits::new(12, 0b111111110111));
        table.insert(53, Bits::new(16, 0b1111111110010001));
        table.insert(54, Bits::new(16, 0b1111111110010010));
        table.insert(55, Bits::new(16, 0b1111111110010011));
        table.insert(56, Bits::new(16, 0b1111111110010100));
        table.insert(57, Bits::new(16, 0b1111111110010101));
        table.insert(58, Bits::new(16, 0b1111111110010110));
        table.insert(65, Bits::new(6, 0b111010));
        table.insert(66, Bits::new(9, 0b111110110));
        table.insert(67, Bits::new(16, 0b1111111110010111));
        table.insert(68, Bits::new(16, 0b1111111110011000));
        table.insert(69, Bits::new(16, 0b1111111110011001));
        table.insert(70, Bits::new(16, 0b1111111110011010));
        table.insert(71, Bits::new(16, 0b1111111110011011));
        table.insert(72, Bits::new(16, 0b1111111110011100));
        table.insert(73, Bits::new(16, 0b1111111110011101));
        table.insert(74, Bits::new(16, 0b1111111110011110));
        table.insert(81, Bits::new(6, 0b111011));
        table.insert(82, Bits::new(10, 0b1111111001));
        table.insert(83, Bits::new(16, 0b1111111110011111));
        table.insert(84, Bits::new(16, 0b1111111110100000));
        table.insert(85, Bits::new(16, 0b1111111110100001));
        table.insert(86, Bits::new(16, 0b1111111110100010));
        table.insert(87, Bits::new(16, 0b1111111110100011));
        table.insert(88, Bits::new(16, 0b1111111110100100));
        table.insert(89, Bits::new(16, 0b1111111110100101));
        table.insert(90, Bits::new(16, 0b1111111110100110));
        table.insert(97, Bits::new(7, 0b1111001));
        table.insert(98, Bits::new(11, 0b11111110111));
        table.insert(99, Bits::new(16, 0b1111111110100111));
        table.insert(100, Bits::new(16, 0b1111111110101000));
        table.insert(101, Bits::new(16, 0b1111111110101001));
        table.insert(102, Bits::new(16, 0b1111111110101010));
        table.insert(103, Bits::new(16, 0b1111111110101011));
        table.insert(104, Bits::new(16, 0b1111111110101100));
        table.insert(105, Bits::new(16, 0b1111111110101101));
        table.insert(106, Bits::new(16, 0b1111111110101110));
        table.insert(113, Bits::new(7, 0b1111010));
        table.insert(114, Bits::new(11, 0b11111111000));
        table.insert(115, Bits::new(16, 0b1111111110101111));
        table.insert(116, Bits::new(16, 0b1111111110110000));
        table.insert(117, Bits::new(16, 0b1111111110110001));
        table.insert(118, Bits::new(16, 0b1111111110110010));
        table.insert(119, Bits::new(16, 0b1111111110110011));
        table.insert(120, Bits::new(16, 0b1111111110110100));
        table.insert(121, Bits::new(16, 0b1111111110110101));
        table.insert(122, Bits::new(16, 0b1111111110110110));
        table.insert(129, Bits::new(8, 0b11111001));
        table.insert(130, Bits::new(16, 0b1111111110110111));
        table.insert(131, Bits::new(16, 0b1111111110111000));
        table.insert(132, Bits::new(16, 0b1111111110111001));
        table.insert(133, Bits::new(16, 0b1111111110111010));
        table.insert(134, Bits::new(16, 0b1111111110111011));
        table.insert(135, Bits::new(16, 0b1111111110111100));
        table.insert(136, Bits::new(16, 0b1111111110111101));
        table.insert(137, Bits::new(16, 0b1111111110111110));
        table.insert(138, Bits::new(16, 0b1111111110111111));
        table.insert(145, Bits::new(9, 0b111110111));
        table.insert(146, Bits::new(16, 0b1111111111000000));
        table.insert(147, Bits::new(16, 0b1111111111000001));
        table.insert(148, Bits::new(16, 0b1111111111000010));
        table.insert(149, Bits::new(16, 0b1111111111000011));
        table.insert(150, Bits::new(16, 0b1111111111000100));
        table.insert(151, Bits::new(16, 0b1111111111000101));
        table.insert(152, Bits::new(16, 0b1111111111000110));
        table.insert(153, Bits::new(16, 0b1111111111000111));
        table.insert(154, Bits::new(16, 0b1111111111001000));
        table.insert(161, Bits::new(9, 0b111111000));
        table.insert(162, Bits::new(16, 0b1111111111001001));
        table.insert(163, Bits::new(16, 0b1111111111001010));
        table.insert(164, Bits::new(16, 0b1111111111001011));
        table.insert(165, Bits::new(16, 0b1111111111001100));
        table.insert(166, Bits::new(16, 0b1111111111001101));
        table.insert(167, Bits::new(16, 0b1111111111001110));
        table.insert(168, Bits::new(16, 0b1111111111001111));
        table.insert(169, Bits::new(16, 0b1111111111010000));
        table.insert(170, Bits::new(16, 0b1111111111010001));
        table.insert(177, Bits::new(9, 0b111111001));
        table.insert(178, Bits::new(16, 0b1111111111010010));
        table.insert(179, Bits::new(16, 0b1111111111010011));
        table.insert(180, Bits::new(16, 0b1111111111010100));
        table.insert(181, Bits::new(16, 0b1111111111010101));
        table.insert(182, Bits::new(16, 0b1111111111010110));
        table.insert(183, Bits::new(16, 0b1111111111010111));
        table.insert(184, Bits::new(16, 0b1111111111011000));
        table.insert(185, Bits::new(16, 0b1111111111011001));
        table.insert(186, Bits::new(16, 0b1111111111011010));
        table.insert(193, Bits::new(9, 0b111111010));
        table.insert(194, Bits::new(16, 0b1111111111011011));
        table.insert(195, Bits::new(16, 0b1111111111011100));
        table.insert(196, Bits::new(16, 0b1111111111011101));
        table.insert(197, Bits::new(16, 0b1111111111011110));
        table.insert(198, Bits::new(16, 0b1111111111011111));
        table.insert(199, Bits::new(16, 0b1111111111100000));
        table.insert(200, Bits::new(16, 0b1111111111100001));
        table.insert(201, Bits::new(16, 0b1111111111100010));
        table.insert(202, Bits::new(16, 0b1111111111100011));
        table.insert(209, Bits::new(11, 0b11111111001));
        table.insert(210, Bits::new(16, 0b1111111111100100));
        table.insert(211, Bits::new(16, 0b1111111111100101));
        table.insert(212, Bits::new(16, 0b1111111111100110));
        table.insert(213, Bits::new(16, 0b1111111111100111));
        table.insert(214, Bits::new(16, 0b1111111111101000));
        table.insert(215, Bits::new(16, 0b1111111111101001));
        table.insert(216, Bits::new(16, 0b1111111111101010));
        table.insert(217, Bits::new(16, 0b1111111111101011));
        table.insert(218, Bits::new(16, 0b1111111111101100));
        table.insert(225, Bits::new(14, 0b11111111100000));
        table.insert(226, Bits::new(16, 0b1111111111101101));
        table.insert(227, Bits::new(16, 0b1111111111101110));
        table.insert(228, Bits::new(16, 0b1111111111101111));
        table.insert(229, Bits::new(16, 0b1111111111110000));
        table.insert(230, Bits::new(16, 0b1111111111110001));
        table.insert(231, Bits::new(16, 0b1111111111110010));
        table.insert(232, Bits::new(16, 0b1111111111110011));
        table.insert(233, Bits::new(16, 0b1111111111110100));
        table.insert(234, Bits::new(16, 0b1111111111110101));
        table.insert(240, Bits::new(10, 0b1111111010));
        table.insert(241, Bits::new(15, 0b111111111000011));
        table.insert(242, Bits::new(16, 0b1111111111110110));
        table.insert(243, Bits::new(16, 0b1111111111110111));
        table.insert(244, Bits::new(16, 0b1111111111111000));
        table.insert(245, Bits::new(16, 0b1111111111111001));
        table.insert(246, Bits::new(16, 0b1111111111111010));
        table.insert(247, Bits::new(16, 0b1111111111111011));
        table.insert(248, Bits::new(16, 0b1111111111111100));
        table.insert(249, Bits::new(16, 0b1111111111111101));
        table.insert(250, Bits::new(16, 0b1111111111111110));

        table
    };
}

fn get_abs_bit_conut(num: i32) -> u32 {
    let num = num.abs();
    std::mem::size_of::<i32>() as u32 * 8 - num.leading_zeros()
}

fn get_low_n_bits(num: u32, n: usize) -> u16 {
    (num & (2_u32.pow(n as u32) - 1)) as u16
}

pub fn encode_dc(dc: i32) -> Bits {
    let amplitude = get_abs_bit_conut(dc) as u8;
    let codeword = LUMINANCE_DC_TABLE.get(&amplitude);

    let ones_complements = if dc < 0 { dc - 1 } else { dc };
    if let Some(codeword) = codeword {
        *codeword + Bits::new(amplitude, ones_complements as u32)
    } else {
        panic!("No such DC value!");
    }
}

pub fn encode_ac(run_length: u8, ac: i32) -> Bits {
    let size = get_abs_bit_conut(ac) as u8;
    let symbol1 = run_length << 4 | size;

    let codeword = LUMINANCE_AC_TABLE.get(&symbol1);
    let ones_complements = if ac < 0 { ac - 1 } else { ac };
    if let Some(codeword) = codeword {
        *codeword + Bits::new(size, ones_complements as u32)
    } else {
        panic!("No such AC value!");
    }
}

pub fn encode(squence: &[i32], bits: &mut Bits, prev_dc: i32) -> Vec<u8> {
    let mut result = Vec::new();

    let mut run_length = 0;
    for (index, num) in squence.iter().enumerate() {
        let mut encode = Bits::new(0, 0);
        if index == 0 {
            encode = encode_dc(*num - prev_dc);
        } else {
            // Do not record when encounter 0
            if *num == 0 {
                if run_length < 15 {
                    run_length += 1;
                }
            } else {
                encode = encode_ac(run_length, *num);
                run_length = 0;
            }
        }
        *bits += encode;
        let mut bytes = bits.dump();
        result.append(&mut bytes);
    }
    // Deal with runlength != 0
    if run_length != 0 {
        *bits += Bits {length: 4, bits: 0xa0000000};
        println!("{}", bits);
        let mut last_byte = bits.dump();
        result.append(&mut last_byte);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_dc() {
        assert_eq!(encode_dc(2), Bits{ length: 5, bits: 0b01110000000000000000000000000000 });
    }

    #[test]
    fn test_encode_ac() {
        assert_eq!(encode_ac(0, 16), Bits{ length: 10, bits: 0b11010100000000000000000000000000 });
    }

    #[test]
    fn test_encode_sequence() {
        let test_sequence = [2, 16, -21, 10, -15, 0, 0, 0, 3, -2, 0];
        encode(&test_sequence, &mut Bits::new(0, 0), 0);
    }
}