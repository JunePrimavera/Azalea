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
