use super::{Image, Serializable};

use super::dct::get_dct;
use super::huffman::{LUMINANCE_AC_SPEC, LUMINANCE_DC_SPEC};
use super::quant::quant;
use super::quant::QUANT_TABLE;
use super::rle::{encode, Bits};

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
    // length: Option<u16>,     // This field can be deduced by payload
    payload: Option<T>,
}

// Quantization tables
struct DQT {
    // The following field is imported from `quant` module
    // quant_table: [[[u8; 8]; 8]; 2]
}

// TODO: RGB
struct SOF0 {
    depth: u8,
    width: u16,
    height: u16,
    channel: u8,
}

// Huffman tables
// TODO: RGB
struct DHT {
    channel: u8,
    // The following field is imported from `huffman` module
    // huffman_tables: [HuffmanSpec]
}

// Start Of Scanning
// TODO: Remove redudant fields
struct SOS {
    width: u16,
    height: u16,
    data: Vec<u8>,
}

impl JPEG {
    pub fn new(width: u16, height: u16, data: &Vec<u8>) -> Self {
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
                    channel: 1,
                }),
            },
            huffman_tables: Segment {
                marker: [0xff, 0xc4],
                payload: Some(DHT { channel: 1 }),
            },
            image_data: Segment {
                marker: [0xff, 0xda],
                payload: Some(SOS {
                    width,
                    height,
                    data: data.to_vec(),
                }),
            },
        }
    }
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
            bytes.extend(payload.get_bytes());  // Effective data
        }

        bytes
    }
}

impl Serializable for DQT {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Index 0
        bytes.push(0_u8);
        bytes.extend(QUANT_TABLE[0].iter().flatten());
        // Index 1
        bytes.push(1_u8);
        bytes.extend(QUANT_TABLE[1].iter().flatten());

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
        bytes.push(self.channel);

        // No subsampling for grayscale image
        bytes.extend([0x1, 0x11, 0x00]);

        bytes
    }
}

impl Payload for SOF0 {
    fn get_length(&self) -> u16 {
        6 + 3
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

        bytes
    }
}

impl Payload for DHT {
    fn get_length(&self) -> u16 {
        // 1: header
        // 16: count array length
        (1 + 16 + LUMINANCE_DC_SPEC.value.len() as u16)
            + (1 + 16 + LUMINANCE_AC_SPEC.value.len() as u16)
    }
}

impl Serializable for SOS {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend([0x01, 0x01, 0x00, 0x00, 0x3f, 0x00]);

        let mut prev_dc = 0;
        let mut bits = Bits::new(0, 0);
        // Process every block
        for start_y in (0..self.height).step_by(8) {
            for start_x in (0..self.width).step_by(8) {
                let block = get_block(&self, start_x as usize, start_y as usize);
                // DCT -> ZigZag -> Quantization -> Huffman
                let dct = get_dct(block);
                let zig_zag = to_zig_zag(dct);
                let (sequence, dc) = quant(zig_zag);
                let mut encoded = encode(&sequence, &mut bits, prev_dc);
                bytes.append(&mut encoded);

                prev_dc = dc;
            }
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
        6
    }
}

fn get_block(image: &SOS, start_x: usize, start_y: usize) -> [i32; 64] {
    let mut block = [0; 64];

    // Pad the edge
    for y in 0..8 {
        for x in 0..8 {
            let offset_y = std::cmp::min(start_y + y, image.height as usize);
            let offset_x = std::cmp::min(start_x + x, image.width as usize);
            block[y * 8 + x] = image.data[offset_y * image.width as usize + offset_x] as i32;
        }
    }

    block
}

fn to_zig_zag(array: [f64; 64]) -> [f64; 64] {
    let mut result = [0.; 64];

    for index in 0..64 {
        result[index] = array[ZIG_ZAG_ORDER[index]];
    }

    result
}
