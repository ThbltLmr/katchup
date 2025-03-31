use reqwest::{blocking::Client, header::ACCEPT, Method};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct TvResult {
    pub id: usize,
    pub name: String,
    pub poster_path: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SearchResults {
    pub results: Vec<TvResult>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ShowDetailsSeason {
    pub id: usize,
    pub name: String,
    pub episode_count: usize,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ShowDetails {
    pub number_of_episodes: usize,
    pub number_of_seasons: usize,
    pub seasons: Vec<ShowDetailsSeason>,
}

pub struct TmdbAdapter {
    client: Client,
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
            "https://api.themoviedb.org/3/tv/{}",
            query.split_once('=').expect("Could not find id in query").1
        );

        let api_token = std::env::var("TMDB_API_TOKEN").unwrap();

        let builder = self
            .client
            .request(Method::GET, request_url)
            .bearer_auth(api_token)
            .header(ACCEPT, "application/json");

        let response = builder.send()?;

        println!("TMDB response: {response:#?}");

        let details: ShowDetails = response.json().expect("Could not format json");
        Ok(self.format_show_seasons(details))
    }

    fn format_show_seasons(&self, mut details: ShowDetails) -> ShowDetails {
        let sum_episodes = details
            .seasons
            .iter()
            .fold(0, |sum, season| sum + season.episode_count);

        if sum_episodes > details.number_of_episodes {
            details.seasons.remove(0);
        }

        details
    }
}
