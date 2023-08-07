/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::ops::Range;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a random byte (u8)
pub fn random_byte() -> u8 {
    let mut state : u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos() as u64;
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((state >> 32) as u8).wrapping_mul(2)
}

/// Generates a random 32-bit float
pub fn random_f32(range: Range<f32>) -> f32 {
    let rand_u8: u8 = random_byte();
    let rand_f32 : f32 = range.start + (f32::from(rand_u8) / 255.0) * (range.end - range.start);
    rand_f32
}

/// Generates a random 64-bit float
pub fn random_f64(range: Range<f64>) -> f64 {
    let rand_u8: u8 = random_byte();
    range.start + (f64::from(rand_u8) / 255.0) * (range.end - range.start)
}

