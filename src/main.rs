use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    loop {
        let (_, _) = listener.accept().await.unwrap();
        println!("accepted new connection");
    }
}
