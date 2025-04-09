use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct OllamaAdapter {
    client: Client,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterList {
    pub characters: Vec<Character>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterListResult {
    pub done: bool,
    pub response: CharacterList,
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

    pub fn describe_cast(
        &self,
        characters: Vec<String>,
    ) -> Result<CharacterList, Box<dyn std::error::Error>> {
        let url = format!("{}/api/generate", std::env::var("OLLAMA_API_URL").unwrap());

        let prompt = format!("You are a critic for TV shows, who has watched every show ever written. Given a list of characters from a TV show, your job is to provide a short description of a character and their role in the story. For example, if you are asked about Joey Tribbiani, your answer could be: 'Actor, Chandler's roommate, great with women'. The characters you have to describe are the following: {:?}", characters);

        let body = json!({
                  "model": "llama3.2:3b",
                  "stream": false,
                  "prompt": prompt,
                  "format": {
                    "type": "object",
                    "properties": {
                        "characters": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string",
                                    },
                                    "description": {
                                        "type": "string"
                                    }
                                }
                    }
                }
          },
          "required": [
            "characters"
          ]
        }
              });

        let response: CharacterList = self
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
