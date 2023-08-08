/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::f32;

/// Sigmoid function
pub fn sigmoid<T: Into<f32>>(x: T) -> f32 {
    let x_f64 = x.into();
    1.0 / (1.0 + (-x_f64).exp())
}

/// Sigmoid derivative function
pub fn sigmoid_derivative<T: Into<f32> + Copy>(x: T) -> f32 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

/// Calculates the dot product of 2 vectors
pub fn dot_product<T: Into<f32> + Copy>(x: &[T], y: &[T]) -> f32 {
    x.iter()
        .zip(y.iter())
        .map(|(&i, &j)| i.into() * j.into())
        .sum()
}
