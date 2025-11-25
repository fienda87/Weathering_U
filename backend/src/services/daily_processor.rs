use crate::models::{City, DailyForecast};
use crate::services::providers::{
    fetch_open_meteo, 
    fetch_openweather, 
    fetch_weatherapi
};
use log::{info, warn, error};
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Process a single day's forecast with timeout and error handling
pub async fn process_day(
    day: usize,
    city: &City,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<DailyForecast, String> {
    let start_time = Instant::now();
    info!("Starting processing day {} for city: {} ({})", day, city.name, city.province);
    
    // 5 second timeout per day
    let process_result = timeout(Duration::from_secs(5), async {
        // Try Open-Meteo first (free, no API key needed)
        info!("Day {}: Attempting Open-Meteo provider", day);
        match fetch_open_meteo(city.latitude, city.longitude).await {
            Ok(mut forecast) => {
                if forecast.len() > day {
                    info!("Day {}: Successfully retrieved from Open-Meteo", day);
                    return Ok(forecast.swap_remove(day));
                }
                warn!("Day {}: Open-Meteo returned insufficient forecast data", day);
            }
            Err(e) => {
                warn!("Day {}: Open-Meteo provider failed: {:?}", day, e);
            }
        }

        // Try OpenWeatherMap if API key is available
        if !openweather_key.is_empty() && openweather_key != "your-key-here" {
            info!("Day {}: Attempting OpenWeatherMap provider", day);
            match fetch_openweather(city.latitude, city.longitude, openweather_key).await {
                Ok(mut forecast) => {
                    if forecast.len() > day {
                        info!("Day {}: Successfully retrieved from OpenWeatherMap", day);
                        return Ok(forecast.swap_remove(day));
                    }
                    warn!("Day {}: OpenWeatherMap returned insufficient forecast data", day);
                }
                Err(e) => {
                    warn!("Day {}: OpenWeatherMap provider failed: {:?}", day, e);
                }
            }
        } else {
            warn!("Day {}: OpenWeatherMap API key not configured", day);
        }

        // Try WeatherAPI if API key is available
        if !weatherapi_key.is_empty() && weatherapi_key != "your-key-here" {
            info!("Day {}: Attempting WeatherAPI provider", day);
            match fetch_weatherapi(city.name, weatherapi_key).await {
                Ok(mut forecast) => {
                    if forecast.len() > day {
                        info!("Day {}: Successfully retrieved from WeatherAPI", day);
                        return Ok(forecast.swap_remove(day));
                    }
                    warn!("Day {}: WeatherAPI returned insufficient forecast data", day);
                }
                Err(e) => {
                    warn!("Day {}: WeatherAPI provider failed: {:?}", day, e);
                }
            }
        } else {
            warn!("Day {}: WeatherAPI key not configured", day);
        }

        Err("All providers failed for this day".to_string())
    }).await;

    let elapsed = start_time.elapsed();
    
    match process_result {
        Ok(Ok(forecast)) => {
            info!("Day {} completed successfully in {:?}", day, elapsed);
            Ok(forecast)
        }
        Ok(Err(e)) => {
            error!("Day {} failed after {:?}: {}", day, elapsed, e);
            Err(e)
        }
        Err(_) => {
            error!("Day {} timed out after 5 seconds", day);
            Err("Day processing timed out".to_string())
        }
    }
}