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
        let params: Vec<&str> = query.split('&').collect();
        let (show, season, episode) = (
            params[0].split('=').collect::<Vec<&str>>()[1],
            params[1].split('=').collect::<Vec<&str>>()[1],
            params[2].split('=').collect::<Vec<&str>>()[1],
        );

        let url = format!("{}/api/generate", std::env::var("OLLAMA_API_URL").unwrap());

        let prompt = format!("You are a critic for TV shows, who has watched every show ever written. People come to you when they want to catch up to a TV show. You are given the name of the show, as well as the season and episode that the person will watch next. You should give them a detailed summary of what happened until that point. You will only summarize facts, and you will include every major event in your summary. You will not include any opinions or recommandations. You will start the summary with a quick explanation of when and where the show is set, then move to what happens in the show. You will avoid spoilers at all costs, or you will lose your job. For instance, if asked to summarize a show up to S2E2, you should summarize what happened in season 1 and in the first episode of season 2, but no further. Now summarize the show {} up to season {} episode {}", show, season, episode);

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
            .json()?;

        println!("{:?}", response);

        Ok(response)
    }

    pub fn describe_cast(
        &self,
        characters: Vec<String>,
    ) -> Result<CharacterListResult, Box<dyn std::error::Error>> {
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
                                    "type": "string"
                                },
                                "description": {
                                    "type": "string"
                                }
                            },
                            "required": ["name", "description"]
                        }
                    }
                },
                "required": [
                    "characters"
                ]
            }
        });

        #[derive(Debug, Deserialize)]
        struct RawOllamaResponse {
            response: String,
            done: bool,
        }

        let raw_response: RawOllamaResponse = self
            .client
            .post(url)
            .body(body.to_string())
            .send()?
            .json()
            .unwrap();

        let character_list: CharacterList = serde_json::from_str(&raw_response.response).unwrap();

        let result = CharacterListResult {
            done: raw_response.done,
            response: character_list,
        };

        println!("{:?}", result);
        Ok(result)
    }
}
