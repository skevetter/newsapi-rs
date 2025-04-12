//! # NewsAPI Rust Client
//!
//! A Rust client for the NewsAPI service.

pub mod client;
pub mod constant;
pub mod error;
pub mod model;

// Re-export commonly used items for convenience
pub use client::NewsApiClient;
pub use error::{ApiClientError, ApiClientErrorCode, ApiClientErrorResponse};
pub use model::{
    GetEverythingRequest, GetEverythingResponse, GetTopHeadlinesRequest, TopHeadlinesResponse,
};
