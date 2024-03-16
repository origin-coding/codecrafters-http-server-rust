use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Hash)]
pub enum HttpHeaderName {
    Authorization,
    ContentType,
    // 其他的请求头名称
    Custom(String),
}

impl HttpHeaderName {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Authorization" => HttpHeaderName::Authorization,
            "Content-Type" => HttpHeaderName::ContentType,
            // 其他预定义的请求头名称...
            _ => HttpHeaderName::Custom(s.to_string()),
        }
    }
}

impl Display for HttpHeaderName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeaderName::Authorization => write!(f, "Authorization"),
            HttpHeaderName::ContentType => write!(f, "Content-Type"),
            HttpHeaderName::Custom(s) => write!(f, "{}", s),
        }
    }
}

pub struct HttpHeaders {
    headers: HashMap<HttpHeaderName, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        HttpHeaders { headers: HashMap::new() }
    }

    pub fn get(&self, key: &HttpHeaderName) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn insert(&mut self, key: HttpHeaderName, value: String) {
        self.headers.insert(key, value);
    }
}

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        Ok(())
    }
}
