use crate::constant::{
    EVERYTHING_ENDPOINT, NEWS_API_CLIENT_USER_AGENT, NEWS_API_KEY_ENV, NEWS_API_URI,
    TOP_HEADLINES_ENDPOINT,
};
use crate::error::{ApiClientError, ApiClientErrorCode, ApiClientErrorResponse};
use crate::model::{
    GetEverythingRequest, GetEverythingResponse, GetTopHeadlinesRequest, TopHeadlinesResponse,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct NewsApiErrorResponse {
    status: String,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NewsApiClient<T> {
    client: T,
    api_key: String,
    base_url: Url,
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use reqwest::blocking::Client as BlockingClient;

    impl NewsApiClient<BlockingClient> {
        pub fn new(api_key: &str) -> Self {
            NewsApiClient {
                client: BlockingClient::new(),
                api_key: api_key.to_string(),
                base_url: Url::parse(NEWS_API_URI).unwrap(),
            }
        }

        fn parse_error_response(&self, response_text: String) -> ApiClientError {
            NewsApiClient::<BlockingClient>::parse_error_response_internal(response_text)
        }

        pub fn get_everything(
            self,
            request: &GetEverythingRequest,
        ) -> Result<GetEverythingResponse, ApiClientError> {
            log::debug!("Request: {:?}", request);

            let mut url = self.base_url.clone();
            NewsApiClient::<BlockingClient>::get_endpoint_with_query_params_for_everything(
                &mut url, request,
            );
            log::debug!("Request URL: {}", url.as_str());

            let headers = self.get_request_headers()?;
            let response = self.client.get(url.as_str()).headers(headers).send()?;
            log::debug!("Response status: {:?}", response.status());

            if response.status().is_success() {
                let response_text = response.text()?;
                match serde_json::from_str::<GetEverythingResponse>(&response_text) {
                    Ok(everything_response) => Ok(everything_response),
                    Err(e) => Err(ApiClientError::InvalidRequest(format!("{}", e))),
                }
            } else {
                let response_text = response.text()?;
                Err(self.parse_error_response(response_text))
            }
        }

        pub fn get_top_headlines(
            self,
            request: &GetTopHeadlinesRequest,
        ) -> Result<TopHeadlinesResponse, ApiClientError> {
            log::debug!("Request: {:?}", request);
            NewsApiClient::<BlockingClient>::top_headlines_validate_request(request)?;

            let mut url = self.base_url.clone();
            NewsApiClient::<BlockingClient>::get_endpoint_with_query_params_for_top_headlines(
                &mut url, request,
            );
            log::debug!("Request URL: {}", url.as_str());

            let headers = self.get_request_headers()?;
            let response = self.client.get(url.as_str()).headers(headers).send()?;
            log::debug!("Response status: {:?}", response.status());

            if response.status().is_success() {
                let response_text = response.text()?;
                match serde_json::from_str::<TopHeadlinesResponse>(&response_text) {
                    Ok(headline_response) => Ok(headline_response),
                    Err(e) => Err(ApiClientError::InvalidRequest(format!(
                        "Failed to parse response: {}",
                        e
                    ))),
                }
            } else {
                let response_text = response.text()?;
                Err(self.parse_error_response(response_text))
            }
        }
    }
}

impl NewsApiClient<reqwest::Client> {
    pub fn new_async(api_key: &str) -> Self {
        NewsApiClient {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            base_url: Url::parse(NEWS_API_URI).unwrap(),
        }
    }

    pub fn from_env_async() -> Self {
        match env::var(NEWS_API_KEY_ENV) {
            Ok(api_key) => NewsApiClient::new_async(&api_key),
            Err(_) => panic!("{} is not set", NEWS_API_KEY_ENV),
        }
    }

    fn parse_error_response(&self, response_text: String) -> ApiClientError {
        NewsApiClient::<reqwest::Client>::parse_error_response_internal(response_text)
    }

    pub async fn get_everything(
        &self,
        request: &GetEverythingRequest,
    ) -> Result<GetEverythingResponse, ApiClientError> {
        log::debug!("Request: {:?}", request);

        let mut url = self.base_url.clone();
        Self::get_endpoint_with_query_params_for_everything(&mut url, request);
        log::debug!("Request URL: {}", url.as_str());

        let headers = self.get_request_headers()?;
        let response = self
            .client
            .get(url.as_str())
            .headers(headers)
            .send()
            .await?;
        log::debug!("Response status: {:?}", response.status());

        if response.status().is_success() {
            let response_text = response.text().await?;
            match serde_json::from_str::<GetEverythingResponse>(&response_text) {
                Ok(everything_response) => Ok(everything_response),
                Err(e) => Err(ApiClientError::InvalidRequest(format!("{}", e))),
            }
        } else {
            let response_text = response.text().await?;
            Err(self.parse_error_response(response_text))
        }
    }

    pub async fn get_top_headlines(
        &self,
        request: &GetTopHeadlinesRequest,
    ) -> Result<TopHeadlinesResponse, ApiClientError> {
        log::debug!("Request: {:?}", request);
        Self::top_headlines_validate_request(request)?;

        let mut url = self.base_url.clone();
        Self::get_endpoint_with_query_params_for_top_headlines(&mut url, request);
        log::debug!("Request URL: {}", url.as_str());

        let headers = self.get_request_headers()?;
        let response = self
            .client
            .get(url.as_str())
            .headers(headers)
            .send()
            .await?;
        log::debug!("Response status: {:?}", response.status());

        if response.status().is_success() {
            let response_text = response.text().await?;
            match serde_json::from_str::<TopHeadlinesResponse>(&response_text) {
                Ok(headline_response) => Ok(headline_response),
                Err(e) => Err(ApiClientError::InvalidRequest(format!(
                    "Failed to parse response: {}",
                    e
                ))),
            }
        } else {
            let response_text = response.text().await?;
            Err(self.parse_error_response(response_text))
        }
    }
}

// Add this top-level function outside the blocking module
#[cfg(feature = "blocking")]
impl NewsApiClient<reqwest::blocking::Client> {
    /// Creates a new NewsApiClient instance using the API key from the environment variable
    pub fn from_env() -> Self {
        match env::var(NEWS_API_KEY_ENV) {
            Ok(api_key) => Self::new(&api_key),
            Err(_) => panic!("{} is not set", NEWS_API_KEY_ENV),
        }
    }
}

// Shared implementation for both client types
impl<T> NewsApiClient<T> {
    // Internal method for parsing error responses
    fn parse_error_response_internal(response_text: String) -> ApiClientError {
        match serde_json::from_str::<NewsApiErrorResponse>(&response_text) {
            Ok(error_response) => {
                let error_code = match error_response.code.as_deref() {
                    Some("apiKeyDisabled") => ApiClientErrorCode::ApiKeyDisabled,
                    Some("apiKeyExhausted") => ApiClientErrorCode::ApiKeyExhausted,
                    Some("apiKeyInvalid") => ApiClientErrorCode::ApiKeyInvalid,
                    Some("apiKeyMissing") => ApiClientErrorCode::ApiKeyMissing,
                    Some("parameterInvalid") => ApiClientErrorCode::ParameterInvalid,
                    Some("parametersMissing") => ApiClientErrorCode::ParametersMissing,
                    Some("rateLimited") => ApiClientErrorCode::RateLimited,
                    Some("sourcesTooMany") => ApiClientErrorCode::SourcesTooMany,
                    Some("sourceDoesNotExist") => ApiClientErrorCode::SourceDoesNotExist,
                    _ => ApiClientErrorCode::UnexpectedError,
                };

                ApiClientError::InvalidResponse(ApiClientErrorResponse {
                    status: error_response.status,
                    code: error_code,
                    message: error_response
                        .message
                        .unwrap_or_else(|| "Unknown error".to_string()),
                })
            }
            Err(_) => ApiClientError::InvalidResponse(ApiClientErrorResponse {
                status: "error".to_string(),
                code: ApiClientErrorCode::UnexpectedError,
                message: "Failed to parse error response".to_string(),
            }),
        }
    }

    fn get_request_headers(&self) -> Result<HeaderMap, ApiClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))?,
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(NEWS_API_CLIENT_USER_AGENT),
        );
        Ok(headers)
    }

    fn top_headlines_validate_request(
        request: &GetTopHeadlinesRequest,
    ) -> Result<(), ApiClientError> {
        log::debug!("Validating request");
        if request.get_sources().is_some()
            && (request.get_country().is_some() || request.get_category().is_some())
        {
            return Err(ApiClientError::InvalidRequest(
                "Cannot specify sources with country or category".to_string(),
            ));
        }
        Ok(())
    }

    fn get_endpoint_with_query_params_for_top_headlines(
        url: &mut Url,
        request: &GetTopHeadlinesRequest,
    ) {
        url.set_path(TOP_HEADLINES_ENDPOINT);
        url.query_pairs_mut().clear();

        for (key, value) in Self::get_top_headlines_query_params(request) {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        url.query_pairs_mut().finish();
    }

    fn get_top_headlines_query_params(request: &GetTopHeadlinesRequest) -> Vec<(String, String)> {
        let mut query_params = Vec::new();

        if let Some(country) = request.get_country() {
            query_params.push(("country".to_string(), country.to_string()));
        }

        if let Some(category) = request.get_category() {
            query_params.push(("category".to_string(), category.to_string()));
        }

        if let Some(sources) = request.get_sources() {
            query_params.push(("sources".to_string(), sources.to_string()));
        }

        if !request.get_search_term().is_empty() {
            query_params.push(("q".to_string(), request.get_search_term().to_string()));
        }

        if *request.get_page_size() > 1 {
            query_params.push(("pageSize".to_string(), request.get_page_size().to_string()));
        }

        if *request.get_page() > 1 {
            query_params.push(("page".to_string(), request.get_page().to_string()));
        }

        query_params
    }

    fn get_endpoint_with_query_params_for_everything(
        url: &mut Url,
        request: &GetEverythingRequest,
    ) {
        url.set_path(EVERYTHING_ENDPOINT);
        url.query_pairs_mut().clear();

        let query_params = Self::get_everything_query_params(request);
        for (key, value) in query_params {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        url.query_pairs_mut().finish();
    }

    fn get_everything_query_params(request: &GetEverythingRequest) -> Vec<(String, String)> {
        let mut query_params = Vec::new();

        query_params.push(("q".to_string(), request.get_search_term().to_string()));

        if let Some(language) = request.get_language() {
            query_params.push(("language".to_string(), language.to_string().to_lowercase()));
        }

        if let Some(start_date) = request.get_start_date() {
            query_params.push(("from".to_string(), start_date.to_rfc3339()));
        }

        if let Some(end_date) = request.get_end_date() {
            query_params.push(("to".to_string(), end_date.to_rfc3339()));
        }

        if *request.get_page_size() > 0 {
            query_params.push(("pageSize".to_string(), request.get_page_size().to_string()));
        }

        if *request.get_page() > 1 {
            query_params.push(("page".to_string(), request.get_page().to_string()));
        }

        query_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Country, Language, NewsCategory};
    use chrono::{DateTime, Utc};
    use mockito::{self};
    use std::collections::HashMap;
    use std::str::FromStr;

    // Helper function to create a client with mocked base URL
    fn create_test_client() -> NewsApiClient<reqwest::Client> {
        let api_key = "test-api-key";
        let mut client = NewsApiClient::new_async(api_key);
        // Get the mock server URL - using server_address() instead of server_url()
        let server = mockito::Server::new();
        let mock_url = server.url();
        client.base_url = Url::parse(&format!("http://{}", mock_url)).unwrap();
        client
    }

    #[test]
    fn test_parse_error_response() {
        let error_json =
            r#"{"status":"error","code":"apiKeyInvalid","message":"Your API key is invalid"}"#;
        let error =
            NewsApiClient::<reqwest::Client>::parse_error_response_internal(error_json.to_string());

        match error {
            ApiClientError::InvalidResponse(response) => {
                assert_eq!(response.status, "error");
                assert_eq!(response.code, ApiClientErrorCode::ApiKeyInvalid);
                assert_eq!(response.message, "Your API key is invalid");
            }
            _ => panic!("Expected InvalidResponse error"),
        }

        let error_json =
            r#"{"status":"error","code":"parameterInvalid","message":"Invalid parameter"}"#;
        let error =
            NewsApiClient::<reqwest::Client>::parse_error_response_internal(error_json.to_string());

        match error {
            ApiClientError::InvalidResponse(response) => {
                assert_eq!(response.code, ApiClientErrorCode::ParameterInvalid);
            }
            _ => panic!("Expected InvalidResponse error"),
        }

        let error_json = r#"invalid json"#;
        let error =
            NewsApiClient::<reqwest::Client>::parse_error_response_internal(error_json.to_string());

        match error {
            ApiClientError::InvalidResponse(response) => {
                assert_eq!(response.code, ApiClientErrorCode::UnexpectedError);
            }
            _ => panic!("Expected InvalidResponse error"),
        }
    }

    #[test]
    fn test_get_request_headers() {
        let client = create_test_client();
        let headers = client.get_request_headers().unwrap();

        assert_eq!(
            headers.get(AUTHORIZATION).unwrap().to_str().unwrap(),
            "Bearer test-api-key"
        );
        assert_eq!(
            headers.get(USER_AGENT).unwrap().to_str().unwrap(),
            NEWS_API_CLIENT_USER_AGENT
        );
    }

    #[test]
    fn test_top_headlines_validate_request_country_and_category() {
        let request = GetTopHeadlinesRequest::builder()
            .country(Country::US)
            .category(NewsCategory::Business)
            .search_term(String::new())
            .page_size(20)
            .page(1)
            .build()
            .unwrap();
        assert!(NewsApiClient::<reqwest::Client>::top_headlines_validate_request(&request).is_ok());
    }

    #[test]
    fn test_top_headlines_validate_request_sources_only() {
        let request = GetTopHeadlinesRequest::builder()
            .sources("bbc-news,cnn".to_string())
            .search_term(String::new())
            .page_size(20)
            .page(1)
            .build()
            .unwrap();
        assert!(NewsApiClient::<reqwest::Client>::top_headlines_validate_request(&request).is_ok());
    }

    #[test]
    fn test_top_headlines_validate_request_sources_with_country() {
        let request = GetTopHeadlinesRequest::builder()
            .sources("bbc-news".to_string())
            .country(Country::US)
            .search_term(String::new())
            .page_size(20)
            .page(1)
            .build();

        assert!(request.is_err());
    }

    #[test]
    fn test_top_headlines_validate_request_sources_with_category() {
        let request = GetTopHeadlinesRequest::builder()
            .sources("bbc-news".to_string())
            .category(NewsCategory::Business)
            .search_term(String::new())
            .page_size(20)
            .page(1)
            .build();

        assert!(request.is_err());
    }

    #[test]
    fn test_get_top_headlines_query_params() {
        let request = GetTopHeadlinesRequest::builder()
            .country(Country::US)
            .category(NewsCategory::Technology)
            .search_term("ai".to_string())
            .page_size(15)
            .page(2)
            .build()
            .unwrap();

        let params = NewsApiClient::<reqwest::Client>::get_top_headlines_query_params(&request);
        let params_map: HashMap<_, _> = params.into_iter().collect();

        assert_eq!(params_map.get("country").unwrap(), "us");
        assert_eq!(params_map.get("category").unwrap(), "technology");
        assert_eq!(params_map.get("q").unwrap(), "ai");
        assert_eq!(params_map.get("page").unwrap(), "2");
        assert_eq!(params_map.get("pageSize").unwrap(), "15");
    }

    #[test]
    fn test_get_everything_query_params() {
        let start_date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end_date = DateTime::<Utc>::from_str("2023-01-31T23:59:59Z").unwrap();

        let request = GetEverythingRequest::builder()
            .search_term(format!("bitcoin"))
            .language(Language::AR)
            .start_date(start_date)
            .end_date(end_date)
            .page(3)
            .page_size(20)
            .build();

        let params = NewsApiClient::<reqwest::Client>::get_everything_query_params(&request);
        let params_map: HashMap<_, _> = params.into_iter().collect();

        assert_eq!(params_map.get("q").unwrap(), "bitcoin");
        assert_eq!(params_map.get("language").unwrap(), "ar"); // Fix expectation to "ar" instead of "en"
        assert_eq!(params_map.get("from").unwrap(), "2023-01-01T00:00:00+00:00");
        assert_eq!(params_map.get("to").unwrap(), "2023-01-31T23:59:59+00:00");
        assert_eq!(params_map.get("page").unwrap(), "3");
        assert_eq!(params_map.get("pageSize").unwrap(), "20");
    }

    #[tokio::test]
    async fn test_get_everything_async() {
        let mock_response = r#"{
            "status": "ok",
            "totalResults": 2,
            "articles": [
                {
                    "source": {"id": "test-source", "name": "Test Source"},
                    "author": "Test Author",
                    "title": "Test Title",
                    "description": "Test Description",
                    "url": "https://example.com/article1",
                    "urlToImage": "https://example.com/image1.jpg",
                    "publishedAt": "2023-05-01T12:00:00Z",
                    "content": "Test content"
                },
                {
                    "source": {"id": "test-source-2", "name": "Test Source 2"},
                    "author": "Test Author 2",
                    "title": "Test Title 2",
                    "description": "Test Description 2",
                    "url": "https://example.com/article2",
                    "urlToImage": "https://example.com/image2.jpg",
                    "publishedAt": "2023-05-02T12:00:00Z",
                    "content": "Test content 2"
                }
            ]
        }"#;

        let mut server = mockito::Server::new_async().await;

        let _m = server
            .mock("GET", "/v2/everything")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create_async()
            .await;

        // Create a client using the server URL
        let mut client = NewsApiClient::new_async("test-api-key");
        client.base_url = Url::parse(&format!("{}", server.url())).unwrap();

        let request = GetEverythingRequest::builder()
            .search_term(format!("test"))
            .build();

        let response = client.get_everything(&request).await.unwrap();

        assert_eq!(response.get_status(), "ok");
        assert_eq!(*response.get_total_results(), 2);
        assert_eq!(response.get_articles().len(), 2);
        assert_eq!(response.get_articles()[0].get_title(), "Test Title");
        assert_eq!(response.get_articles()[1].get_title(), "Test Title 2");
    }

    #[tokio::test]
    async fn test_get_top_headlines_async() {
        let mock_response = r#"{
            "status": "ok",
            "totalResults": 1,
            "articles": [
                {
                    "source": {"id": "test-source", "name": "Test Source"},
                    "author": "Test Author",
                    "title": "Breaking News",
                    "description": "Test Description",
                    "url": "https://example.com/article1",
                    "urlToImage": "https://example.com/image1.jpg",
                    "publishedAt": "2023-05-01T12:00:00Z",
                    "content": "Test content"
                }
            ]
        }"#;

        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/v2/top-headlines")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create_async()
            .await;
        let mut client = NewsApiClient::new_async("test-api-key");
        client.base_url = Url::parse(&format!("{}", server.url())).unwrap();

        let request = GetTopHeadlinesRequest::builder()
            .country(Country::US)
            .search_term(String::new())
            .page_size(20)
            .page(1)
            .build()
            .unwrap();

        let response = client.get_top_headlines(&request).await.unwrap();

        assert_eq!(response.get_status(), "ok");
        assert_eq!(*response.get_total_results(), 1);
        assert_eq!(response.get_articles().len(), 1);
        assert_eq!(response.get_articles()[0].get_title(), "Breaking News");
    }

    #[tokio::test]
    async fn test_error_responses_async() {
        let error_response = r#"{
            "status": "error",
            "code": "apiKeyInvalid",
            "message": "Your API key is invalid or incorrect"
        }"#;

        let mut server = mockito::Server::new_async().await;

        let _m = server
            .mock("GET", "/v2/everything")
            .match_query(mockito::Matcher::Any)
            .with_status(401)
            .with_body(error_response)
            .create_async()
            .await;

        let mut client = NewsApiClient::new_async("test-api-key");
        client.base_url = Url::parse(&format!("{}", server.url())).unwrap();

        let request = GetEverythingRequest::builder()
            .search_term(format!("test"))
            .build();

        let result = client.get_everything(&request).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            ApiClientError::InvalidResponse(response) => {
                assert_eq!(response.code, ApiClientErrorCode::ApiKeyInvalid);
            }
            _ => panic!("Expected InvalidResponse error"),
        }
    }

    #[cfg(feature = "blocking")]
    mod blocking_tests {
        use super::*;
        use mockito::Mock;

        #[test]
        fn test_get_everything_blocking() {
            let mock_response = r#"{
                "status": "ok",
                "totalResults": 1,
                "articles": [
                    {
                        "source": {"id": "test-source", "name": "Test Source"},
                        "author": "Test Author",
                        "title": "Test Title Blocking",
                        "description": "Test Description",
                        "url": "https://example.com/article1",
                        "urlToImage": "https://example.com/image1.jpg",
                        "publishedAt": "2023-05-01T12:00:00Z",
                        "content": "Test content"
                    }
                ]
            }"#;
            let mut server = mockito::Server::new();
            let _m: Mock = server
                .mock("GET", "/v2/everything")
                .match_query(mockito::Matcher::Any)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(mock_response)
                .create();

            let mut client = NewsApiClient::new("test-api-key");
            client.base_url = Url::parse(&format!("{}", server.url())).unwrap();
            let request = GetEverythingRequest::builder()
                .search_term("test".to_string())
                .build();
            let response = client.get_everything(&request).unwrap();

            assert_eq!(response.get_status(), "ok");
            assert_eq!(*response.get_total_results(), 1);
            assert_eq!(
                response.get_articles()[0].get_title(),
                "Test Title Blocking"
            );
        }
    }
}
