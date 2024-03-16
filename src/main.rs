mod http;
mod server;

use tokio::net::TcpListener;
use crate::server::Server;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let server = Server::new(stream).await;
        let future = server.run();
        tokio::spawn(future);
    }
}
