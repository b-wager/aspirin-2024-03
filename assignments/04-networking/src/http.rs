use std::{fmt::Display, str::FromStr};

use crate::error::AspirinEatsError;

/// Simple wrapper for an HTTP Request
#[derive(Debug)]
pub struct HttpRequest {
    /// The HTTP method used in the request (GET, POST, etc)
    pub method: Option<String>,

    /// The path requested by the client
    pub path: Option<String>,

    /// The body of the request
    pub body: Option<String>,
}

impl FromStr for HttpRequest {
    type Err = AspirinEatsError;

    // Parse a string into an HTTP Request
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, "\r\n\r\n");
        let headers = parts.next().ok_or("Invalid request")?;
        let body = parts.next().map(|s| s.to_string());

        let request_line = headers.lines().next().ok_or("Invalid request")?;
        let mut parts = request_line.split_whitespace();

        let method = parts.next().map(|s| s.to_string());
        let path = parts.next().map(|s| s.to_string());

        Ok(HttpRequest { method, path, body })
    }
}

pub struct HttpResponse {
    status_code: u16,
    status_text: String,
    body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str, body: &str) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            body: body.to_string(),
        }
    }
}

impl Display for HttpResponse {
    /// Convert an HttpResponse struct to a valid HTTP Response
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code, self.status_text, self.body
        )
    }
}

impl From<AspirinEatsError> for HttpResponse {
    /// Given an error type, convert it to an appropriate HTTP Response
    fn from(value: AspirinEatsError) -> Self {
        match value {
            AspirinEatsError::ParseError(_) => {
                HttpResponse::new(400, "Bad Request", "Failed to parse request\n")
            }
            AspirinEatsError::Database(_) => HttpResponse::new(
                503,
                "Service Unavailable",
                "Failed to interact with database\n",
            ),
            AspirinEatsError::Io(_) => {
                HttpResponse::new(500, "Internal Server Error", "Internal Server Error\n")
            }
            AspirinEatsError::InvalidRequest => {
                HttpResponse::new(400, "Bad Request", "Invalid Request\n")
            }
            AspirinEatsError::NotFound => {
                HttpResponse::new(404, "Not Found", "Resource not found\n")
            }
            AspirinEatsError::MethodNotAllowed => {
                HttpResponse::new(405, "Method Not Allowed", "Method not allowed\n")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_from_str() {
        let request = "GET /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\nthis is the body.";
        let http_request = HttpRequest::from_str(request).unwrap();
        assert_eq!(http_request.method, Some("GET".to_string()));
        assert_eq!(http_request.path, Some("/orders".to_string()));
        assert_eq!(http_request.body, Some("this is the body.".to_string()));
    }

    #[test]
    fn test_http_response_to_string() {
        let response = HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!");
        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\n\r\nWelcome to Aspirin Eats!"
        );
    }

    #[test]
    fn test_http_response_from_aspirin_eats_error() {
        let error = AspirinEatsError::InvalidRequest;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 400);
        assert_eq!(response.status_text, "Bad Request");
        assert_eq!(response.body, "Invalid Request\n");

        let error = AspirinEatsError::NotFound;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 404);
        assert_eq!(response.status_text, "Not Found");
        assert_eq!(response.body, "Resource not found\n");

        let error = AspirinEatsError::MethodNotAllowed;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 405);
        assert_eq!(response.status_text, "Method Not Allowed");
        assert_eq!(response.body, "Method not allowed\n");

        let error = AspirinEatsError::Io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 500);
        assert_eq!(response.status_text, "Internal Server Error");
        assert_eq!(response.body, "Internal Server Error\n");
    }
}
