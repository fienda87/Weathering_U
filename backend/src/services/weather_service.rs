use crate::models::{WeatherForecast, City};
use log::info;
use std::sync::Arc;
use tokio::sync::Semaphore;

use super::parallel_forecast::{fetch_forecast_parallel, fetch_forecast_with_rate_limit};

pub struct WeatherService {
    pub openweather_key: String,
    pub weatherapi_key: String,
}

impl WeatherService {
    pub fn new(openweather_key: String, weatherapi_key: String) -> Self {
        Self {
            openweather_key,
            weatherapi_key,
        }
    }

    /// Pemrosesan paralel: 7 task sekaligus untuk 7 hari
    pub async fn get_forecast_parallel(
        &self,
        city: &City,
    ) -> Result<WeatherForecast, String> {
        info!("Getting parallel weather forecast for city={}, lat={}, lon={}", 
              city.name, city.latitude, city.longitude);

        fetch_forecast_parallel(city, &self.openweather_key, &self.weatherapi_key).await
    }

    /// Rate limiting pakai semaphore: batasi concurrent API calls
    pub async fn get_forecast_rate_limited(
        &self,
        city: &City,
        semaphore: Arc<Semaphore>,
    ) -> Result<WeatherForecast, String> {
        info!("Getting rate-limited weather forecast for city={}, lat={}, lon={}", 
              city.name, city.latitude, city.longitude);

        fetch_forecast_with_rate_limit(city, semaphore, &self.openweather_key, &self.weatherapi_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_weather_service_creation() {
        let service = WeatherService::new(
            "test-key".to_string(),
            "test-key".to_string(),
        );
        assert!(!service.openweather_key.is_empty());
        assert!(!service.weatherapi_key.is_empty());
    }
}
