/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates random bytes
fn generate_random_bytes(buffer: &mut [u8]) {
    let mut rng = File::open("/dev/urandom").expect("Cant access /dev/urandom!");
    rng.read_exact(buffer).expect("Error generating random bytes");
}

/// Generates a random 32-bit float
pub fn random_f32(range: Range<f32>) -> f32 {
    let mut buffer = [0u8; 4];
    generate_random_bytes(&mut buffer);
    let rand_f32 = f32::from_bits(0x3F800000 | (u32::from_le_bytes(buffer) >> 9));
    range.start + rand_f32 * (range.end - range.start)
}

/// Generates a random 64-bit float
pub fn random_f64(range: Range<f64>) -> f64 {
    let mut buffer = [0u8; 8];
    generate_random_bytes(&mut buffer);
    let rand_f64 = f64::from_bits(0x3FF0000000000000 | (u64::from_le_bytes(buffer) >> 12));
    range.start + rand_f64 * (range.end - range.start)
}

/// Generates a random 32-bit float vector
pub fn random_vecf32(values : usize, range : Range<f32>) {
    let mut vec = vec![];
    for _ in 0..values {
        vec.push(random_f32(range.clone()))
    }
}

/// Generates a random 64-bit float vector
pub fn random_vecf64(values : usize, range : Range<f64>) {
    let mut vec = vec![];
    for _ in 0..values {
        vec.push(random_f64(range.clone()))
    }
}