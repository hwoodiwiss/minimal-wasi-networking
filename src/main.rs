use anyhow::*;
use core::time;
use std::io::ErrorKind;
use std::result::Result::Ok;

use std::thread::sleep;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let listener = get_tcplistener();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                sleep(time::Duration::from_millis(5))
            }
            Err(err) => panic!("{}", err),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handle connection");
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn get_tcplistener() -> TcpListener {
    #[cfg(target_os = "wasi")]
    return get_tcplistener_wasi();

    #[cfg(not(target_os = "wasi"))]
    return get_tcplistener_bind();
}

#[cfg(target_os = "wasi")]
fn get_tcplistener_wasi() -> TcpListener {
    use std::os::wasi::prelude::{FromRawFd, RawFd};

    let sock: RawFd = 3i32;
    unsafe { TcpListener::from_raw_fd(sock) }
}

#[cfg(not(target_os = "wasi"))]
fn get_tcplistener_bind() -> TcpListener {
    TcpListener::bind("127.0.0.1:25565").expect("Could not bind address")
}
