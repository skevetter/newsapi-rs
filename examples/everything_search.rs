use chrono::{TimeZone, Utc};
use env_logger::Env;
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::constant::DEFAULT_LOG_LEVEL;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{GetEverythingRequest, Language};

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Create client from environment variable
    let client = NewsApiClient::from_env();

    // Build "everything" request
    let everything_request = GetEverythingRequest::builder()
        .search_term(String::from("Nvidia+NVDA+stock"))
        .language(Language::EN)
        .start_date(Utc.with_ymd_and_hms(2025, 3, 14, 0, 0, 0).unwrap())
        .end_date(Utc.with_ymd_and_hms(2025, 3, 20, 0, 0, 0).unwrap())
        .page_size(1)
        .build();

    match client.get_everything(&everything_request) {
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
                "Error: {}",
                match err {
                    ApiClientError::InvalidResponse(response) => response.message.clone(),
                    _ => err.to_string(),
                }
            );
        }
    }
}
