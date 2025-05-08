use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct GeminiAdapter {
    client: Client,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Character {
    pub character_name: String,
    pub character_description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterList {
    pub characters: Vec<Character>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Part {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

impl GeminiResponse {
    pub fn get_text(&self) -> String {
        self.candidates[0].content.parts[0].text.clone()
    }
}

impl GeminiAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn summarize_show(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let params: Vec<&str> = query.split('&').collect();
        let (show_id, season, episode) = (
            params[0].split('=').collect::<Vec<&str>>()[1],
            params[1].split('=').collect::<Vec<&str>>()[1],
            params[2].split('=').collect::<Vec<&str>>()[1],
        );

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-preview-04-17:generateContent?key={}", std::env::var("GEMINI_API_KEY").unwrap());

        let prompt = format!("You are a critic for TV shows, who has watched every show ever written. People come to you when they want to catch up to a TV show. You are given the name of the show, as well as the season and episode that the person will watch next. You should give them a detailed summary of what happened until that point. You will only summarize facts, and you will include every major event in your summary. You will not include any opinions or recommandations. Your answer will only include the summary of the show: you will not write anything in the first person, nor will you wish a 'happy viewing' or anything along these lines. Your answer will not have any conversational filler, only a factual summary of the show up to the mentioned episode. You will start the summary with a quick explanation of when and where the show is set, then move to what happens in the show. You will avoid spoilers at all costs, or you will lose your job. For instance, if asked to summarize a show up to season 2 episode 2, you should summarize what happened in season 1 and in the first episode of season 2, but no further. Now summarize the show {} up to season {} episode {}", show_id, season, episode);

        let body = json!({
            "contents": [{ "parts": [ { "text": prompt } ] }]
        });

        let response: GeminiResponse = self
            .client
            .post(url)
            .body(body.to_string())
            .send()
            .unwrap()
            .json()?;

        println!("{:?}", response);

        Ok(response.get_text())
    }

    pub fn describe_cast(
        &self,
        mut characters: Vec<String>,
    ) -> Result<CharacterList, Box<dyn std::error::Error>> {
        if characters.len() > 10 {
            characters.truncate(10);
        }

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-preview-04-17:generateContent?key={}", std::env::var("GEMINI_API_KEY").unwrap());

        let prompt = format!("You are a critic for TV shows, who has watched every show ever written. Given a list of characters from a TV show, your job is to provide a short description of a character and their role in the story. For example, if you are asked about Joey Tribbiani, your answer could be: 'Chandler's roommate, great with women, trying to make it as an actor'. The characters you have to describe are the following: {:?}", characters);

        let body = json!({
               "contents": [{ "parts": [ { "text": prompt } ] }],
                "generationConfig": {
            "response_mime_type": "application/json",
            "response_schema": {
                "properties": {
                    "characters": {
                        "items": {
                            "type": "object",
                            "properties": {
                                "character_description": {
                                    "type": "string"
                                },
                                "character_name": {
                                    "type": "string"
                                }
                            },
                            "required": ["character_name", "character_description"]
                        },
                        "type": "ARRAY"
                    }
                },
                "type": "OBJECT"
            }
        }
               });

        println!("Body: {}", body);

        let raw_response: GeminiResponse = self
            .client
            .post(url)
            .body(body.to_string())
            .send()?
            .json()
            .unwrap();

        println!("{:?}", raw_response);

        let character_list: CharacterList = serde_json::from_str(&raw_response.get_text()).unwrap();

        println!("{:?}", character_list);
        Ok(character_list)
    }
}
