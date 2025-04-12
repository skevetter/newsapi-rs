//! # NewsAPI Rust Client
//!
//! A Rust client for the [NewsAPI](https://newsapi.org/) service, providing both async (default)
//! and blocking implementations.
//!
//! ## Features
//!
//! - Async client as the default implementation
//! - Optional blocking client available with the `blocking` feature
//! - Support for all NewsAPI endpoints (everything, top headlines)
//! - Strongly typed request and response models
//! - Builder patterns for easy request construction
//! - Automatic API key detection from environment variables
//! - Configurable retry mechanisms with different strategies
//!
//! ## Usage
//!
//! ### Async Example (Default)
//!
//! ```rust,no_run
//! use newsapi_rs::client::NewsApiClient;
//! use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create client (will check for NEWS_API_KEY environment variable)
//!     let client = NewsApiClient::builder()
//!         .build()
//!         .expect("Failed to build client");
//!
//!     // Build a request
//!     let request = GetTopHeadlinesRequest::builder()
//!         .category(NewsCategory::Business)
//!         .search_term(String::from("Technology"))
//!         .page_size(5)
//!         .build()
//!         .unwrap();
//!
//!     // Make API call
//!     match client.get_top_headlines(&request).await {
//!         Ok(response) => {
//!             println!("Total Results: {}", response.get_total_results());
//!             // Process articles...
//!         },
//!         Err(err) => {
//!             eprintln!("Error: {}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ### Blocking Example (requires 'blocking' feature)
//!
//! ```rust,no_run
//! use newsapi_rs::client::NewsApiClient;
//! use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory};
//!
//! fn main() {
//!     // Create blocking client
//!     let client = NewsApiClient::builder_blocking()
//!         .build()
//!         .expect("Failed to build client");
//!
//!     // Build request
//!     let request = GetTopHeadlinesRequest::builder()
//!         .category(NewsCategory::Business)
//!         .search_term(String::from("Technology"))
//!         .page_size(5)
//!         .build()
//!         .unwrap();
//!
//!     // Make API call
//!     match client.get_top_headlines(&request) {
//!         Ok(response) => {
//!             println!("Total Results: {}", response.get_total_results());
//!             // Process articles...
//!         },
//!         Err(err) => {
//!             eprintln!("Error: {}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ## API Key
//!
//! You can provide your NewsAPI key in several ways:
//!
//! 1. Environment variable: Set the `NEWS_API_KEY` environment variable
//! 2. Explicitly in code: Use the `new()` constructor or the builder's `api_key()` method
//!
//! The builder will automatically check for the environment variable if no key is provided explicitly.
//!
//! ## Retry Strategies
//!
//! The client supports different retry strategies for handling transient errors:
//!
//! ```rust,no_run
//! use newsapi_rs::client::NewsApiClient;
//! use newsapi_rs::retry::RetryStrategy;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Exponential backoff strategy
//!     let client = NewsApiClient::builder()
//!         .retry(RetryStrategy::Exponential(Duration::from_millis(100)), 3)
//!         .build()
//!         .expect("Failed to build client");
//!
//!     // Or constant delay strategy
//!     let client = NewsApiClient::builder()
//!         .retry(RetryStrategy::Constant(Duration::from_secs(1)), 2)
//!         .build()
//!         .expect("Failed to build client");
//!
//!     // Or disable retries
//!     let client = NewsApiClient::builder()
//!         .retry(RetryStrategy::None, 0)
//!         .build()
//!         .expect("Failed to build client");
//! }
//! ```

pub mod client;
pub mod constant;
pub mod error;
pub mod model;
pub mod retry;

pub use client::NewsApiClient;
pub use error::{ApiClientError, ApiClientErrorCode, ApiClientErrorResponse};
pub use model::{
    GetEverythingRequest, GetEverythingResponse, GetSourcesRequest, GetSourcesResponse,
    GetTopHeadlinesRequest, Source, TopHeadlinesResponse,
};
pub use retry::{retry, RetryStrategy};

#[cfg(feature = "blocking")]
pub use retry::retry_blocking;
