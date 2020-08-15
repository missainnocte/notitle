use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::Command;

pub fn run_server() {
    let _listener = TcpListener::bind("127.0.0.1:3154");
    let listener = match _listener {
        Ok(_l) => _l,
        Err(e) => panic!(e),
    };
    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => panic!(e),
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let peer_addr = stream.peer_addr().unwrap();
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _child = child.wait().unwrap();
    println!(
        "remote:{}:{}, content:{}",
        peer_addr.ip(),
        peer_addr.port(),
        String::from_utf8_lossy(&buffer[..])
    );
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
