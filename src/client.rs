use crate::constant::{NEWS_API_CLIENT_USER_AGENT, NEWS_API_KEY_ENV};
use crate::error::{ApiClientError, ApiClientErrorCode, ApiClientErrorResponse};
use crate::model::{
    GetEverythingRequest, GetEverythingResponse, GetTopHeadlinesRequest, TopHeadlinesResponse,
};
use reqwest::blocking::Client as BlockingClient;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

const NEWS_API_URI: &str = "https://newsapi.org/";
const TOP_HEADLINES_ENDPOINT: &str = "/v2/top-headlines";
const EVERYTHING_ENDPOINT: &str = "/v2/everything";

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

impl NewsApiClient<BlockingClient> {
    pub fn new(api_key: &str) -> Self {
        NewsApiClient {
            client: BlockingClient::new(),
            api_key: api_key.to_string(),
            base_url: Url::parse(NEWS_API_URI).unwrap(),
        }
    }

    pub fn from_env() -> Self {
        match env::var(NEWS_API_KEY_ENV) {
            Ok(api_key) => NewsApiClient::new(&api_key),
            Err(_) => panic!("{} is not set", NEWS_API_KEY_ENV),
        }
    }

    fn parse_error_response(&self, response_text: String) -> ApiClientError {
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

    pub fn get_everything(
        self,
        request: &GetEverythingRequest,
    ) -> Result<GetEverythingResponse, ApiClientError> {
        log::debug!("Request: {:?}", request);

        let mut url = self.base_url.clone();
        Self::get_endpoint_with_query_params_for_everything(&mut url, request);
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
        Self::top_headlines_validate_request(request)?;

        let mut url = self.base_url.clone();
        Self::get_endpoint_with_query_params_for_top_headlines(&mut url, request);
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

impl<T> NewsApiClient<T> {
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
            query_params.push(("language".to_string(), language.to_string()));
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
