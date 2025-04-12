# NewsAPI Rust Client

A Rust client for the [NewsAPI](https://newsapi.org/) service.

## Features

- Async client (default)
- Optional blocking client
- Support for all NewsAPI endpoints
- Strongly typed request/response models
- Builder pattern with automatic environment variable detection
- Retry mechanisms with configurable strategies

## Usage

### Add to your project

```toml
[dependencies]
newsapi-rs = "0.1.0"

# If you need the blocking client
newsapi-rs = { version = "0.1.0", features = ["blocking"] }
```

### Async Example (Default)

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory};

#[tokio::main]
async fn main() {
    // Create client using various methods:

    // 1. From environment variable (requires NEWS_API_KEY to be set)
    let client = NewsApiClient::from_env();

    // 2. With explicit API key
    let client = NewsApiClient::new("your-api-key");

    // 3. Using the builder pattern (checks env vars automatically if no key provided)
    let client = NewsApiClient::builder()
        .retry(RetryStrategy::Exponential(Duration::from_millis(100)), 3)
        .build()
        .expect("Failed to build NewsApiClient");

    // Build request
    let request = GetTopHeadlinesRequest::builder()
        .category(NewsCategory::Business)
        .search_term(String::from("Technology"))
        .page_size(5)
        .build()
        .unwrap();

    // Make API call
    match client.get_top_headlines(&request).await {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            for article in response.get_articles() {
                println!("Title: {}", article.get_title());
                // Process articles...
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
```

### Blocking Example (requires 'blocking' feature)

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory};
use newsapi_rs::retry::RetryStrategy;
use std::time::Duration;

fn main() {
    // Create blocking client using various methods:

    // 1. From environment variable
    let client = NewsApiClient::from_env_blocking();

    // 2. With explicit API key
    let client = NewsApiClient::new_blocking("your-api-key");

    // 3. Using the builder pattern
    let client = NewsApiClient::builder_blocking()
        .retry(RetryStrategy::Constant(Duration::from_secs(1)), 2)
        .build()
        .expect("Failed to build NewsApiClient");

    // Build request
    let request = GetTopHeadlinesRequest::builder()
        .category(NewsCategory::Business)
        .search_term(String::from("Technology"))
        .page_size(5)
        .build()
        .unwrap();

    // Make API call
    match client.get_top_headlines(&request) {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            for article in response.get_articles() {
                println!("Title: {}", article.get_title());
                // Process articles...
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
```

## Everything API Example

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetEverythingRequest, Language};
use chrono::Utc;

#[tokio::main]
async fn main() {
    // Create client (will check environment variable NEWS_API_KEY if no key provided)
    let client = NewsApiClient::builder()
        .build()
        .expect("Failed to build NewsApiClient");

    let everything_request = GetEverythingRequest::builder()
        .search_term(String::from("Bitcoin"))
        .language(Language::EN)
        .start_date(Utc::now() - chrono::Duration::days(7))
        .end_date(Utc::now())
        .page_size(10)
        .build();

    match client.get_everything(&everything_request).await {
        Ok(response) => {
            println!("Found {} articles", response.get_total_results());
            for article in response.get_articles() {
                println!("- {}", article.get_title());
                println!("  {}", article.get_url());
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
```

## Running Examples

Run the async examples:
```bash
cargo run --example top_headlines
cargo run --example everything_search
cargo run --example async_everything_search
```

Run the blocking example (with the blocking feature enabled):
```bash
cargo run --example blocking_example --features blocking
```

## License

MIT
