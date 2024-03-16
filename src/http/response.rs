use std::fmt::{Display, Formatter};
use crate::http::header::HttpHeaders;
use crate::http::status::HttpResponseStatus;
use crate::http::version::HttpVersion;

pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpResponseStatus,
    pub headers: HttpHeaders,
    // This needs to be Vec<u8> for binary body content.
    // But we will use String for simplicity.
    pub body: String,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\r\n", self.version, self.status)?;
        write!(f, "{}", self.headers)?;
        write!(f, "\r\n{}", self.body)
    }
}
