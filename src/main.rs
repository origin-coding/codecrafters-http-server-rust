mod header;

use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        println!("accepted new connection");
        stream
            .write(b"HTTP/1.1 200 OK\r\n\r\n").await
            .with_context(|| format!("writing on {:?}", stream)).unwrap();
    }
}
