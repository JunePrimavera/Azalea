## - File utils.rs

### - `read_raw_bytes(file : String) -> Vec<u8>`

Reads the raw binary data from a file and returns it as a `Vec<u8>`

### - `write_raw_bytes(file : String, data : Vec<u8>)`

Just a simple wrapper for `fs::write()`, with built in error handling
