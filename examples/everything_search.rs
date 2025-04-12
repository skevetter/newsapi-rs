/// This example requires the "blocking" feature to be enabled
/// Run with: cargo run --example everything_search --features blocking
use chrono::Utc;
use env_logger::Env;
use newsapi_rs::{
    client::NewsApiClient,
    constant::DEFAULT_LOG_LEVEL,
    error::ApiClientError,
    model::{GetEverythingRequest, Language},
};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    dotenvy::dotenv().ok();

    // Provide your API key here or set it in the environment variable NEWSAPI_API_KEY
    // let client = NewsApiClient::new("api_key");
    let client = NewsApiClient::from_env();

    let everything_request = GetEverythingRequest::builder()
        .search_term(String::from("Nvidia+NVDA+stock"))
        .language(Language::EN)
        .start_date(Utc::now() - chrono::Duration::days(30))
        .end_date(Utc::now())
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
