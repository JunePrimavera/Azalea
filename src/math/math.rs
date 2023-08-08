/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::f32;
use std::ops::Sub;

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
    x.iter().zip(y.iter()).map(|(&i, &j)| i.into() * j.into()).sum()
}

/// Loss function for neural networks (MSE)
pub fn loss(predicted: &[f32], target: &[f32]) -> f32 {
    let sum_squared_errors = predicted.iter().zip(target.iter()).map(|(&pred, &targ)| (pred - targ).powi(2)).sum::<f32>();
    sum_squared_errors / (2.0 * predicted.len() as f32)
}

/// Computes a neural network from the given weights and biases (MLP)
pub fn compute(weights: &[Vec<Vec<f32>>], biases: &[Vec<f32>], input: &[f32]) -> Vec<Vec<f32>> {
    let mut layer_outputs = Vec::with_capacity(weights.len() + 1);
    layer_outputs.push(input.to_vec());
    for (layer_weights, layer_biases) in weights.iter().zip(biases.iter()) {
        let previous_output = layer_outputs.last().unwrap();
        let mut layer_output = Vec::with_capacity(layer_weights.len());
        for neuron_weights in layer_weights {
            let weighted_sum = neuron_weights.iter().zip(previous_output.iter()).map(|(&weight, &input_value)| weight * input_value).sum::<f32>();
            let neuron_output = sigmoid(weighted_sum + layer_biases[0]);
            layer_output.push(neuron_output);
        }
        layer_outputs.push(layer_output);
    }
    layer_outputs
}