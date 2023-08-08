/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::fs;
use std::fs::{File, Metadata};
use std::io::Read;

/// Reads raw bytes from a file
pub fn read_raw_bytes(file: String) -> Vec<u8> {
    let mut f: File = File::open(&file).expect("File does not exist!");
    let metadata: Metadata =
        fs::metadata(&file).expect("Can't read file metadata! - Could be corrupted");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("Buffer overflow!");
    buffer
}
/// Writes raw bytes to a file
pub fn write_raw_bytes(file: String, data: Vec<u8>) {
    fs::write(file, data).expect("Could not write to file!");
}
