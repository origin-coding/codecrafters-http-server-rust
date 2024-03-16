use std::str::FromStr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::http::{HttpHeaderName, HttpHeaders, HttpRequest, HttpResponse, HttpResponseStatus, HttpVersion};
use crate::http::HttpHeaderName::{ContentLength, ContentType};

pub struct Server {
    stream: TcpStream,
}

impl Server {
    pub async fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn run(mut self) {
        loop {
            // Accept a new socket.
            let mut buffer = [0; 1024];
            let n = self.stream.read(&mut buffer).await.unwrap();
            let request_string = &*String::from_utf8_lossy(&buffer[..n]);

            // Parse the request and get response.
            if let Ok(response) = HttpRequest::from_str(request_string)
                .and_then(Self::handle_request) {
                // Convert response to string and write all its bytes to TcoStream.
                let response_string = response.to_string();
                self.stream.write_all(response_string.as_bytes()).await.unwrap();
            }
        }
    }

    fn handle_request(http_request: HttpRequest) -> Result<HttpResponse, ()> {
        let mut headers = HttpHeaders::new();

        let status = match http_request.path.as_str() {
            "/" => HttpResponseStatus::Ok,
            path if path.starts_with("/echo/") => HttpResponseStatus::Ok,
            path if path.starts_with("/user-agent") => HttpResponseStatus::Ok,
            _ => HttpResponseStatus::NotFound,
        };

        let content = match http_request.path.as_str() {
            path if path.starts_with("/echo/") => {
                let content = &path["/echo/".len()..];
                headers.insert(ContentType, "text/plain".to_string());
                headers.insert(ContentLength, content.len().to_string());
                content
            }
            path if path.starts_with("/user-agent") => {
                let content = http_request.headers.get(&HttpHeaderName::UserAgent).unwrap();
                headers.insert(ContentType, "text/plain".to_string());
                headers.insert(ContentLength, content.len().to_string());
                content
            }
            _ => ""
        };

        Ok(HttpResponse {
            status,
            version: HttpVersion::V1_1,
            headers,
            body: content.to_string(),
        })
    }
}
