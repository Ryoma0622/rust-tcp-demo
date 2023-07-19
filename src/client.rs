use std::io::{Read, Write};
use std::net::TcpStream;

pub fn start_client() {
  let mut tcp_stream: TcpStream = TcpStream::connect("127.0.0.1:9001").unwrap();
  tcp_stream.write_all(b"Hello server").unwrap();
  tcp_stream.flush().unwrap();
  let mut buffer: [u8; 1024] = [0; 1024];
  let size: usize = tcp_stream.read(&mut buffer).unwrap();
  let message = String::from_utf8_lossy(&buffer[..size]);
  println!("Server says: {}", message);
}
