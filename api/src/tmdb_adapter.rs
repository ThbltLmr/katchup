use reqwest::{Method, blocking::Client, header::ACCEPT};
use serde::{Deserialize, Serialize};

pub struct TmdbAdapter {
    client: Client,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TvResult {
    pub id: usize,
    pub name: String,
    pub poster_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResults {
    pub results: Vec<TvResult>,
}

impl TmdbAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn search_tv_show(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {
        let request_url = format!("https://api.themoviedb.org/3/search/tv?{}", query);

        let api_token = std::env::var("TMDB_API_TOKEN").unwrap();

        let builder = self
            .client
            .request(Method::GET, request_url)
            .bearer_auth(api_token)
            .header(ACCEPT, "application/json");

        let response = builder.send()?;

        println!("TMDB respnse: {response:#?}");

        Ok(response.json().expect("Could not format json"))
    }
}
