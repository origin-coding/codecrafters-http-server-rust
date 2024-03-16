mod http;
mod server;

use std::env::args;
use tokio::net::TcpListener;
use crate::server::Server;

#[tokio::main]
async fn main() {
    let args = args().collect::<Vec<String>>();

    let mut directory: Option<String> = None;
    if let Some(arg) = args.get(1) {
        if arg == "--directory" {
            directory = args.get(2).cloned();
        }
    }

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let server = Server::new(stream).await;
        let future = server.run(directory.clone().unwrap_or("".to_string()));
        tokio::spawn(future);
    }
}
