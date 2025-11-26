use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub mod cache;
pub mod providers;
pub mod weather_service;
pub mod daily_processor;
pub mod parallel_forecast;
pub mod ensemble;
pub mod ensemble_fetcher;
pub mod ensemble_nextweek;
pub mod ensemble_orchestrator;
pub mod confidence_calculator;
pub mod city_service;

pub use cache::{ForecastCache, CachedEntry, CacheStats};
pub use weather_service::WeatherService;
pub use parallel_forecast::{fetch_forecast_parallel, fetch_forecast_with_rate_limit, TaskMetrics};
pub use ensemble_nextweek::{fetch_next_week_ensemble, fetch_ensemble_day_offset};
pub use ensemble_orchestrator::EnsembleOrchestrator;
pub use confidence_calculator::calculate_confidence;
pub use city_service::{find_city, validate_city_input, get_all_cities, get_city_count};

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