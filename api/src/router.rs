use crate::{
    request_parser::Uri,
    tmdb_adapter::{SearchResults, TmdbAdapter},
};

pub enum Route {
    Search(String),
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
            "/search" => Some(Route::Search(uri.query.clone().unwrap_or(String::from("")))),
            "/summary" => Some(Route::Summary(
                uri.query.clone().unwrap_or(String::from("")),
            )),
            _ => None,
        }
    }

    pub fn respond(&self, route: &Route) -> Result<SearchResults, Box<dyn std::error::Error>> {
        match route {
            Route::Search(query) => self.respond_search(&query),
            Route::Summary(query) => self.respond_summary(&query),
        }
    }

    fn respond_search(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {
        Ok(self.tmdb_adapter.search_tv_show(query)?)
    }

    fn respond_summary(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {
        Ok(self.tmdb_adapter.search_tv_show(query)?)
    }
}
