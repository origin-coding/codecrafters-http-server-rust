mod common;

use std::str::FromStr;
use anyhow::Context;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::common::{handle_request, HttpRequest};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0u8; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        buffer.truncate(len);
        // println!("accepted new connection");
        // stream
        //     .write(b"HTTP/1.1 200 OK\r\n\r\n").await
        //     .with_context(|| format!("writing on {:?}", stream)).unwrap();
        let request = HttpRequest::from_str(&String::from_utf8(buffer).unwrap()).unwrap();
        let response = handle_request(request);
        stream
            .write(response.to_string().as_bytes()).await
            .with_context(|| format!("writing on {:?}", stream)).unwrap();
    }
}
