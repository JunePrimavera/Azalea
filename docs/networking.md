## - Networking.rs

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
