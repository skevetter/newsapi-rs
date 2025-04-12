//! # NewsAPI Rust Client
//!
//! A Rust client for the NewsAPI service.

pub mod client;
pub mod constant;
pub mod error;
pub mod model;
pub mod retry;

pub use client::NewsApiClient;
pub use error::{ApiClientError, ApiClientErrorCode, ApiClientErrorResponse};
pub use model::{
    GetEverythingRequest, GetEverythingResponse, GetTopHeadlinesRequest, TopHeadlinesResponse,
};
pub use retry::{retry, RetryStrategy};

#[cfg(feature = "blocking")]
pub use retry::retry_blocking;
