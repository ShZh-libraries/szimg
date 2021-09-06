use crate::img::{Serializable, Image};

pub struct BMP {
    header: Header,
    dib: DIB,
    data: Data,
}

struct Header {
    magic_number: [u8; 2],
    file_size: u32,
    data_offset: u32,
}

struct DIB {
    size: u32,
    width: u32,
    height: u32,
    plane_num: u16,
    depth: u16,
    compression: u32,
    data_size: u32,
    horizental_resolution: u32,
    vertical_resolution: u32,
    palette_color_num: u32,
    importance_color: u32,
}

struct Data {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl BMP {
    pub fn new(width: u32, height: u32, data: &Vec<u8>) -> Self {
        Self {
            header: Header::new(width, height),
            dib: DIB::new(width, height),
            data: Data::new(width, height, data),
        }
    }
}

impl Serializable for BMP {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.header.get_bytes());
        bytes.extend(self.dib.get_bytes());
        bytes.extend(self.data.get_bytes());

        bytes
    }
}

impl Header {
    fn new(width: u32, height: u32) -> Self {
        Self {
            magic_number: [66, 77],
            file_size: 54 + get_data_size(width, height),
            data_offset: 54,
        }
    }
}

impl Serializable for Header {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.magic_number);
        bytes.extend(self.file_size.to_le_bytes());
        bytes.extend([0; 4]);
        bytes.extend(self.data_offset.to_le_bytes());

        bytes
    }
}

impl DIB {
    fn new(width: u32, height: u32) -> Self {
        Self {
            size: 40,
            width, height,
            plane_num: 1,
            depth: 24,
            compression: 0,
            data_size: get_data_size(width, height),
            horizental_resolution: 1000,
            vertical_resolution: 1000,
            palette_color_num: 0,
            importance_color: 0,
        }
    }
}

impl Serializable for DIB {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.size.to_le_bytes());
        bytes.extend(self.width.to_le_bytes());
        bytes.extend(self.height.to_le_bytes());
        bytes.extend(self.plane_num.to_le_bytes());
        bytes.extend(self.depth.to_le_bytes());
        bytes.extend(self.compression.to_le_bytes());
        bytes.extend(self.data_size.to_le_bytes());
        bytes.extend(self.horizental_resolution.to_le_bytes());
        bytes.extend(self.vertical_resolution.to_le_bytes());
        bytes.extend(self.palette_color_num.to_le_bytes());
        bytes.extend(self.importance_color.to_le_bytes());

        bytes
    }
}

impl Data {
    fn new(width: u32, height: u32, data: &Vec<u8>) -> Self {
        Self {
            width,
            height,
            data: data.to_vec(),
        }
    }
}

impl Serializable for Data {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for y_index in 0..self.height {
            // Create a row vector with specified length due to the padding
            let row_size= (self.width as f64 * 24. / 32.).ceil() as usize * 4;
            let mut row = Vec::with_capacity(row_size);
            row.resize(row_size, 0u8);

            for x_index in 0..self.width {
                let offset = (self.height - 1 - y_index) * self.width + x_index;
                // BGR order
                row[x_index as usize * 3 + 0] = self.data[(offset * 3 + 2) as usize];
                row[x_index as usize * 3 + 1] = self.data[(offset * 3 + 1) as usize];
                row[x_index as usize * 3 + 2] = self.data[(offset * 3 + 0) as usize];
            }

            bytes.extend(row);
        }

        bytes
    }
}

fn get_data_size(width: u32, height: u32) -> u32 {
    // Each pixel have 24 bits
    // And a row will padding to multiple of 4
    let row_size = (width as f64 * 24. / 32.).ceil() as u32 * 4;
    let total_size = height * row_size;

    total_size
}

impl Image for BMP {}
