use std::error::Error;
use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiClientErrorCode {
    #[serde(rename = "apiKeyDisabled")]
    ApiKeyDisabled,
    #[serde(rename = "apiKeyExhausted")]
    ApiKeyExhausted,
    #[serde(rename = "apiKeyInvalid")]
    ApiKeyInvalid,
    #[serde(rename = "apiKeyMissing")]
    ApiKeyMissing,
    #[serde(rename = "parameterInvalid")]
    ParameterInvalid,
    #[serde(rename = "parametersMissing")]
    ParametersMissing,
    #[serde(rename = "rateLimited")]
    RateLimited,
    #[serde(rename = "sourcesTooMany")]
    SourcesTooMany,
    #[serde(rename = "sourceDoesNotExist")]
    SourceDoesNotExist,
    #[serde(rename = "unexpectedError")]
    UnexpectedError,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct ApiClientErrorResponse {
    #[allow(dead_code)]
    pub status: String, // required for deserialization, even if unused
    pub code: ApiClientErrorCode,
    pub message: String,
}

#[derive(Debug)]
pub enum ApiClientError {
    Http(reqwest::Error),
    InvalidRequest(String),
    InvalidResponse(ApiClientErrorResponse),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}

impl fmt::Display for ApiClientErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientErrorCode::ApiKeyDisabled => write!(f, "apiKeyDisabled"),
            ApiClientErrorCode::ApiKeyExhausted => write!(f, "apiKeyExhausted"),
            ApiClientErrorCode::ApiKeyInvalid => write!(f, "apiKeyInvalid"),
            ApiClientErrorCode::ApiKeyMissing => write!(f, "apiKeyMissing"),
            ApiClientErrorCode::ParameterInvalid => write!(f, "parameterInvalid"),
            ApiClientErrorCode::ParametersMissing => write!(f, "parametersMissing"),
            ApiClientErrorCode::RateLimited => write!(f, "rateLimited"),
            ApiClientErrorCode::SourcesTooMany => write!(f, "sourcesTooMany"),
            ApiClientErrorCode::SourceDoesNotExist => write!(f, "sourceDoesNotExist"),
            ApiClientErrorCode::UnexpectedError => write!(f, "unexpectedError"),
            ApiClientErrorCode::Unknown => write!(f, "unknown"),
        }
    }
}

impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::Http(err) => write!(f, "HTTP error: {}", err),
            ApiClientError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiClientError::InvalidResponse(response) => {
                // Special handling for ParameterInvalid due to subscription limitation
                if let ApiClientErrorCode::ParameterInvalid = response.code {
                    if response.message.contains("as far back as") && response.message.contains("upgrade to a paid plan") {
                        return write!(
                            f,
                            "NewsAPI subscription limitation: {}. (code: {})",
                            response.message,
                            response.code
                        );
                    }
                }
                write!(f, "Invalid response: {} - {}", response.code, response.message)
            }
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
