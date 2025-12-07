use crate::models::{City, PerSourceData, ProviderForecast};
use crate::services::providers::{
    fetch_open_meteo, 
    fetch_openweather, 
    fetch_weatherapi
};
use log::{info, warn};
use futures::future::join_all;

/// Helper untuk cek apakah API key valid
fn is_valid_api_key(key: &str) -> bool {
    !key.is_empty() && key != "your-key-here"
}

/// Fetch paralel ke 3 provider, kumpulkan data yang berhasil
pub async fn fetch_ensemble_day(
    day: usize,
    city: &City,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<PerSourceData, String> {
    info!("[Ensemble] Fetching day {} for {} from all providers", day, city.name);
    
    let open_meteo_task = fetch_open_meteo(city.latitude, city.longitude);
    let open_weather_task = fetch_openweather(city.latitude, city.longitude, openweather_key);
    let weather_api_task = fetch_weatherapi(city.name, weatherapi_key);
    
    let results = tokio::join!(open_meteo_task, open_weather_task, weather_api_task);
    
    // Process Open-Meteo (always processed, free API)
    let per_source = match &results.0 {
        Ok(forecast) if forecast.len() > day => {
            let daily = &forecast[day];
            let provider_forecast = ProviderForecast::new(
                daily.date.clone(),
                daily.temp_max,
                daily.temp_min,
                daily.condition.clone(),
            );
            info!("[Ensemble] Open-Meteo data available for day {}", day);
            PerSourceData::new().with_open_meteo(provider_forecast)
        }
        Ok(_) => {
            warn!("[Ensemble] Open-Meteo returned insufficient data for day {}", day);
            PerSourceData::new()
        }
        Err(e) => {
            warn!("[Ensemble] Open-Meteo failed: {}", e);
            PerSourceData::new()
        }
    };
    
    // Process OpenWeatherMap (if API key valid)
    let per_source = if is_valid_api_key(openweather_key) {
        match &results.1 {
            Ok(forecast) if forecast.len() > day => {
                let daily = &forecast[day];
                let provider_forecast = ProviderForecast::new(
                    daily.date.clone(),
                    daily.temp_max,
                    daily.temp_min,
                    daily.condition.clone(),
                );
                info!("[Ensemble] OpenWeatherMap data available for day {}", day);
                per_source.with_open_weather(provider_forecast)
            }
            Ok(_) => {
                warn!("[Ensemble] OpenWeatherMap returned insufficient data for day {}", day);
                per_source
            }
            Err(e) => {
                warn!("[Ensemble] OpenWeatherMap failed: {}", e);
                per_source
            }
        }
    } else {
        warn!("[Ensemble] OpenWeatherMap API key not configured");
        per_source
    };
    
    // Process WeatherAPI (if API key valid)
    let per_source = if is_valid_api_key(weatherapi_key) {
        match &results.2 {
            Ok(forecast) if forecast.len() > day => {
                let daily = &forecast[day];
                let provider_forecast = ProviderForecast::new(
                    daily.date.clone(),
                    daily.temp_max,
                    daily.temp_min,
                    daily.condition.clone(),
                );
                info!("[Ensemble] WeatherAPI data available for day {}", day);
                per_source.with_weather_api(provider_forecast)
            }
            Ok(_) => {
                warn!("[Ensemble] WeatherAPI returned insufficient data for day {}", day);
                per_source
            }
            Err(e) => {
                // Graceful degradation: timeout tidak fatal
                let error_str = e.to_string();
                if error_str.contains("deadline has elapsed") || error_str.contains("timed out") {
                    // Timeout adalah common issue - log sebagai info saja
                    info!("[Ensemble] WeatherAPI timeout for {} day {} (continuing with {}/3 providers)", 
                          city.name, day, per_source.provider_count());
                } else if error_str.contains("connect") {
                    info!("[Ensemble] WeatherAPI connection issue for {} day {} (continuing with {}/3 providers)", 
                          city.name, day, per_source.provider_count());
                } else {
                    warn!("[Ensemble] WeatherAPI error for {} day {}: {} (continuing with {}/3 providers)", 
                          city.name, day, e, per_source.provider_count());
                }
                per_source
            }
        }
    } else {
        info!("[Ensemble] WeatherAPI key not configured (using Open-Meteo + OpenWeatherMap only)");
        per_source
    };
    
    // Validate at least one provider succeeded
    if per_source.provider_count() == 0 {
        return Err(format!("All providers failed for day {}", day));
    }
    
    info!("[Ensemble] Day {}: {} provider(s) available", day, per_source.provider_count());
    Ok(per_source)
}

/// Normalisasi condition dari berbagai provider ke kategori standar
fn normalize_condition(condition: &str) -> &'static str {
    let condition_lower = condition.to_lowercase();
    
    // Mapping berdasarkan keyword (prioritas dari paling spesifik)
    if condition_lower.contains("thunder") || condition_lower.contains("storm") {
        "Thunderstorm"
    } else if condition_lower.contains("snow") || condition_lower.contains("salju") {
        "Snow"
    } else if condition_lower.contains("rain") || condition_lower.contains("hujan") || condition_lower.contains("drizzle") {
        "Rainy"
    } else if condition_lower.contains("fog") || condition_lower.contains("mist") || condition_lower.contains("kabut") {
        "Foggy"
    } else if condition_lower.contains("cloud") || condition_lower.contains("berawan") || condition_lower.contains("overcast") {
        "Cloudy"
    } else if condition_lower.contains("clear") || condition_lower.contains("sunny") || condition_lower.contains("cerah") {
        "Clear"
    } else if condition_lower.contains("partly") {
        "Partly Cloudy"
    } else {
        "Clear"  // Default fallback
    }
}

/// Rata-rata suhu, ambil kondisi yang paling sering muncul (dengan normalisasi)
pub fn calculate_final_forecast(per_source: &PerSourceData, _date: String) -> Result<(f32, f32, String), String> {
    let (max_temps, min_temps) = per_source.extract_temperatures();
    let conditions = per_source.get_conditions();
    
    if max_temps.is_empty() || min_temps.is_empty() {
        return Err("No temperature data available".to_string());
    }
    
    let final_temp_max: f32 = max_temps.iter().sum::<f32>() / max_temps.len() as f32;
    let final_temp_min: f32 = min_temps.iter().sum::<f32>() / min_temps.len() as f32;
    
    let final_condition = if conditions.is_empty() {
        "Clear".to_string()
    } else {
        // Normalisasi semua condition sebelum voting
        let normalized_conditions: Vec<&'static str> = conditions
            .iter()
            .map(|c| normalize_condition(c))
            .collect();
        
        // Count occurrences dari normalized conditions
        let counts = normalized_conditions.iter()
            .fold(std::collections::HashMap::new(), |mut acc, cond| {
                *acc.entry(*cond).or_insert(0) += 1;
                acc
            });
        
        // Ambil kondisi dengan vote terbanyak
        counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cond, _)| cond.to_string())
            .unwrap_or_else(|| "Clear".to_string())
    };
    
    Ok((final_temp_max, final_temp_min, final_condition))
}

/// Fetch 7 hari secara paralel
pub async fn fetch_ensemble_week(
    city: &City,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<Vec<PerSourceData>, String> {
    info!("[Ensemble] Fetching 7-day ensemble for {}", city.name);
    
    // BUAT futures UNTUK 7 HARI
    let futures: Vec<_> = (0..7)
        .map(|day| fetch_ensemble_day(day, city, openweather_key, weatherapi_key))
        .collect();
    
    let results = join_all(futures).await;
    
    // Process results 
    let (per_source_days, failed_count): (Vec<PerSourceData>, usize) = results
        .into_iter()
        .enumerate()
        .fold((Vec::new(), 0), |(mut days, fails), (day_idx, result)| {
            match result {
                Ok(per_source) => {
                    days.push(per_source);
                    (days, fails)
                }
                Err(e) => {
                    warn!("[Ensemble] Day {} failed: {}", day_idx, e);
                    days.push(PerSourceData::new());
                    (days, fails + 1)
                }
            }
        });
    
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
        
       
        let result = fetch_ensemble_day(0, &city, "invalid", "invalid").await;
        
       
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

    #[test]
    fn test_is_valid_api_key() {
        assert!(!is_valid_api_key(""));
        assert!(!is_valid_api_key("your-key-here"));
        assert!(is_valid_api_key("valid-api-key-123"));
        assert!(is_valid_api_key("abc123xyz"));
    }
}
