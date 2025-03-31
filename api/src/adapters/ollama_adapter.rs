use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct OllamaAdapter {
    client: Client,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SummaryResult {
    pub done: bool,
    pub response: String,
}

impl OllamaAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn summarize_show(&self, query: &str) -> Result<SummaryResult, Box<dyn std::error::Error>> {
        let show = query
            .split_once('=')
            .expect("Could not find show name in query")
            .1;

        let url = format!("{}/api/generate", std::env::var("OLLAMA_API_URL").unwrap());

        let prompt = format!("Tell me a summary about the TV show: {}", show);

        let body = json!({
            "model": "llama3.2:3b",
            "stream": false,
            "prompt": prompt
        });

        let response: SummaryResult = self
            .client
            .post(url)
            .body(body.to_string())
            .send()
            .unwrap()
            .json()
            .unwrap();

        println!("{:?}", response);

        Ok(response)
    }
}
