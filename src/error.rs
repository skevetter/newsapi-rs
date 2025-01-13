use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApiClientError {
    Http(reqwest::Error),
    InvalidRequest(String),
    InvalidResponse(String),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}

impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::Http(err) => write!(f, "HTTP error: {}", err),
            ApiClientError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiClientError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            ApiClientError::InvalidHeaderValue(err) => write!(f, "Invalid header value: {}", err),
        }
    }
}

impl Error for ApiClientError {}

impl From<reqwest::Error> for ApiClientError {
    fn from(err: reqwest::Error) -> ApiClientError {
        ApiClientError::Http(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for ApiClientError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> ApiClientError {
        ApiClientError::InvalidHeaderValue(err)
    }
}
