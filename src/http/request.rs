use std::str::FromStr;
use crate::http::{HttpHeaderName, HttpHeaders, HttpRequestMethod, HttpVersion};

pub struct HttpRequest {
    pub method: HttpRequestMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HttpHeaders,
    // This needs to be Vec<u8> for binary body content.
    // But we will use String for simplicity.
    pub body: String,
}

impl FromStr for HttpRequest {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut lines = value.lines();
        let request_line = lines.next().unwrap();
        let mut parts = request_line.split_whitespace();

        // Parse the first line.
        let method = parts.next().and_then(HttpRequestMethod::from_str).ok_or(())?;
        let path = parts.next().unwrap().to_string();
        let version = parts.next().and_then(HttpVersion::from_str).unwrap_or(HttpVersion::V1_1);

        // Parse headers.
        let mut headers = HttpHeaders::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.splitn(2, ": ").collect::<Vec<&str>>();
            // Maybe need to add error handling.
            if parts.len() != 2 { return Err(()); }

            let key = HttpHeaderName::from_str(parts[0]);
            let value = parts[1].to_string();
            headers.insert(key, value);
        }
        let body = lines.collect::<Vec<&str>>().join("\r\n");
        Ok(Self { method, path, version, headers, body })
    }
}
