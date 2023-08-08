# ml.rs documentation

### - `sigmoid(x : f32) -> f32`

Sigmoid

### - `sigmoid_derivative(x : f32) -> f32`

Sigmoid derivative 

### - `loss(predicted: &[f32], target: &[f32]) -> f32`

Calculates the loss (or "cost") of a neural network

### - `compute(weights: &[Vec<Vec<f32>>], biases: &[Vec<f32>], input: &[f32]) -> Vec<Vec<f32>>`

Computes the neuron values in a neural network with the given biases and inputs

### - `generate_nn_w(hidden_layers: &[usize], input_size: usize, output_size: usize) -> Vec<Vec<Vec<f32>>>`

Generates weights for a neural network

### - `generate_nn_b(hidden_layers: &[usize], input_size: usize, output_size: usize) -> Vec<Vec<f32>>`

Generates biases for a neural network

### - `generate_nn_wb(hidden_layers: &[usize], input_size: usize, output_size: usize) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>)`

Generates the weights and biases for a neural network