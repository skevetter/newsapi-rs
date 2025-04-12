use chrono::{TimeZone, Utc};
use env_logger::Env;
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::constant::DEFAULT_LOG_LEVEL;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{GetEverythingRequest, GetTopHeadlinesRequest, Language, NewsCategory};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();
    dotenvy::dotenv().ok();
    let client = NewsApiClient::from_env();
    let request = GetTopHeadlinesRequest::builder()
        // .country(Country::US)
        .category(NewsCategory::Business)
        .search_term(String::from("Nvidia Stock"))
        .page_size(5)
        // .page(3)
        .build();
    let response = client.clone().get_top_headlines(&request.unwrap()).unwrap();
    log::info!("{:?}", response);

    let everything_request = GetEverythingRequest::builder()
        .search_term(String::from("Nvidia+NVDA+stock"))
        .language(Language::EN)
        .start_date(Utc.with_ymd_and_hms(2025, 3, 14, 0, 0, 0).unwrap())
        .end_date(Utc.with_ymd_and_hms(2025, 3, 20, 0, 0, 0).unwrap())
        .page_size(1)
        .build();
    match client.get_everything(&everything_request) {
        Ok(everything_response) => {
            log::info!(
                "Retrieved {:?} articles",
                everything_response.get_articles().len()
            );
            log::info!("{:?}", everything_response);
        }
        Err(err) => {
            log::error!(
                "{}",
                match err {
                    ApiClientError::InvalidResponse(response) => response.message.clone(),
                    _ => err.to_string(),
                }
            );
        }
    };
}
