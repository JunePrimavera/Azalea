/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::f32;


/// Calculates the dot product of 2 vectors
pub fn dot_product<T: Into<f32> + Copy>(x: &[T], y: &[T]) -> f32 {
    x.iter()
        .zip(y.iter())
        .map(|(&i, &j)| i.into() * j.into())
        .sum()
}

/// ReLu function
pub fn relu(x: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

/// ReLu derivative
pub fn relu_derivative(x: &[f32], y: &[f32]) -> f32 {
    let weighted_sum = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| xi * yi)
        .sum::<f32>();
    if weighted_sum > 0.0 {
        1.0
    } else {
        0.0
    }
}