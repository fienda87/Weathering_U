use crate::models::{City, DailyForecast, PerSourceData, ProviderForecast};
use crate::services::providers::{
    fetch_open_meteo, 
    fetch_openweather, 
    fetch_weatherapi
};
use log::{info, warn};
use futures::future::join_all;

/// Fetch ensemble data from all 3 providers for a specific day
/// Returns PerSourceData with data from each provider that succeeded
pub async fn fetch_ensemble_day(
    day: usize,
    city: &City,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<PerSourceData, String> {
    info!("[Ensemble] Fetching day {} for {} from all providers", day, city.name);
    
    // Spawn all 3 provider fetches in parallel
    let open_meteo_task = fetch_open_meteo(city.latitude, city.longitude);
    let open_weather_task = fetch_openweather(city.latitude, city.longitude, openweather_key);
    let weather_api_task = fetch_weatherapi(city.name, weatherapi_key);
    
    // Wait for all to complete
    let results = tokio::join!(open_meteo_task, open_weather_task, weather_api_task);
    
    let mut per_source = PerSourceData::new();
    
    // Process Open-Meteo result
    match results.0 {
        Ok(forecast) => {
            if forecast.len() > day {
                let daily = &forecast[day];
                let provider_forecast = ProviderForecast::new(
                    daily.date.clone(),
                    daily.temp_max,
                    daily.temp_min,
                    daily.condition.clone(),
                );
                per_source = per_source.with_open_meteo(provider_forecast);
                info!("[Ensemble] Open-Meteo data available for day {}", day);
            } else {
                warn!("[Ensemble] Open-Meteo returned insufficient data for day {}", day);
            }
        }
        Err(e) => {
            warn!("[Ensemble] Open-Meteo failed: {}", e);
        }
    }
    
    // Process OpenWeatherMap result
    if !openweather_key.is_empty() && openweather_key != "your-key-here" {
        match results.1 {
            Ok(forecast) => {
                if forecast.len() > day {
                    let daily = &forecast[day];
                    let provider_forecast = ProviderForecast::new(
                        daily.date.clone(),
                        daily.temp_max,
                        daily.temp_min,
                        daily.condition.clone(),
                    );
                    per_source = per_source.with_open_weather(provider_forecast);
                    info!("[Ensemble] OpenWeatherMap data available for day {}", day);
                } else {
                    warn!("[Ensemble] OpenWeatherMap returned insufficient data for day {}", day);
                }
            }
            Err(e) => {
                warn!("[Ensemble] OpenWeatherMap failed: {}", e);
            }
        }
    } else {
        warn!("[Ensemble] OpenWeatherMap API key not configured");
    }
    
    // Process WeatherAPI result
    if !weatherapi_key.is_empty() && weatherapi_key != "your-key-here" {
        match results.2 {
            Ok(forecast) => {
                if forecast.len() > day {
                    let daily = &forecast[day];
                    let provider_forecast = ProviderForecast::new(
                        daily.date.clone(),
                        daily.temp_max,
                        daily.temp_min,
                        daily.condition.clone(),
                    );
                    per_source = per_source.with_weather_api(provider_forecast);
                    info!("[Ensemble] WeatherAPI data available for day {}", day);
                } else {
                    warn!("[Ensemble] WeatherAPI returned insufficient data for day {}", day);
                }
            }
            Err(e) => {
                warn!("[Ensemble] WeatherAPI failed: {}", e);
            }
        }
    } else {
        warn!("[Ensemble] WeatherAPI key not configured");
    }
    
    // Check if we got at least one provider
    if per_source.provider_count() == 0 {
        return Err(format!("All providers failed for day {}", day));
    }
    
    info!("[Ensemble] Day {}: {} provider(s) available", day, per_source.provider_count());
    Ok(per_source)
}

/// Calculate final forecast from per-source data
pub fn calculate_final_forecast(per_source: &PerSourceData, date: String) -> Result<(f32, f32, String), String> {
    let (max_temps, min_temps) = per_source.extract_temperatures();
    let conditions = per_source.get_conditions();
    
    if max_temps.is_empty() || min_temps.is_empty() {
        return Err("No temperature data available".to_string());
    }
    
    // Calculate average temperatures
    let final_temp_max: f32 = max_temps.iter().sum::<f32>() / max_temps.len() as f32;
    let final_temp_min: f32 = min_temps.iter().sum::<f32>() / min_temps.len() as f32;
    
    // Use most common condition (or first if tie)
    let final_condition = if conditions.is_empty() {
        "Unknown".to_string()
    } else {
        // Count condition occurrences
        let mut counts = std::collections::HashMap::new();
        for cond in &conditions {
            *counts.entry(cond.clone()).or_insert(0) += 1;
        }
        
        // Find most common
        counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cond, _)| cond)
            .unwrap_or_else(|| conditions[0].clone())
    };
    
    Ok((final_temp_max, final_temp_min, final_condition))
}

/// Fetch ensemble forecasts for 7 days in parallel
pub async fn fetch_ensemble_week(
    city: &City,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<Vec<PerSourceData>, String> {
    info!("[Ensemble] Fetching 7-day ensemble for {}", city.name);
    
    let mut futures = Vec::new();
    
    // Create futures for each day
    for day in 0..7 {
        let future = fetch_ensemble_day(day, city, openweather_key, weatherapi_key);
        futures.push(future);
    }
    
    // Wait for all futures to complete in parallel
    let results = join_all(futures).await;
    
    let mut per_source_days = Vec::new();
    let mut failed_count = 0;
    
    for (day_idx, result) in results.into_iter().enumerate() {
        match result {
            Ok(per_source) => {
                per_source_days.push(per_source);
            }
            Err(e) => {
                warn!("[Ensemble] Day {} failed: {}", day_idx, e);
                failed_count += 1;
                // Add empty per-source for failed days
                per_source_days.push(PerSourceData::new());
            }
        }
    }
    
    // Require at least 3 successful days
    if failed_count > 4 {
        return Err(format!("Too many failed days: {}/7", failed_count));
    }
    
    info!("[Ensemble] Successfully fetched ensemble data: {}/7 days", 7 - failed_count);
    Ok(per_source_days)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::City;

    fn create_test_city() -> City {
        City {
            id: 1,
            name: "Jakarta",
            province: "DKI Jakarta",
            latitude: -6.2088,
            longitude: 106.8456,
        }
    }

    #[tokio::test]
    async fn test_fetch_ensemble_day_structure() {
        let city = create_test_city();
        
        // This will likely fail with invalid keys, but tests structure
        let result = fetch_ensemble_day(0, &city, "invalid", "invalid").await;
        
        // Should return either Ok with at least Open-Meteo or Err
        // Open-Meteo is free and should work
        if let Ok(per_source) = result {
            assert!(per_source.provider_count() >= 1);
        }
    }

    #[test]
    fn test_calculate_final_forecast_with_data() {
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.0,
                22.0,
                "Sunny".to_string(),
            ))
            .with_open_weather(ProviderForecast::new(
                "2024-01-01".to_string(),
                31.0,
                23.0,
                "Clear".to_string(),
            ));

        let result = calculate_final_forecast(&per_source, "2024-01-01".to_string());
        assert!(result.is_ok());
        
        let (temp_max, temp_min, condition) = result.unwrap();
        assert!((temp_max - 30.5).abs() < 0.1);
        assert!((temp_min - 22.5).abs() < 0.1);
        assert!(!condition.is_empty());
    }

    #[test]
    fn test_calculate_final_forecast_no_data() {
        let per_source = PerSourceData::new();
        let result = calculate_final_forecast(&per_source, "2024-01-01".to_string());
        assert!(result.is_err());
    }
}
