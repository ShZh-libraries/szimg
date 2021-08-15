use std::fs::File;
use std::io::Read;

pub fn diff_file(file1_path: &str, file2_path: &str) -> bool {
    let mut file1 = File::open(file1_path).unwrap();
    let mut buffer1 = Vec::new();
    file1.read_to_end(&mut buffer1);

    let mut file2 = File::open(file2_path).unwrap();
    let mut buffer2 = Vec::new();
    file2.read_to_end(&mut buffer2);

    buffer1 == buffer2
}