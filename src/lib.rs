use std::error::Error;

pub trait Image {
    fn get_bytes(&self) -> Vec<u8>;

    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>>;
}
