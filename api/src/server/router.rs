use crate::{
    adapters::{
        ollama_adapter::{OllamaAdapter, SummaryResult},
        tmdb_adapter::{CastDetails, SearchResults, ShowDetails, TmdbAdapter},
    },
    server::request_parser::Uri,
};

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug)]
pub enum Route {
    GetCast(String),
    GetShow(String),
    SearchShow(String),
    Summary(String),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum RouterResponse {
    CastDetails(CastDetails),
    SearchResults(SearchResults),
    ShowDetails(ShowDetails),
    SummaryResult(SummaryResult),
    None,
}

impl From<CastDetails> for RouterResponse {
    fn from(details: CastDetails) -> Self {
        RouterResponse::CastDetails(details)
    }
}

impl From<SearchResults> for RouterResponse {
    fn from(results: SearchResults) -> Self {
        RouterResponse::SearchResults(results)
    }
}

impl From<ShowDetails> for RouterResponse {
    fn from(details: ShowDetails) -> Self {
        RouterResponse::ShowDetails(details)
    }
}

impl From<SummaryResult> for RouterResponse {
    fn from(summary: SummaryResult) -> Self {
        RouterResponse::SummaryResult(summary)
    }
}

pub struct Router {
    tmdb_adapter: TmdbAdapter,
    ollama_adapter: OllamaAdapter,
}

impl Router {
    pub fn new() -> Self {
        Router {
            tmdb_adapter: TmdbAdapter::new(),
            ollama_adapter: OllamaAdapter::new(),
        }
    }

    pub fn get_route(&self, uri: &Uri) -> Option<Route> {
        match uri.path.as_str() {
            "/cast" => Some(Route::GetCast(
                uri.query.clone().unwrap_or(String::from("")),
            )),
            "/shows" => Some(Route::GetShow(
                uri.query.clone().unwrap_or(String::from("")),
            )),
            "/search" => Some(Route::SearchShow(
                uri.query.clone().unwrap_or(String::from("")),
            )),
            "/summary" => Some(Route::Summary(
                uri.query.clone().unwrap_or(String::from("")),
            )),
            _ => None,
        }
    }

    pub fn respond(&self, route: &Route) -> Result<RouterResponse, Box<dyn Error>> {
        match route {
            Route::GetCast(query) => self.respond_get_cast(&query).map(RouterResponse::from),
            Route::GetShow(query) => self.respond_get_show(&query).map(RouterResponse::from),
            Route::SearchShow(query) => self.respond_search(&query).map(RouterResponse::from),
            Route::Summary(query) => self.respond_summary(&query).map(RouterResponse::from),
        }
    }

    fn respond_get_cast(&self, query: &str) -> Result<CastDetails, Box<dyn Error>> {
        Ok(self.tmdb_adapter.get_cast(query)?)
    }

    fn respond_get_show(&self, query: &str) -> Result<ShowDetails, Box<dyn Error>> {
        Ok(self.tmdb_adapter.get_tv_show(query)?)
    }

    fn respond_search(&self, query: &str) -> Result<SearchResults, Box<dyn Error>> {
        Ok(self.tmdb_adapter.search_tv_show(query)?)
    }

    fn respond_summary(&self, query: &str) -> Result<SummaryResult, Box<dyn Error>> {
        Ok(self.ollama_adapter.summarize_show(query)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_route() {
        let router = Router::new();
        let uri_shows = Uri {
            path: "/shows".to_string(),
            query: Some("Test Show".to_string()),
        };
        let uri_search = Uri {
            path: "/search".to_string(),
            query: Some("Test Show".to_string()),
        };
        let uri_summary = Uri {
            path: "/summary".to_string(),
            query: Some("Test Show".to_string()),
        };

        match router.get_route(&uri_shows) {
            Some(Route::GetShow(query)) => assert_eq!(query, "Test Show"),
            _ => panic!("Expected GetShow route"),
        }

        match router.get_route(&uri_search) {
            Some(Route::SearchShow(query)) => assert_eq!(query, "Test Show"),
            _ => panic!("Expected SearchShow route"),
        }

        match router.get_route(&uri_summary) {
            Some(Route::Summary(query)) => assert_eq!(query, "Test Show"),
            _ => panic!("Expected Summary route"),
        }
    }
}
