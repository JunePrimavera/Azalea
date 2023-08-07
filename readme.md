# Azalea

June's standard libraries for rust projects.

Has a lot of common code that is often important, or copied to many places in a project.

# Documentation

## - File utils

### - `read_raw_bytes(file : String) -> Vec<u8>`

Reads the raw binary data from a file and returns it as a `Vec<u8>`

### - `write_raw_bytes(file : String, data : Vec<u8>)`

Just a simple wrapper for `fs::write()`, with built in error handling

## - Networking

### - `struct Address {}`

Stores an IP address as 4 `u8` values and one `u16`. The `Address` struct has a few methods implemented to make it easy to use with other types (e.g get the ip as raw bytes, a string, etc)

Implementations:
- `Address::from_string()`
- `Address::from_bytes()`
- `Address.get_string()`
- `Address.get_bytes()`

Most of these functions are pretty self-explanatory - you can get the address as a `String` value (This does not include the port, only the IP itself). You can create a new address from a `String` and a `u16` (Which is the port).

The byte format is quite simple - It's just 6 bytes, one for each number and 2 for the port, which is how IP addresses should be stored. When using `Addres::from_bytes()`, it **has** to be in this format otherwise it will be innacurate/incorrect.

### - `ssh_command(address : Address, username : &str, command : &str) -> Result<Vec<u8>, Vec<u8>> `

This function will attempt an SSH connection to the given address, and username, then run the command and return the result as a `Result<Vec<u8>, Vec<u8>` - A vector of binary values, which usually needs converting to a string. This is useful when running code on multiple devices, or using SSH through code.

## - Math

### - `struct f16 {}`

16-bit floating point numbers. Rust by default does not come with 16-bit numbers, which are much more memory efficient to operate on and store. They may be slower sometimes due to lack of hardware acceleration.

Implementations:
- `to_f32()`
- `from_f32()`

Fairly self-explanatory, you can convert a 32 bit float to a 16-bit one, as well as do all your standard operations on it (Division, multiplication, addition and subtraction).

### - `sigmoid(x : f32/f64) -> f32/f64`

There are 2 sigmoid implementations. `sigmoidf32(x : f32)` and `sigmoidf64(x :f64)`. They both behave identically - Except one allows for higher-precision floating point numbers.

### - `sigmoid_derivative(x : f32/f64) -> f32/f64`

Similar to `sigmoid()`, there's 2 different implementations. `sigmoid_derivativef32(x : f32)` and `sigmoid_derivativef64(x : f64)`. They both behave the same except one allows for higher-precision floating point numbers.

### - `dot_product(x : &[f32/f64], y : &[f32/f64]) -> f32/f64`

There are two implementations `dot_productf32(x : &[f32], y : &[f32]) -> f32` and `dot_productf6432(x : &[f64], y : &[f64]) -> f64` for different accuracies. They all compute the dot product of two vectors and behave identically to one another, except some having higher accuracies than others.

## - Device

### - `get_total_system_memory() -> u64`

Returns the total system memory.

### - `get_total_system_swap() -> u64`

Returns the total system swap memory

### - `get_total_used_system_memory() -> u64`

Returns the total memory in use

### - `get_total_used_system_swap() -> u64`

Returns the total swap memory in use

### - `get_system_uptime() -> u64`

Returns the total uptime of the device

### - `get_system_cpus() -> u64`

Returns the cpu/core count of the current device

### - `get_operating_system() -> String`

Returns the operating system version


## - Util

### - `prettify_number(x : usize) -> String`

Returns number with comma separators - e.g. it will turn 100000000 into 100,000,000. This also converts the number to a string. Useful for UI elements.

### - `randf32(x : Range<f32, f32>) -> f32`

Generates a random f32 number in the provided range

### - `randf64(x : Range<f64, f64>) -> f64`

Generates a random f64 number in the provided range

### - `rand_vecf32(x : Range<f32, f32>) -> Vec<f32>`

Generates a vector of random f32 numbers in the range - This is a lot faster than calling `randf32()` multiple times. 

### - `rand_vecf64(x : Range<f64, f64>) -> Vec<f64>`

Generates a vector of random f32 numbers in the range - This is a lot faster than calling `randf64()` multiple times.

### - `rand_byte() -> u8`

Returns a random byte (Random u8). This is the fastest random generation of them all, but is less convenient to use.

# Copyright notice:

Copyright Juniper Gardiner - MIT
Jul 24 2023
