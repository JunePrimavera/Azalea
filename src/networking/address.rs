/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::ops::Add;

// Stores IP address as struct (v4 only)
pub struct Address {
    pub n: [u8; 4],
    pub p: u16,
}
impl Address {
    /// Create IP from string value and port
    pub fn from_string(address: String, port: u16) -> Address {
        let address_n: Vec<u8> = address
            .split('.')
            .map(|i| i.parse::<u8>().unwrap())
            .collect();
        Address {
            n: [address_n[0], address_n[1], address_n[2], address_n[3]],
            p: port,
        }
    }
    /// Return string value
    pub fn get_string(&self) -> String {
        return self.n[0]
            .to_string()
            .add(".")
            .add(self.n[1].to_string().as_str())
            .add(".")
            .add(self.n[2].to_string().as_str())
            .add(".")
            .add(self.n[3].to_string().as_str());
    }
    /// Return raw bytes
    pub fn get_bytes(&self) -> Vec<u8> {
        let port_b: [u8; 2] = self.p.to_le_bytes();
        Vec::from([
            self.n[0].to_le_bytes()[0],
            self.n[1].to_le_bytes()[0],
            self.n[2].to_le_bytes()[0],
            self.n[3].to_le_bytes()[0],
            port_b[0],
            port_b[1],
        ])
    }
    /// Create a new Address struct from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Address {
        Address {
            n: [
                u8::from_le_bytes([bytes[0]]),
                u8::from_le_bytes([bytes[1]]),
                u8::from_le_bytes([bytes[2]]),
                u8::from_le_bytes([bytes[3]]),
            ],
            p: u16::from_le_bytes([bytes[4], bytes[5]]),
        }
    }
}

impl Clone for Address {
    fn clone(&self) -> Address {
        Address {
            n: self.n,
            p: self.p,
        }
    }
}
