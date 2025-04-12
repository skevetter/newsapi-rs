use newsapi_rs::client::NewsApiClient;
use newsapi_rs::model::{GetEverythingRequest, Language};

/// Run with: cargo run --example async_everything_search
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("Example 1: Using the builder pattern");
    let builder_client = NewsApiClient::builder()
        .build()
        .expect("Failed to build NewsApiClient");

    let request1 = GetEverythingRequest::builder()
        .search_term(String::from("Nvidia+NVDA+stock"))
        .language(Language::EN)
        .page_size(1)
        .build();

    match builder_client.get_everything(&request1).await {
        Ok(response) => {
            println!(
                "Builder client - Total Results: {}",
                response.get_total_results()
            );
            println!("Articles retrieved: {}", response.get_articles().len());
            if let Some(article) = response.get_articles().first() {
                println!("First article: {}", article.get_title());
            }
        }
        Err(err) => {
            eprintln!("Builder client error: {}", err);
        }
    }

    println!("\nExample 2: Using from_env");
    let env_client = NewsApiClient::from_env();

    let request2 = GetEverythingRequest::builder()
        .search_term(String::from("Bitcoin+crypto"))
        .language(Language::EN)
        .page_size(1)
        .build();

    match env_client.get_everything(&request2).await {
        Ok(response) => {
            println!(
                "Env client - Total Results: {}",
                response.get_total_results()
            );
            println!("Articles retrieved: {}", response.get_articles().len());
            if let Some(article) = response.get_articles().first() {
                println!("First article: {}", article.get_title());
            }
        }
        Err(err) => {
            eprintln!("Env client error: {}", err);
        }
    }
}
