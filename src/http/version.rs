use std::fmt::{Display, Formatter};

pub enum HttpVersion {
    V1_0,
    V1_1,
}

impl HttpVersion {
    pub fn from_str(s: &str) -> Option<HttpVersion> {
        match s {
            "HTTP/1.0" => Some(HttpVersion::V1_0),
            "HTTP/1.1" => Some(HttpVersion::V1_1),
            _ => None,
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::V1_0 => write!(f, "HTTP/1.0"),
            HttpVersion::V1_1 => write!(f, "HTTP/1.1"),
        }
    }
}
