mod http;
mod server;

// use std::str::FromStr;
// use anyhow::Context;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new().await;

    server.run().await;
}
