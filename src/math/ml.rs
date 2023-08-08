/*
Copyright Juniper Gardiner - MIT
Aug 8 2023
*/

use math::math::sigmoid;
use math::rand::random_vecf32;

/// Loss function for neural networks (MSE)
pub fn loss(predicted: &[f32], target: &[f32]) -> f32 {
    predicted
        .iter()
        .zip(target.iter())
        .map(|(&pred, &targ)| (pred - targ).powi(2))
        .sum::<f32>()
        / (2.0 * predicted.len() as f32)
}

/// Computes a neural network from the given weights and biases (MLP)
pub fn compute(weights: &[Vec<Vec<f32>>], biases: &[Vec<f32>], input: &[f32]) -> Vec<Vec<f32>> {
    let mut layer_outputs: Vec<Vec<f32>> = Vec::new();
    layer_outputs.push(input.to_vec());
    for (layer_weights, layer_biases) in weights.iter().zip(biases.iter()) {
        let mut layer_output = Vec::with_capacity(layer_weights.len());
        for neuron_weights in layer_weights {
            let weighted_sum = neuron_weights
                .iter()
                .zip(layer_outputs.last().unwrap().iter())
                .map(|(&weight, &input_value)| weight * input_value)
                .sum::<f32>()
                + layer_biases[0];
            layer_output.push(sigmoid(weighted_sum));
        }
        layer_outputs.push(layer_output);
    }
    layer_outputs
}

/// Generate neural network's weights
pub fn generate_nn_w(
    hidden_layers: &[usize],
    input_size: usize,
    output_size: usize,
) -> Vec<Vec<Vec<f32>>> {
    let mut layer_size = Vec::new();
    layer_size.push(input_size);
    layer_size.extend_from_slice(hidden_layers);
    layer_size.push(output_size);
    let mut weights = Vec::new();
    for i in 0..layer_size.len() - 1 {
        let mut layer_weight = Vec::new();
        for _ in 0..layer_size[i + 1] {
            layer_weight.push(random_vecf32(layer_size[i], -1.0..1.0));
        }
        weights.push(layer_weight);
    }
    weights
}

/// Generate neural network's biases
pub fn generate_nn_b(
    hidden_layers: &[usize],
    input_size: usize,
    output_size: usize,
) -> Vec<Vec<f32>> {
    let mut layer_size = Vec::new();
    layer_size.push(input_size);
    layer_size.extend_from_slice(hidden_layers);
    layer_size.push(output_size);
    let mut biases = Vec::new();
    for i in 0..layer_size.len() - 1 {
        biases.push(random_vecf32(layer_size[i + 1], -1.0..1.0));
    }
    biases
}

/// Generate neural network's weights + biases
pub fn generate_nn_wb(
    hidden_layers: &[usize],
    input_size: usize,
    output_size: usize,
) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>) {
    (
        generate_nn_w(hidden_layers, input_size, output_size),
        generate_nn_b(hidden_layers, input_size, output_size),
    )
}

/// Back propagation function
pub fn backpropagation(
    weights: &mut [Vec<Vec<f32>>],
    biases: &mut [Vec<f32>],
    input: &[f32],
    target: &[f32],
    learning_rate: f32,
) {
    let layer_outputs = compute(weights, biases, input);
    let output_layer_delta = layer_outputs
        .last()
        .unwrap()
        .iter()
        .zip(target.iter())
        .map(|(output, target)| output - target)
        .collect::<Vec<f32>>();
    let mut layer_deltas = vec![output_layer_delta];
    for i in (1..weights.len()).rev() {
        let delta = &layer_deltas.last().unwrap();
        let weights = &weights[i];
        let mut layer_delta = Vec::with_capacity(weights[0].len());
        for j in 0..weights[0].len() {
            let weighted_delta = weights.iter().enumerate()
                .map(|(i, neuron_weights)| neuron_weights[j] * delta[i])
                .sum();
            layer_delta.push(weighted_delta);
        }

        layer_deltas.push(layer_delta);
    }
    layer_deltas.reverse();
    for i in 0..weights.len() {
        let prev_output = if i == 0 { input } else { &layer_outputs[i - 1] };
        for j in 0..weights[i].len() {
            for k in 0..weights[i][j].len() {
                let mut weighted_sum = 0.0;
                for (_l, v) in prev_output.iter().enumerate() {
                    weighted_sum += weights[i][j][k] * *v;
                }
                let gradient = learning_rate
                    * layer_deltas[i][j]
                    * (weighted_sum / prev_output.len() as f32);
                weights[i][j][k] -= gradient;
            }
        }
    }
    for i in 0..biases.len() {
        for j in 0..biases[i].len() {
            let bias_gradient = learning_rate * layer_deltas[i][j];
            biases[i][j] -= bias_gradient;
        }
    }
}
