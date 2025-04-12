/// Run with: cargo run --example async_everything_search
use chrono::Utc;
use env_logger::Env;
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::constant::DEFAULT_LOG_LEVEL;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{GetEverythingRequest, Language};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    dotenvy::dotenv().ok();

    // Provide your API key here or set it in the environment variable NEWSAPI_API_KEY
    // let client = NewsApiClient::new_async("api_key");
    let client = NewsApiClient::from_env_async();

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
