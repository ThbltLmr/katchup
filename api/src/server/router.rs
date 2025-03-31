use crate::{
    adapters::{
        ollama_adapter::{OllamaAdapter, SummaryResult},
        tmdb_adapter::{SearchResults, ShowDetails, TmdbAdapter},
    },
    server::request_parser::Uri,
};

use serde::{Deserialize, Serialize};
use std::error::Error;

pub enum Route {
    GetShow(String),
    SearchShow(String),
    Summary(String),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum RouterResponse {
    SearchResults(SearchResults),
    ShowDetails(ShowDetails),
    SummaryResult(SummaryResult),
    None,
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
            Route::GetShow(query) => self.respond_get_show(&query).map(RouterResponse::from),
            Route::SearchShow(query) => self.respond_search(&query).map(RouterResponse::from),
            Route::Summary(query) => self.respond_summary(&query).map(RouterResponse::from),
        }
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
