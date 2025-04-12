//! # NewsAPI Rust Client
//!
//! A Rust client for the [NewsAPI](https://newsapi.org/) service, providing both async (default)
//! and blocking implementations.
//!
//! ## Features
//!
//! - Async client as the default implementation
//! - Optional blocking client available with the `blocking` feature
//! - Support for all NewsAPI endpoints (top headlines, everything, sources)
//! - Strongly typed request and response models
//! - Builder patterns for easy request construction
//! - Automatic API key detection from environment variables
//! - Configurable retry mechanisms with different strategies
//!
//! ## Endpoints
//!
//! This library supports all three endpoints provided by NewsAPI:
//!
//! ### 1. Top Headlines
//!
//! ```rust,no_run
//! use newsapi_rs::client::NewsApiClient;
//! use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory, Country};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = NewsApiClient::builder()
//!         .build()
//!         .expect("Failed to build client");
//!
//!     let request = GetTopHeadlinesRequest::builder()
//!         .country(Country::US)
//!         .category(NewsCategory::Business)
//!         .page_size(5)
//!         .build()
//!         .unwrap();
//!
//!     let response = client.get_top_headlines(&request).await.unwrap();
//!     println!("Found {} articles", response.get_total_results());
//! }
//! ```
//!
//! ### 2. Everything
//!
//! ```rust,no_run
//! use newsapi_rs::client::NewsApiClient;
//! use newsapi_rs::model::{GetEverythingRequest, Language};
//! use chrono::Utc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = NewsApiClient::builder()
//!         .build()
//!         .expect("Failed to build client");
//!
//!     let request = GetEverythingRequest::builder()
//!         .search_term("Bitcoin".to_string())
//!         .language(Language::EN)
//!         .start_date(Utc::now() - chrono::Duration::days(7))
//!         .build();
//!
//!     let response = client.get_everything(&request).await.unwrap();
//!     println!("Found {} articles", response.get_total_results());
//! }
//! ```
//!
//! ### 3. Sources
//!
//! ```rust,no_run
//! use newsapi_rs::client::NewsApiClient;
//! use newsapi_rs::model::{GetSourcesRequest, NewsCategory, Language};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = NewsApiClient::builder()
//!         .build()
//!         .expect("Failed to build client");
//!
//!     let request = GetSourcesRequest::builder()
//!         .category(NewsCategory::Technology)
//!         .language(Language::EN)
//!         .build();
//!
//!     let response = client.get_sources(&request).await.unwrap();
//!     println!("Found {} sources", response.get_sources().len());
//!
//!     // Print out the first source info
//!     if !response.get_sources().is_empty() {
//!         let source = &response.get_sources()[0];
//!         println!("Name: {}", source.get_name());
//!         if let Some(desc) = source.get_description() {
//!             println!("Description: {}", desc);
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
