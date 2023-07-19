use std::io::{Read, Write};
use std::net::TcpListener;

pub fn start_server() {
  let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
  for stream in listener.incoming() {
    println!("New client!");
    let mut tcp_stream: std::net::TcpStream = stream.unwrap();
    let mut buffer: [u8; 1024] = [0; 1024];
    let _size: usize = tcp_stream.read(&mut buffer).unwrap();
    tcp_stream.write_all(b"Hello, client").unwrap();
    tcp_stream.flush().unwrap();
  }
}
