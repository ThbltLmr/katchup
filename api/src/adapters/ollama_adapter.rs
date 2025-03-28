use reqwest::{blocking::Client, header::ACCEPT, Method};

pub struct OllamaAdapter {
    client: Client,
}

impl OllamaAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}
