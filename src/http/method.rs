use std::fmt::Display;

pub enum HttpRequestMethod {
    Get,
    Post,
}

impl HttpRequestMethod {
    pub fn from_str(s: &str) -> Option<HttpRequestMethod> {
        match s {
            "GET" => Some(HttpRequestMethod::Get),
            "POST" => Some(HttpRequestMethod::Post),
            _ => None,
        }
    }
}

impl Display for HttpRequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpRequestMethod::Get => write!(f, "GET"),
            HttpRequestMethod::Post => write!(f, "POST"),
        }
    }
}
