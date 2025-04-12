/// Run with: cargo run --example async_everything_search
use chrono::Utc;
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{GetEverythingRequest, Language};
use newsapi_rs::retry::RetryStrategy;
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Provide your API key here or set it in the environment variable NEWSAPI_API_KEY
    // Create client with retry support - will retry up to 3 times with exponential backoff
    let client = NewsApiClient::from_env_async()
        .with_retry(RetryStrategy::Constant(Duration::from_secs(1)), 3);

    // Or without retry (default):
    // let client = NewsApiClient::from_env_async();

    let everything_request = GetEverythingRequest::builder()
        .search_term(String::from("Nvidia+NVDA+stock"))
        .language(Language::EN)
        .start_date(Utc::now() - chrono::Duration::days(30))
        .end_date(Utc::now())
        .page_size(1)
        .build();

    match client.get_everything(&everything_request).await {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            println!("Articles retrieved: {}", response.get_articles().len());

            for (i, article) in response.get_articles().iter().enumerate() {
                println!("Article #{}: {}", i + 1, article.get_title());
                println!("  Source: {}", article.get_source().get_name());
                println!("  Published: {}", article.get_published_at());
                println!("  URL: {}", article.get_url());
                println!();
            }
        }
        Err(err) => {
            eprintln!(
                "Error [{}]: {}",
                match &err {
                    ApiClientError::InvalidResponse(response) => response.code.to_string(),
                    _ => err.to_string(),
                },
                match &err {
                    ApiClientError::InvalidResponse(response) => response.message.to_string(),
                    _ => err.to_string(),
                }
            );
        }
    }
}
