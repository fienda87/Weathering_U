use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub mod providers;
pub mod weather_service;
pub mod daily_processor;
pub mod parallel_forecast;

pub use weather_service::WeatherService;
pub use parallel_forecast::{fetch_forecast_parallel, fetch_forecast_with_rate_limit, TaskMetrics};

#[allow(dead_code)]
pub struct ApiService {
    client: Client,
    base_url: String,
}

#[allow(dead_code)]
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