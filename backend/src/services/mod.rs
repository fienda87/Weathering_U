use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub mod providers;
pub mod weather_service;

pub use weather_service::WeatherService;

pub struct ApiService {
    client: Client,
    base_url: String,
}

impl ApiService {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn post(&self, endpoint: &str, body: Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(&body).send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
}