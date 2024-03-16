use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::http::{HttpHeaders, HttpRequest, HttpResponse, HttpResponseStatus, HttpVersion};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
        Self { listener }
    }

    pub async fn run(&mut self) {
        loop {
            // Accept a new socket.
            let (mut socket, _) = self.listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            let request_string = &*String::from_utf8_lossy(&buffer[..n]);

            // Parse the request and get response.
            if let Ok(response) = HttpRequest::from_str(request_string)
                .and_then(Self::handle_request) {
                // Convert response to string and write all its bytes to TcoStream.
                let response_string = response.to_string();
                socket.write_all(response_string.as_bytes()).await.unwrap();
            }
        }
    }

    fn handle_request(http_request: HttpRequest) -> Result<HttpResponse, ()> {
        let status = match http_request.path.as_str() {
            "/" => HttpResponseStatus::Ok,
            path if path.starts_with("/echo/") => HttpResponseStatus::Ok,
            _ => HttpResponseStatus::NotFound,
        };

        let content = match http_request.path.as_str() {
            path if path.starts_with("/echo/") => {
                let content = &path["/echo/".len()..];
                content
            }
            _ => ""
        };

        Ok(HttpResponse {
            status,
            version: HttpVersion::V1_1,
            headers: HttpHeaders::new(),
            body: content.to_string(),
        })
    }
}
