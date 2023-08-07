/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::f32;
use std::f64;

/// Sigmoid function (f32)
pub fn sigmoidf32(x : f32) -> f32 { 1.0 / (1.0 + f32::consts::E.powf(-x)) }
/// Sigmoid function (f64)
pub fn sigmoidf64(x : f64) -> f64 { 1.0 / (1.0 + f64::consts::E.powf(-x)) }

/// Sigmoid derivative function (f32)
pub fn sigmoid_derivativef32(x : f32) -> f32 {
    let s : f32 = sigmoidf32(x);
    s * (1.0 - s)
}
/// Sigmoid derivative function (f64)
pub fn sigmoid_derivativef64(x : f64) -> f64 {
    let s : f64 = sigmoidf64(x);
    s * (1.0 - s)
}

/// Calculates the dot product of 2 vectors (f32)
pub fn dot_productf32(x: &[f32], y: &[f32]) -> f32 {
    return x.iter().zip(y.iter()).map(|(&i, &j)| i * j).sum()
}
/// Calculates the dot product of 2 vectors (f64)
pub fn dot_productf64(x: &[f64], y: &[f64]) -> f64 {
    return x.iter().zip(y.iter()).map(|(&i, &j)| i * j).sum()
}


