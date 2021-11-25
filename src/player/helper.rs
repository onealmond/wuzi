use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;

pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);

pub fn connect(addr: SocketAddr) -> Result<TcpStream, String> {
    match TcpStream::connect(addr) {
        Ok(stream) => {
            //println!("Successfully connected to {}!", stream.peer_addr().unwrap());
            return Ok(stream);
        },
        Err(e) => Err(e.to_string()),
    }
}

pub fn recv(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buf = vec![0; 1024];
    let mut cur = 0;

    loop {
        match stream.read(&mut buf) {
            Ok(0) => { break; }
            Ok(n) => {
                cur += n;
                if cur == buf.len() {
                    buf.resize(buf.len() + 1024, 0);
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => { break }
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => { continue }
            Err(e) => {
                eprintln!("Error[recv]: {}", e.to_string());
                break
            }
        };
    }

    if cur > 0 {
        return Some((&buf[..cur]).to_vec());
    }
    return None;
}
