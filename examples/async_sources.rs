use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetSourcesRequest, NewsCategory};
use newsapi_rs::retry::RetryStrategy;

/// Run with: cargo run --example async_sources
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = NewsApiClient::builder()
        .retry(RetryStrategy::None, 0)
        .build()
        .expect("Failed to build NewsApiClient");

    let sources_request = GetSourcesRequest::builder()
        .category(NewsCategory::Technology)
        .build();

    match client.get_sources(&sources_request).await {
        Ok(response) => {
            println!("Sources found: {}", response.get_sources().len());
            println!("Status: {}", response.get_status());

            for (i, source) in response.get_sources().iter().enumerate() {
                println!("Source #{}: {}", i + 1, source.get_name());
                if let Some(desc) = source.get_description() {
                    println!("  Description: {}", desc);
                }
                if let Some(url) = source.get_url() {
                    println!("  URL: {}", url);
                }
                println!();
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
