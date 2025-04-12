/// This example requires the "blocking" feature to be enabled
/// Run with: cargo run --example top_headlines --features blocking
use env_logger::Env;
use newsapi_rs::{
    client::NewsApiClient,
    constant::DEFAULT_LOG_LEVEL,
    error::ApiClientError,
    model::{GetTopHeadlinesRequest, NewsCategory},
};

#[cfg(feature = "blocking")]
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    dotenvy::dotenv().ok();

    let client = NewsApiClient::from_env();

    let request = GetTopHeadlinesRequest::builder()
        .category(NewsCategory::Business)
        .search_term(String::from("china"))
        .page_size(5)
        .build();

    match client.get_top_headlines(&request.unwrap()) {
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
