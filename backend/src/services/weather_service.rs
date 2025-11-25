use crate::models::WeatherForecast;
use log::{info, warn, error};
use chrono::Utc;

use super::providers::{
    fetch_open_meteo, 
    fetch_openweather, 
    fetch_weatherapi
};

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

    /// Get weather forecast with fallback strategy
    /// 
    /// Tries providers in order:
    /// 1. Open-Meteo (free, reliable)
    /// 2. OpenWeatherMap (requires API key)
    /// 3. WeatherAPI (requires API key)
    pub async fn get_forecast(
        &self,
        city: &str,
        lat: f64,
        lon: f64,
    ) -> Result<WeatherForecast, String> {
        info!("Getting weather forecast for city={}, lat={}, lon={}", city, lat, lon);

        // Try Open-Meteo first (free, no API key needed)
        info!("Attempting to fetch from Open-Meteo provider");
        match fetch_open_meteo(lat, lon).await {
            Ok(forecast) => {
                info!("Successfully retrieved forecast from Open-Meteo");
                return Ok(WeatherForecast {
                    city: city.to_string(),
                    province: "".to_string(),
                    country: "Indonesia".to_string(),
                    latitude: lat,
                    longitude: lon,
                    last_updated: Utc::now().to_rfc3339(),
                    forecast,
                });
            }
            Err(e) => {
                warn!("Open-Meteo provider failed: {:?}", e);
            }
        }

        // Try OpenWeatherMap if API key is available
        if !self.openweather_key.is_empty() && self.openweather_key != "your-key-here" {
            info!("Attempting to fetch from OpenWeatherMap provider");
            match fetch_openweather(lat, lon, &self.openweather_key).await {
                Ok(forecast) => {
                    info!("Successfully retrieved forecast from OpenWeatherMap");
                    return Ok(WeatherForecast {
                        city: city.to_string(),
                        province: "".to_string(),
                        country: "Indonesia".to_string(),
                        latitude: lat,
                        longitude: lon,
                        last_updated: Utc::now().to_rfc3339(),
                        forecast,
                    });
                }
                Err(e) => {
                    warn!("OpenWeatherMap provider failed: {:?}", e);
                }
            }
        } else {
            warn!("OpenWeatherMap API key not configured, skipping provider");
        }

        // Try WeatherAPI if API key is available
        if !self.weatherapi_key.is_empty() && self.weatherapi_key != "your-key-here" {
            info!("Attempting to fetch from WeatherAPI provider");
            match fetch_weatherapi(city, &self.weatherapi_key).await {
                Ok(forecast) => {
                    info!("Successfully retrieved forecast from WeatherAPI");
                    return Ok(WeatherForecast {
                        city: city.to_string(),
                        province: "".to_string(),
                        country: "Indonesia".to_string(),
                        latitude: lat,
                        longitude: lon,
                        last_updated: Utc::now().to_rfc3339(),
                        forecast,
                    });
                }
                Err(e) => {
                    warn!("WeatherAPI provider failed: {:?}", e);
                }
            }
        } else {
            warn!("WeatherAPI key not configured, skipping provider");
        }

        // All providers failed
        error!("All weather providers failed for city={}", city);
        Err(
            "Failed to fetch weather forecast from all available providers. \
             Please try again later or check your API configuration."
                .to_string(),
        )
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
