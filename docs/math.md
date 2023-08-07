## Math.rs documentation

### - `sigmoid(x : f32) -> f32`

### - `sigmoid_derivative(x : f32) -> f32`

### - `dot_product(x : &[f32], y : &[f32]) -> f32`


### - `random_f32(x : Range<f32, f32>) -> f32`

Generates a random f32 number in the provided range

### - `random_f64(x : Range<f64, f64>) -> f64`

Generates a random f64 number in the provided range

### - `random_vecf32(size : usize, x : Range<f32, f32>) -> f32`

Generates a random f32 vector in the provided range

### - `random_vecf64(size : usize, x : Range<f64, f64>) -> f64`

Generates a random f64 vector in the provided range


### - `rand_bytes(buffer : &mut [u8])`

Returns a random byte (Random u8). This is the fastest random generation of them all, but is less convenient to use.
