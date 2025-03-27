use reqwest::{Method, blocking::Client, header::ACCEPT};
use serde::{Deserialize, Serialize};
use std::error::Error;

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

#[derive(Deserialize, Serialize)]
pub struct ShowDetails {
    pub number_of_episodes: u8,
    pub number_of_seasons: u8,
}

impl TmdbAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn search_tv_show(&self, query: &str) -> Result<SearchResults, Box<dyn Error>> {
        let request_url = format!("https://api.themoviedb.org/3/search/tv?{}", query);

        let api_token = std::env::var("TMDB_API_TOKEN").unwrap();

        let builder = self
            .client
            .request(Method::GET, request_url)
            .bearer_auth(api_token)
            .header(ACCEPT, "application/json");

        let response = builder.send()?;

        println!("TMDB response: {response:#?}");

        Ok(response.json().expect("Could not format json"))
    }

    pub fn get_tv_show(&self, query: &str) -> Result<ShowDetails, Box<dyn Error>> {
        let request_url = format!(
            "https://api.themoviedb.org/3/search/tv/{}",
            query.split_once('=').expect("Could not find id in query").0
        );

        let api_token = std::env::var("TMDB_API_TOKEN").unwrap();

        let builder = self
            .client
            .request(Method::GET, request_url)
            .bearer_auth(api_token)
            .header(ACCEPT, "application/json");

        let response = builder.send()?;

        println!("TMDB response: {response:#?}");

        Ok(response.json().expect("Could not format json"))
    }
}
