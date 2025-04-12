use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use getset::{Getters, MutGetters};
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

fn default_page_size() -> i32 {
    1
}

fn default_page() -> i32 {
    1
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone)]
pub enum ArticleSortBy {
    #[strum(serialize = "publishedAt")]
    PublishedAt,
    #[strum(serialize = "relevancy")]
    Relevancy,
    #[strum(serialize = "popularity")]
    Popularity,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum SearchInOption {
    Title,
    Description,
    Content,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum NewsCategory {
    Business,
    Entertainment,
    General,
    Health,
    Science,
    Sports,
    Technology,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Country {
    AE,
    AR,
    AT,
    AU,
    BE,
    BG,
    BR,
    CA,
    CH,
    CN,
    CO,
    CU,
    CZ,
    DE,
    EG,
    FR,
    GB,
    GR,
    HK,
    HU,
    ID,
    IE,
    IL,
    IN,
    IT,
    JP,
    KR,
    LT,
    LV,
    MA,
    MX,
    MY,
    NG,
    NL,
    NO,
    NZ,
    PH,
    PL,
    PT,
    RO,
    RS,
    RU,
    SA,
    SE,
    SG,
    SI,
    SK,
    TH,
    TR,
    TW,
    UA,
    US,
    VE,
    ZA,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    AR,
    DE,
    EN,
    ES,
    FR,
    HE,
    IT,
    NL,
    NO,
    PT,
    RU,
    SV,
    UD,
    ZH,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Source {
    id: Option<String>,

    name: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct Article {
    source: Source,

    author: Option<String>,

    title: String,

    description: Option<String>,

    url: String,

    #[serde(rename = "urlToImage")]
    url_to_image: Option<String>,

    #[serde(rename = "publishedAt")]
    published_at: String,

    content: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct GetTopHeadlinesRequest {
    country: Option<Country>,

    category: Option<NewsCategory>,

    sources: Option<String>,

    #[serde(rename = "q")]
    search_term: String,

    #[serde(rename = "pageSize", default = "default_page_size")]
    #[validate(range(min = 1, max = 100))]
    page_size: i32,

    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    page: i32,
}

impl GetTopHeadlinesRequest {
    pub fn builder() -> GetTopHeadlinesRequestBuilder {
        GetTopHeadlinesRequestBuilder::new()
    }
}

#[derive(Default)]
pub struct GetTopHeadlinesRequestBuilder {
    country: Option<Country>,

    category: Option<NewsCategory>,

    sources: Option<String>,

    search_term: String,

    page_size: i32,

    page: i32,
}

impl GetTopHeadlinesRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(mut self, country: Country) -> Self {
        self.country = Option::Some(country);
        self
    }

    pub fn category(mut self, category: NewsCategory) -> Self {
        self.category = Option::Some(category);
        self
    }

    pub fn sources(mut self, sources: String) -> Self {
        self.sources = Option::Some(sources);
        self
    }

    pub fn search_term(mut self, search_term: String) -> Self {
        self.search_term = search_term;
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = page;
        self
    }

    pub fn build(self) -> Result<GetTopHeadlinesRequest, &'static str> {
        if self.sources.is_some() && (self.country.is_some() || self.category.is_some()) {
            return Err("Cannot specify sources with country or category");
        }
        Ok(GetTopHeadlinesRequest {
            country: self.country,
            category: self.category,
            sources: self.sources,
            search_term: self.search_term,
            page_size: self.page_size,
            page: self.page,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct TopHeadlinesResponse {
    status: String,

    #[serde(rename = "totalResults")]
    total_results: i32,

    articles: Vec<Article>,
}

#[derive(Serialize, Deserialize, Validate, Debug, Getters, MutGetters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct GetEverythingRequest {
    #[serde(rename = "q")]
    search_term: String,

    search_in: Vec<SearchInOption>,

    sources: Option<String>,

    domains: Option<String>,

    #[serde(rename = "excludeDomains")]
    exclude_domains: Option<String>,

    #[serde(rename = "from", with = "ts_seconds_option")]
    start_date: Option<DateTime<Utc>>,

    #[serde(rename = "to", with = "ts_seconds_option")]
    end_date: Option<DateTime<Utc>>,

    language: Option<Language>,

    #[serde(rename = "sortBy")]
    sort_by: Option<String>,

    #[serde(rename = "pageSize", default = "default_page_size")]
    #[validate(range(min = 1, max = 100))]
    page_size: i32,

    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    page: i32,
}

impl GetEverythingRequest {
    pub fn builder() -> GetEverythingRequestBuilder {
        GetEverythingRequestBuilder::new()
    }
}

#[derive(Default)]
pub struct GetEverythingRequestBuilder {
    search_term: String,

    search_in: Vec<SearchInOption>,

    sources: Option<String>,

    domains: Option<String>,

    exclude_domains: Option<String>,

    start_date: Option<DateTime<Utc>>,

    end_date: Option<DateTime<Utc>>,

    language: Option<Language>,

    sort_by: Option<ArticleSortBy>,

    page_size: i32,

    page: i32,
}

impl GetEverythingRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_term(mut self, search_term: String) -> Self {
        self.search_term = search_term;
        self
    }

    pub fn search_in(mut self, search_in: Vec<SearchInOption>) -> Self {
        self.search_in = search_in;
        self
    }

    pub fn sources(mut self, sources: String) -> Self {
        self.sources = Option::Some(sources);
        self
    }

    pub fn domains(mut self, domains: String) -> Self {
        self.domains = Option::Some(domains);
        self
    }

    pub fn exclude_domains(mut self, exclude_domains: String) -> Self {
        self.exclude_domains = Option::Some(exclude_domains);
        self
    }

    pub fn start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Option::Some(start_date);
        self
    }

    pub fn end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end_date = Option::Some(end_date);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Option::Some(language);
        self
    }

    pub fn sort_by(mut self, sort_by: ArticleSortBy) -> Self {
        self.sort_by = Option::Some(sort_by);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = page;
        self
    }

    pub fn build(self) -> GetEverythingRequest {
        GetEverythingRequest {
            search_term: self.search_term,
            search_in: self.search_in,
            sources: self.sources,
            domains: self.domains,
            exclude_domains: self.exclude_domains,
            start_date: self.start_date,
            end_date: self.end_date,
            language: self.language,
            sort_by: self.sort_by.map(|article_sort| article_sort.to_string()),
            page_size: self.page_size,
            page: self.page,
        }
    }
}

#[derive(Serialize, Deserialize, Getters, Debug)]
#[getset(get = "pub with_prefix")]
pub struct GetEverythingResponse {
    status: String,

    #[serde(rename = "totalResults")]
    total_results: i32,

    articles: Vec<Article>,
}
