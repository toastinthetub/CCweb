mod server;
mod user;

use std::net::SocketAddr;

use crate::server::{HttpMethod, ServerConnection};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream, addr: SocketAddr) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    println!("request from IP: {}", addr);
    match std::str::from_utf8(&buffer) {
        Ok(request_str) => println!("Request: {}", request_str),
        Err(e) => println!("Failed to convert request to string: {}", e),
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body>Hello from CCweb backend (Tokio version)!!</body></html>";
    stream.write_all(response.as_bytes()).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(handle_client(stream, addr));
    }
}
