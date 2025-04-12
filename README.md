# NewsAPI Rust Client

A Rust client for the [NewsAPI](https://newsapi.org/) service.

## Features

- Async client (default)
- Optional blocking client
- Support for all NewsAPI endpoints
- Strongly typed request/response models
- Builder pattern with automatic environment variable detection
- Retry mechanisms with configurable strategies

## Installation

```toml
[dependencies]
newsapi-rs = "0.1.0"

# If you need the blocking client
newsapi-rs = { version = "0.1.0", features = ["blocking"] }
```

## Client Creation

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::retry::RetryStrategy;
use std::time::Duration;

// Async client (default)
// Method 1: From environment variable (requires NEWS_API_KEY to be set)
let client = NewsApiClient::from_env();

// Method 2: With explicit API key
let client = NewsApiClient::new("your-api-key");

// Method 3: Using the builder pattern
let client = NewsApiClient::builder()
    .retry(RetryStrategy::Exponential(Duration::from_millis(100)), 3)
    .build()
    .expect("Failed to build NewsApiClient");

// Blocking client (with 'blocking' feature)
// Method 1: From environment variable
let blocking_client = NewsApiClient::from_env_blocking();

// Method 2: With explicit API key
let blocking_client = NewsApiClient::new_blocking("your-api-key");

// Method 3: Using the builder pattern
let blocking_client = NewsApiClient::builder_blocking()
    .retry(RetryStrategy::Constant(Duration::from_secs(1)), 2)
    .build()
    .expect("Failed to build NewsApiClient");
```

## Endpoints

### 1. Top Headlines

Get breaking news headlines from various news sources.

#### Async Example

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory, Country};

#[tokio::main]
async fn main() {
    let client = NewsApiClient::builder().build().expect("Failed to build client");

    let request = GetTopHeadlinesRequest::builder()
        .country(Country::US)
        .category(NewsCategory::Business)
        .search_term(String::from("Technology"))
        .page_size(5)
        .build()
        .unwrap();

    match client.get_top_headlines(&request).await {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            for article in response.get_articles() {
                println!("Title: {}", article.get_title());
                println!("  Source: {}", article.get_source().get_name());
                println!("  URL: {}", article.get_url());
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

### 2. Everything

Search through millions of articles from various news sources.

#### Async Example

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetEverythingRequest, Language};
use chrono::Utc;

#[tokio::main]
async fn main() {
    let client = NewsApiClient::builder().build().expect("Failed to build client");

    let request = GetEverythingRequest::builder()
        .search_term(String::from("Bitcoin"))
        .language(Language::EN)
        .start_date(Utc::now() - chrono::Duration::days(7))
        .end_date(Utc::now())
        .page_size(10)
        .build();

    match client.get_everything(&request).await {
        Ok(response) => {
            println!("Found {} articles", response.get_total_results());
            for article in response.get_articles() {
                println!("- {}", article.get_title());
                println!("  {}", article.get_url());
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

### 3. Sources

Get information about news publishers available in the system.

#### Async Example

```rust
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetSourcesRequest, NewsCategory, Language, Country};

#[tokio::main]
async fn main() {
    let client = NewsApiClient::builder().build().expect("Failed to build client");

    let request = GetSourcesRequest::builder()
        .category(NewsCategory::Technology)
        .language(Language::EN)
        .country(Country::US)
        .build();

    match client.get_sources(&request).await {
        Ok(response) => {
            println!("Found {} sources", response.get_sources().len());
            for source in response.get_sources() {
                println!("- {}", source.get_name());
                if let Some(desc) = source.get_description() {
                    println!("  Description: {}", desc);
                }
                if let Some(url) = source.get_url() {
                    println!("  URL: {}", url);
                }
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## Blocking Examples

With the `blocking` feature enabled, you can use the client without async/await:

```rust
// Top Headlines (blocking)
let client = NewsApiClient::builder_blocking().build().expect("Failed to build client");
let request = GetTopHeadlinesRequest::builder()
    .country(Country::US)
    .build()
    .unwrap();

match client.get_top_headlines(&request) {
    Ok(response) => println!("Found {} articles", response.get_total_results()),
    Err(err) => eprintln!("Error: {}", err),
}

// Sources (blocking)
let sources_request = GetSourcesRequest::builder()
    .category(NewsCategory::Technology)
    .build();

match client.get_sources(&sources_request) {
    Ok(response) => println!("Found {} sources", response.get_sources().len()),
    Err(err) => eprintln!("Error: {}", err),
}
```

## Running Examples

```bash
# Async examples
cargo run --example async_everything_search
cargo run --example async_sources

# Blocking examples (with the blocking feature enabled)
cargo run --example everything_search --features blocking
cargo run --example top_headlines --features blocking
```

## License

MIT
