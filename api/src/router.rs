use crate::{
    request_parser::Uri,
    tmdb_adapter::{SearchResults, TmdbAdapter},
};

pub enum Route {
    GetShow(String),
    SearchShow(String),
    Summary(String),
}

pub struct Router {
    tmdb_adapter: TmdbAdapter,
}

impl Router {
    pub fn new() -> Self {
        Router {
            tmdb_adapter: TmdbAdapter::new(),
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

    pub fn respond(&self, route: &Route) -> Result<SearchResults, Box<dyn std::error::Error>> {
        match route {
            Route::GetShow(query) => self.respond_get_show(&query),
            Route::SearchShow(query) => self.respond_search(&query),
            Route::Summary(query) => self.respond_summary(&query),
        }
    }

    fn respond_get_show(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {
        Ok(self.tmdb_adapter.search_tv_show(query)?)
    }

    fn respond_search(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {
        Ok(self.tmdb_adapter.search_tv_show(query)?)
    }

    fn respond_summary(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {
        Ok(self.tmdb_adapter.search_tv_show(query)?)
    }
}
