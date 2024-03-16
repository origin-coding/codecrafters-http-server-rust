use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum HttpMethod {
    Get,
    Post,
    Delete,
    Put,
}

/// HTTP response status code.
pub enum HttpStatus {
    Ok,
    NotFound,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
        }
    }
}

pub enum HttpVersion {
    V1_1,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::V1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl FromStr for HttpRequest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let request_line = lines.next().unwrap();
        let mut parts = request_line.split_whitespace();
        let method = match parts.next() {
            Some("GET") => HttpMethod::Get,
            Some("POST") => HttpMethod::Post,
            Some("DELETE") => HttpMethod::Delete,
            Some("PUT") => HttpMethod::Put,
            _ => return Err(()),
        };
        let path = parts.next().unwrap().to_string();
        let version = match parts.next() {
            Some("HTTP/1.1") => HttpVersion::V1_1,
            _ => return Err(()),
        };
        let mut headers = HashMap::new();
        for line in lines.clone() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            headers.insert(key, value);
        }
        let body = lines.collect::<Vec<&str>>().join("\r\n");
        Ok(HttpRequest { method, path, version, headers, body })
    }
}

pub struct HttpResponse {
    pub status: HttpStatus,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\r\n", self.version, self.status)?;
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}

pub fn handle_request(http_request: HttpRequest) -> HttpResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    let status = match http_request.path.as_str() {
        "/" => HttpStatus::Ok,
        _ => HttpStatus::NotFound,
    };
    HttpResponse { status, version: HttpVersion::V1_1, headers, body: "".to_string() }
}
