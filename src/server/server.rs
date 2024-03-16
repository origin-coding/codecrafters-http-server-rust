use std::path::Path;
use std::str::FromStr;

use tokio::fs::{metadata, read, write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::http::{HttpHeaderName, HttpHeaders, HttpRequest, HttpRequestMethod, HttpResponse, HttpResponseStatus, HttpVersion};
use crate::http::HttpHeaderName::{ContentLength, ContentType};

pub struct Server {
    stream: TcpStream,
}

impl Server {
    pub async fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn run(mut self, directory: String) {
        loop {
            // Accept a new socket.
            let mut buffer = [0; 1024];
            let n = self.stream.read(&mut buffer).await.unwrap();
            let request_string = &*String::from_utf8_lossy(&buffer[..n]);

            // Parse the request and get response.
            let request = match HttpRequest::from_str(request_string) {
                Ok(request) => request,
                Err(_) => continue
            };
            let response = match Self::handle_request(request, directory.clone().as_str()).await {
                Ok(response) => response,
                Err(_) => continue
            };

            let response_string = response.to_string();
            self.stream.write_all(response_string.as_bytes()).await.unwrap();
        }
    }

    async fn handle_request(http_request: HttpRequest, directory: &str) -> Result<HttpResponse, ()> {
        let mut headers = HttpHeaders::new();

        // Fuck, this is too ugly!

        let status = match http_request.path.as_str() {
            "/" => HttpResponseStatus::Ok,
            path if path.starts_with("/echo/") => HttpResponseStatus::Ok,
            path if path.starts_with("/user-agent") => HttpResponseStatus::Ok,
            path if path.starts_with("/files/") && http_request.method == HttpRequestMethod::Get => {
                let directory = Path::new(&directory).join(&path[7..]);
                if tokio::fs::metadata(directory).await.is_ok() {
                    HttpResponseStatus::Ok
                } else {
                    HttpResponseStatus::NotFound
                }
            }
            path if path.starts_with("/files/") && http_request.method == HttpRequestMethod::Post => {
                HttpResponseStatus::Created
            }
            _ => HttpResponseStatus::NotFound,
        };

        let content = match http_request.path.as_str() {
            path if path.starts_with("/echo/") => {
                let content = &path["/echo/".len()..];
                headers.insert(ContentType, "text/plain".to_string());
                headers.insert(ContentLength, content.len().to_string());
                content.as_bytes().to_vec()
            }
            path if path.starts_with("/user-agent") => {
                let content = http_request.headers.get(&HttpHeaderName::UserAgent).unwrap();
                headers.insert(ContentType, "text/plain".to_string());
                headers.insert(ContentLength, content.len().to_string());
                content.as_bytes().to_owned()
            }
            path if path.starts_with("/files/") && http_request.method == HttpRequestMethod::Get => {
                let path = Path::new(directory).join(&path[7..]);
                let path_str = path.to_str().unwrap();
                let file_content = match metadata(path_str).await {
                    Ok(_) => read(path_str).await.unwrap(),
                    Err(_) => "".as_bytes().to_vec(),
                };
                headers.insert(ContentType, "application/octet-stream".to_string());
                headers.insert(ContentLength, file_content.len().to_string());
                file_content
            }
            path if path.starts_with("/files/") && http_request.method == HttpRequestMethod::Post => {
                let path = Path::new(directory).join(&path[7..]);
                let path_str = path.to_str().unwrap();
                let file_content = http_request.body;
                write(path_str, file_content).await.unwrap();
                headers.insert(ContentType, "text/plain".to_string());
                headers.insert(ContentLength, "".len().to_string());
                "".as_bytes().to_vec()
            }
            _ => "".as_bytes().to_vec()
        };

        Ok(HttpResponse {
            status,
            version: HttpVersion::V1_1,
            headers,
            body: content,
        })
    }
}
