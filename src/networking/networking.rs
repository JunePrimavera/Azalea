/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

extern crate ssh2;

use networking::address::Address;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

/// Runs command through SSH on the target device
pub fn ssh_command(address: Address, username: &str, command: &str) -> Result<Vec<u8>, Vec<u8>> {
    let host: String = address.get_string();
    let port: u16 = address.p;
    let tcp: TcpStream = TcpStream::connect(format!("{}:{}", host, port))
        .map_err(|e| format!("Failed to connect: {}", e))?;
    let mut sess: Session = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| format!("SSH handshake failed: {}", e))?;
    sess.userauth_agent(username)
        .map_err(|e| format!("Failed to authenticate: {}", e))?;
    if !sess.authenticated() {
        return Err(Vec::from("Authentication failed".to_string()));
    }
    let mut channel = sess.channel_session().unwrap();
    channel
        .exec(command)
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    let mut output: Vec<u8> = Vec::new();
    channel
        .read_to_end(&mut output)
        .map_err(|e| format!("Failed to read channel: {}", e))?;
    channel.send_eof().ok();
    channel.wait_close().ok();
    sess.disconnect(None, "Bye bye", Option::from("en_us")).ok();
    Ok(output.to_vec())
}
