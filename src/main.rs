use anyhow::*;
use core::time;
use std::io::ErrorKind;
use std::result::Result::Ok;

use std::thread::sleep;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

#[cfg(target_os = "wasi")]
use std::os::wasi::io::RawFd;

fn main() -> Result<()> {
    let listeners = get_tcplistener();
    println!("{:?}", listeners);
    let mut idx = 0usize;
    loop {
        let listener = &listeners[idx];

        match listener.accept() {
            Ok((stream, _)) => {
                handle_connection(stream);
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                sleep(time::Duration::from_micros(1))
            }
            Err(err) => panic!("{}", err),
        }

        idx = if (idx + 1) == listeners.len() {
            0
        } else {
            idx + 1
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handle connection");
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn get_tcplistener() -> Vec<TcpListener> {
    #[cfg(target_os = "wasi")]
    return get_tcplistener_wasi();

    #[cfg(not(target_os = "wasi"))]
    return get_tcplistener_bind();
}

#[cfg(not(target_os = "wasi"))]
fn get_tcplistener_bind() -> Vec<TcpListener> {
    let mut listeners = Vec::new();
    listeners.push(TcpListener::bind("127.0.0.1:25565").expect("Could not bind address"));
    listeners.push(TcpListener::bind("127.0.0.1:25566").expect("Could not bind address"));
    listeners
}

#[cfg(target_os = "wasi")]
fn get_tcplistener_wasi() -> Vec<TcpListener> {
    use std::os::wasi::prelude::FromRawFd;
    enumerate_tcp_fd()
        .into_iter()
        .map(|raw_fd| unsafe { TcpListener::from_raw_fd(raw_fd) })
        .collect()
}

#[cfg(target_os = "wasi")]

fn enumerate_tcp_fd() -> Vec<RawFd> {
    use wasi::fd_fdstat_get;
    use wasi::FILETYPE_SOCKET_STREAM;

    let mut stats = Vec::<RawFd>::new();
    for i in 1u32..1024 {
        if let Ok(fdstat) = unsafe { fd_fdstat_get(i) } {
            if fdstat.fs_filetype == FILETYPE_SOCKET_STREAM {
                stats.push(i as RawFd)
            }
        } else {
            break;
        }
    }

    stats
}
