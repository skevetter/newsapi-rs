use newsapi_rs::client::NewsApiClient;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{Country, GetTopHeadlinesRequest, NewsCategory};

/// This example requires the "blocking" feature to be enabled
/// Run with: cargo run --example top_headlines --features blocking
#[cfg(feature = "blocking")]
fn main() {
    dotenvy::dotenv().ok();

    // Provide your API key here or set it in the environment variable NEWS_API_KEY
    // let client = NewsApiClient::new("api_key");
    let client = NewsApiClient::from_env_blocking();

    let request = GetTopHeadlinesRequest::builder()
        .country(Country::US)
        .category(NewsCategory::Technology)
        .page_size(5)
        .build()
        .unwrap();

    match client.get_top_headlines(&request) {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            println!("Articles retrieved: {}", response.get_articles().len());

            for (i, article) in response.get_articles().iter().enumerate() {
                println!("Article #{}: {}", i + 1, article.get_title());
                println!("  Source: {}", article.get_source().get_name());
                println!("  URL: {}", article.get_url());
                println!();
            }
        }
        Err(err) => {
            eprintln!(
                "Error: {}",
                match err {
                    ApiClientError::InvalidResponse(response) => response.message.clone(),
                    _ => err.to_string(),
                }
            );
        }
    }
}

/// Default async version that runs when the 'blocking' feature is not enabled
/// Run with: cargo run --example top_headlines
#[cfg(not(feature = "blocking"))]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Provide your API key here or set it in the environment variable NEWS_API_KEY
    // let client = NewsApiClient::new("api_key");
    let client = NewsApiClient::from_env();

    let request = GetTopHeadlinesRequest::builder()
        .country(Country::US)
        .category(NewsCategory::Technology)
        .page_size(5)
        .build()
        .unwrap();

    match client.get_top_headlines(&request).await {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            println!("Articles retrieved: {}", response.get_articles().len());

            for (i, article) in response.get_articles().iter().enumerate() {
                println!("Article #{}: {}", i + 1, article.get_title());
                println!("  Source: {}", article.get_source().get_name());
                println!("  URL: {}", article.get_url());
                println!();
            }
        }
        Err(err) => {
            eprintln!(
                "Error: {}",
                match err {
                    ApiClientError::InvalidResponse(response) => response.message.clone(),
                    _ => err.to_string(),
                }
            );
        }
    }
}
