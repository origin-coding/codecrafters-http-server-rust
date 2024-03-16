use std::fmt::{Display, Formatter};

pub enum HttpResponseStatus {
    Ok,
    NotFound,
    // Created,
}

impl Display for HttpResponseStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpResponseStatus::Ok => write!(f, "200 OK"),
            HttpResponseStatus::NotFound => write!(f, "404 Not Found"),
            // HttpResponseStatus::Created => write!(f, "201 Created")
        }
    }
}
