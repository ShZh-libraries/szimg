use super::{Image, Serializable};

use super::common::Bits;
use super::dct::get_dct;
use super::huffman::{
    CHROMINANCE_AC_SPEC, CHROMINANCE_DC_SPEC, LUMINANCE_AC_SPEC, LUMINANCE_DC_SPEC,
};
use super::quant::{quant, LUMINANCE_QUANT_TABLE, CHROMINANCE_QUANT_TABLE};
use super::rle::encode;

// Pre-defxined zig-zag order index for array
const ZIG_ZAG_ORDER: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10,
	17, 24, 32, 25, 18, 11, 4, 5,
	12, 19, 26, 33, 40, 48, 41, 34,
	27, 20, 13, 6, 7, 14, 21, 28,
	35, 42, 49, 56, 57, 50, 43, 36,
	29, 22, 15, 23, 30, 37, 44, 51,
	58, 59, 52, 45, 38, 31, 39, 46,
	53, 60, 61, 54, 47, 55, 62, 63,
];

pub struct JPEG {
    // start_of_image: Segment<SOI>
    quant_tables: Segment<DQT>,
    start_of_frame0: Segment<SOF0>,
    huffman_tables: Segment<DHT>,
    image_data: Segment<SOS>,
    // end_of_image: Segment<EOI>
}

struct Segment<T: Payload> {
    marker: [u8; 2],
    // The following field can be deduced by payload
    // length: Option<u16>,
    payload: Option<T>,
}

// Quantization tables
struct DQT {
    // The following field is imported from `quant` module
    // quant_table: [[[u8; 8]; 8]; 2]
}

struct SOF0 {
    depth: u8,
    width: u16,
    height: u16,
    component: u8,
}

// Huffman tables
struct DHT {
    component: u8,
    // The following field is imported from `huffman` module
    // huffman_tables: [HuffmanSpec]
}

// Start Of Scanning
// TODO: Remove redudant fields
struct SOS {
    width: u16,
    height: u16,
    component: u8,
    data: Vec<u8>,
}

impl JPEG {
    pub fn new(width: u16, height: u16, component: u8, data: &Vec<u8>) -> Self {
        Self {
            quant_tables: Segment {
                marker: [0xff, 0xdb],
                payload: Some(DQT {}),
            },
            start_of_frame0: Segment {
                marker: [0xff, 0xc0],
                payload: Some(SOF0 {
                    depth: 8,
                    width,
                    height,
                    component,
                }),
            },
            huffman_tables: Segment {
                marker: [0xff, 0xc4],
                payload: Some(DHT { component }),
            },
            image_data: Segment {
                marker: [0xff, 0xda],
                payload: Some(SOS {
                    width,
                    height,
                    component,
                    data: data.to_vec(),
                }),
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Luminance,
    Chromiance,
}

impl Serializable for JPEG {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // SOI marker
        bytes.extend([0xff, 0xd8]);
        bytes.extend(self.quant_tables.get_bytes());
        bytes.extend(self.start_of_frame0.get_bytes());
        bytes.extend(self.huffman_tables.get_bytes());
        bytes.extend(self.image_data.get_bytes());
        // EOI marker
        bytes.extend([0xff, 0xd9]);

        bytes
    }
}

impl Image for JPEG {}

trait Payload: Serializable {
    fn get_length(&self) -> u16;
}

impl<T> Serializable for Segment<T>
where
    T: Payload,
{
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Marker header
        bytes.extend(self.marker);
        if let Some(payload) = &self.payload {
            // Marker length
            let length = payload.get_length() + 2; // include length's own space(2 byte) as well
            bytes.extend(length.to_be_bytes()); // High byte first
            bytes.extend(payload.get_bytes()); // Effective data
        }

        bytes
    }
}

impl Serializable for DQT {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Index 0
        bytes.push(0_u8);
        bytes.extend(LUMINANCE_QUANT_TABLE.iter().flatten());
        // Index 1
        bytes.push(1_u8);
        bytes.extend(CHROMINANCE_QUANT_TABLE.iter().flatten());

        bytes
    }
}

impl Payload for DQT {
    fn get_length(&self) -> u16 {
        // 2: 2 tables(Luminance and chromiance)
        // 1: index of table
        // 64: 64 bytes in each table
        2 * (1 + 64)
    }
}

impl Serializable for SOF0 {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.depth);
        bytes.extend(self.height.to_be_bytes());
        bytes.extend(self.width.to_be_bytes());
        bytes.push(self.component);

        if self.component == 1 {
            // No subsampling for grayscale image
            bytes.extend([0x1, 0x11, 0x00]);
        } else {
            // 4:2:0 subsampling for other image
            bytes.extend([0x01, 0x22, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01]);
        }

        bytes
    }
}

impl Payload for SOF0 {
    fn get_length(&self) -> u16 {
        // See Serializable traits for SOF0 for more details
        6 + 3 * self.component as u16
    }
}

impl Serializable for DHT {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(0x00);
        bytes.extend(LUMINANCE_DC_SPEC.count);
        bytes.extend(&LUMINANCE_DC_SPEC.value);

        bytes.push(0x10);
        bytes.extend(LUMINANCE_AC_SPEC.count);
        bytes.extend(&LUMINANCE_AC_SPEC.value);

        // None-gray scale image
        // Put Chrominace table
        if self.component != 1 {
            bytes.push(0x01);
            bytes.extend(CHROMINANCE_DC_SPEC.count);
            bytes.extend(&CHROMINANCE_DC_SPEC.value);

            bytes.push(0x11);
            bytes.extend(CHROMINANCE_AC_SPEC.count);
            bytes.extend(&CHROMINANCE_AC_SPEC.value);
        }

        bytes
    }
}

impl Payload for DHT {
    fn get_length(&self) -> u16 {
        // 1: header
        // 16: count array length
        let luminance_length = (1 + 16 + LUMINANCE_DC_SPEC.value.len() as u16)
            + (1 + 16 + LUMINANCE_AC_SPEC.value.len() as u16);
        if self.component == 1 {
            return luminance_length;
        }
        let chromiance_length = (1 + 16 + CHROMINANCE_DC_SPEC.value.len() as u16)
            + (1 + 16 + CHROMINANCE_AC_SPEC.value.len() as u16);
        luminance_length + chromiance_length
    }
}

impl SOS {
    // Porcess every block and add byte to the reference
    // Return remained bits
    fn process_gray_blocks(&self, bytes: &mut Vec<u8>) -> Bits {
        let mut prev_dc = 0;
        let mut bits = Bits::new(0, 0);
        // Process every 8x8 block
        for start_y in (0..self.height).step_by(8) {
            for start_x in (0..self.width).step_by(8) {
                let block = self.get_gray_block(start_x as usize, start_y as usize);
                prev_dc = dump_bytes(block, prev_dc, Mode::Luminance, bytes, &mut bits);
            }
        }

        bits
    }

    fn process_rgba_blocks(&self, bytes: &mut Vec<u8>) -> Bits {
        let (mut prev_y_dc, mut prev_cb_dc, mut prev_cr_dc) = (0, 0, 0);
        let mut bits = Bits::new(0, 0);

        // Process order:
        // Y blocks: 16 x 16 (which will be then divided into 4 8x8 blocks)
        // Cb blocks: 8 x 8 (Subsampled from the 16 x 16 block)
        // Cr blocks: 8 x 8 (Subsampled from the 16 x 16 block)
        for start_y in (0..self.height).step_by(16) {
            for start_x in (0..self.width).step_by(16) {
                // Get four 4x4 blocks array from origin 16x16 block
                let (y_blocks, cb_blocks, cr_blocks) =
                    self.convert_rgb_blocks_to_ycbcr_blocks(start_x.into(), start_y.into());
                // Divide 16x16 blocks into 4 4x4 blocks
                for index in 0..4 {
                    prev_y_dc = dump_bytes(
                        y_blocks[index],
                        prev_y_dc,
                        Mode::Luminance,
                        // The following 2 params are only to store the state
                        // If you just want to read the code, just ignore them
                        bytes,
                        &mut bits,
                    );
                }
                let subsampled_cb_block = subsampling(cb_blocks);
                prev_cb_dc = dump_bytes(
                    subsampled_cb_block,
                    prev_cb_dc,
                    Mode::Chromiance,
                    bytes,
                    &mut bits,
                );
                let subsampled_cr_block = subsampling(cr_blocks);
                prev_cr_dc = dump_bytes(
                    subsampled_cr_block,
                    prev_cr_dc,
                    Mode::Chromiance,
                    bytes,
                    &mut bits,
                );
            }
        }

        bits
    }

     // Get four 4x4 blocks array from origin 16x16 block
    fn convert_rgb_blocks_to_ycbcr_blocks(
        &self,
        start_x: usize,
        start_y: usize,
    ) -> ([[i32; 64]; 4], [[i32; 64]; 4], [[i32; 64]; 4]) {
        let (mut y_blocks, mut cb_blocks, mut cr_blocks) = ([[0; 64]; 4], [[0; 64]; 4], [[0; 64]; 4]);

        for y in (0..16).step_by(8) {
            for x in (0..16).step_by(8) {
                // Calculate the position of one of the blocks
                let block_start_y = std::cmp::min(start_y + y, self.height as usize - 1);
                let block_start_x = std::cmp::min(start_x + x, self.width as usize - 1);

                // Insert position of the final 4 elements array
                let index = (2 * y + x) / 8;
                // Get 8x8 component by given start x and start y
                let (y_block, cb_block, cr_block) =
                    self.get_ycbcr_block(block_start_x, block_start_y);
                y_blocks[index] = y_block;
                cb_blocks[index] = cb_block;
                cr_blocks[index] = cr_block;
            }
        }

        (y_blocks, cb_blocks, cr_blocks)
    }

    fn get_gray_block(&self, start_x: usize, start_y: usize) -> [i32; 64] {
        let mut block = [0; 64];
        // Pad the edge
        for row_index in 0..8 {
            for column_index in 0..8 {
                // Map to self.data by using offset pair
                let offset_y = std::cmp::min(start_y + row_index, self.height as usize - 1);
                let offset_x = std::cmp::min(start_x + column_index, self.width as usize - 1);
                block[row_index * 8 + column_index] = self.data[offset_y * self.width as usize + offset_x] as i32;
            }
        }
        block
    }

    fn get_ycbcr_block(&self, start_x: usize, start_y: usize) -> ([i32; 64], [i32; 64], [i32; 64]) {
        let (mut y_block, mut cb_block, mut cr_block) = ([0; 64], [0; 64], [0; 64]);

        for row_index in 0..8 {
            for column_index in 0..8 {
                let offset_y = std::cmp::min(start_y + row_index, self.height as usize - 1);
                let offset_x = std::cmp::min(start_x + column_index, self.width as usize - 1);
                let offset = (offset_y * self.width as usize + offset_x) * 4; // RGBA 4 channels

                let (r, g, b) = (
                    self.data[offset],
                    self.data[offset + 1],
                    self.data[offset + 2],
                );
                let (y, cb, cr) = rgb_2_ycbcr(r, g, b);
                y_block[row_index * 8 + column_index] = y as i32;
                cb_block[row_index * 8 + column_index] = cb as i32;
                cr_block[row_index * 8 + column_index] = cr as i32;
            }
        }

        (y_block, cb_block, cr_block)
    }
}

fn dump_bytes(
    block: [i32; 64],
    prev_dc: i32,
    mode: Mode,
    // These two params are to store the state
    bytes: &mut Vec<u8>,
    bits: &mut Bits,
) -> i32 {
    // DCT -> ZigZag -> Quantization -> Huffman
    let dct = get_dct(block);
    let zig_zag = to_zig_zag(dct);
    let (sequence, dc) = quant(zig_zag, mode);
    let mut encoded = encode(&sequence, bits, prev_dc, mode);
    bytes.append(&mut encoded);

    // Return as previous DC value
    dc
}

fn subsampling(data: [[i32; 64]; 4]) -> [i32; 64] {
    let mut result = [0; 64];

    for y in 0..8 {
        for x in 0..8 {
            let offset_y = 2 * (y / 4) + x / 4;
            let offset_x = 2 * x % 8;
            result[y * 8 + x] = (data[offset_y][offset_x]
                + data[offset_y][offset_x + 1]
                + data[offset_y][offset_x + 8]
                + data[offset_y][offset_x + 9]
                + 2)
                / 4;
        }
    }

    result
}

impl Serializable for SOS {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let bits: Bits;
        // Different header for different color space
        if self.component == 1 {
            bytes.extend([0x01, 0x01, 0x00, 0x00, 0x3f, 0x00]);
            bits = self.process_gray_blocks(&mut bytes);
        } else {
            bytes.extend([0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00]);
            bits = self.process_rgba_blocks(&mut bytes);
        }

        // Deal with last byte
        let (last_byte, is_complete) = bits.complete();
        if !is_complete {
            bytes.push(last_byte);
        }

        bytes
    }
}

impl Payload for SOS {
    fn get_length(&self) -> u16 {
        // The meaningless header's length
        if self.component == 1 {
            6
        } else {
            10
        }
    }
}

fn to_zig_zag(array: [f64; 64]) -> [f64; 64] {
    let mut result = [0.; 64];

    for index in 0..64 {
        result[index] = array[ZIG_ZAG_ORDER[index]];
    }

    result
}

fn rgb_2_ycbcr(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    // Round to nearest, prevent 0.999 is casted to 0
    // See JFIF specification for more details about the following formula
    (
        (0.2990 * r as f64 + 0.5870 * g as f64 + 0.1140 * b as f64).round() as u8,
        (-0.1687 * r as f64 - 0.3313 * g as f64 + 0.5000 * b as f64 + 128.).round() as u8,
        (0.5000 * r as f64 - 0.4187 * g as f64 - 0.0813 * b as f64 + 128.).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_block_ycbr() {
        let test = SOS {
            width: 16,
            height: 16,
            component: 3,
            data: vec![1; 16 * 16 * 4],
        };

        let result = test.convert_rgb_blocks_to_ycbcr_blocks(0, 0);
        println!("{:?}", result);
    }
}
