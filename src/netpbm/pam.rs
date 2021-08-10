use crate::{Image, Serializable};

#[derive(PartialEq)]
pub enum TupleType {
    BlackAndWhite,
    GrayScale,
    RGB,
    BlackAndWhiteAlpha,
    GrayScaleAlpha,
    RGBAlpha,
}

impl TupleType {
    fn to_depth(&self) -> u8 {
        match self {
            TupleType::BlackAndWhite | TupleType::GrayScale => 1,
            TupleType::BlackAndWhiteAlpha | TupleType::GrayScaleAlpha => 2,
            TupleType::RGB => 3,
            TupleType::RGBAlpha => 4,
        }
    }

    // Specifiy max value for users
    fn to_max_value(&self) -> u16 {
        match self {
            TupleType::BlackAndWhite | TupleType::BlackAndWhiteAlpha => 1,
            TupleType::GrayScale
            | TupleType::GrayScaleAlpha
            | TupleType::RGB
            | TupleType::RGBAlpha => 256,
        }
    }
}

impl ToString for TupleType {
    fn to_string(&self) -> String {
        let str = match self {
            TupleType::BlackAndWhite => "BLACKANDWHITE",
            TupleType::GrayScale => "GRAYSCALE",
            TupleType::RGB => "RGB",
            TupleType::BlackAndWhiteAlpha => "BLACKANDWHITE_ALPHA",
            TupleType::GrayScaleAlpha => "GRAYSCALE_ALPHA",
            TupleType::RGBAlpha => "RGB_ALPHA",
        };

        String::from(str)
    }
}

pub struct PAM {
    width: u32,
    height: u32,
    depth: u8,
    max_value: u16, // up to 65535
    tuple_type: TupleType,
    data: Vec<u8>,
}

impl PAM {
    pub fn new(mode: TupleType, width: u32, height: u32, data: &Vec<u8>) -> Self {
        Self {
            width,
            height,
            depth: mode.to_depth(),
            max_value: mode.to_max_value(),
            tuple_type: mode,
            data: data.to_vec(),
        }
    }
}

impl Serializable for PAM {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let header = format!(
            "{magic_number}\n\
            WIDTH {width}\n\
            HEIGHT {height}\n\
            DEPTH {depth}\n\
            MAXVAL {max_value}\n\
            TUPLETYPE {tuple_type}\n\
            ENDHDR\n",
            magic_number = "P7",
            width = self.width,
            height = self.height,
            depth = self.depth,
            max_value = self.max_value,
            tuple_type = self.tuple_type.to_string()
        );
        bytes.extend(header.bytes());

        bytes.extend(self.data.to_vec());

        bytes
    }
}

impl Image for PAM {}
