use serde::{Deserialize, Serialize};
use crate::models::DailyForecast;
use std::error::Error;
use reqwest::Client;
use std::time::Duration;
use log::info;

// Macro untuk struktur API response
macro_rules! api_struct {
    ($name:ident { $($field:ident: $type:ty),+ $(,)? }) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        pub struct $name {
            $(pub $field: $type),+
        }
    };
}

// Struktur WeatherAPI response
api_struct!(WeatherAPICondition {
    text: String,
    icon: String,
});

api_struct!(WeatherAPIDay {
    maxtemp_c: f32,
    mintemp_c: f32,
    avgtemp_c: f32,
    avghumidity: u32,
    condition: WeatherAPICondition,
});

api_struct!(WeatherAPIForecastDay {
    date: String,
    day: WeatherAPIDay,
});

api_struct!(WeatherAPIForecast {
    forecastday: Vec<WeatherAPIForecastDay>,
});

api_struct!(WeatherAPIResponse {
    forecast: WeatherAPIForecast,
});

pub async fn fetch_weatherapi(
    city: &str,
    api_key: &str,
) -> Result<Vec<DailyForecast>, Box<dyn Error + Send + Sync>> {
    info!("Fetching weather from WeatherAPI provider for city={}", city);

    // Increased timeout untuk koneksi yang lebih stabil (15s total, 8s connect)
    let client = Client::builder()
        .timeout(Duration::from_secs(15))        // Total request timeout
        .connect_timeout(Duration::from_secs(8)) // Connection timeout
        .pool_idle_timeout(Duration::from_secs(90)) // Keep connections alive
        .pool_max_idle_per_host(2)               // Connection pooling
        .build()?;

    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7&aqi=no",
        api_key, city
    );

    // Retry logic: coba 3x dengan exponential backoff
    let mut last_error = None;
    for attempt in 1..=3 {
        match client.get(&url).send().await {
            Ok(response) => {
                // Check HTTP status
                if !response.status().is_success() {
                    let status = response.status();
                    last_error = Some(format!("HTTP error: {}", status));
                    if attempt < 3 {
                        let delay = Duration::from_millis(500 * attempt as u64); // Exponential backoff
                        tokio::time::sleep(delay).await;
                    }
                    continue;
                }

                match response.json::<WeatherAPIResponse>().await {
                    Ok(data) => {
                        info!("Successfully fetched WeatherAPI data for {} (attempt {})", city, attempt);
                        let forecasts = normalize_weatherapi(&data)?;
                        return Ok(forecasts);
                    }
                    Err(e) => {
                        last_error = Some(format!("JSON parse error: {}", e));
                        if attempt < 3 {
                            let delay = Duration::from_millis(500 * attempt as u64);
                            tokio::time::sleep(delay).await;
                        }
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("{}", e);
                last_error = Some(error_msg.clone());
                
                // Log error type untuk debugging
                if error_msg.contains("deadline has elapsed") || error_msg.contains("timed out") {
                    info!("WeatherAPI timeout for {} (attempt {}), retrying...", city, attempt);
                } else if error_msg.contains("connect") {
                    info!("WeatherAPI connection error for {} (attempt {}), retrying...", city, attempt);
                } else {
                    info!("WeatherAPI error for {} (attempt {}): {}", city, attempt, error_msg);
                }
                
                if attempt < 3 {
                    let delay = Duration::from_millis(500 * attempt as u64); // Exponential backoff: 500ms, 1s, skip
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Unknown error after 3 attempts".to_string()).into())
}

fn normalize_weatherapi(data: &WeatherAPIResponse) -> Result<Vec<DailyForecast>, Box<dyn Error + Send + Sync>> {
    // Functional approach: map iterator tanpa mutable variable
    let forecasts = data.forecast.forecastday
        .iter()
        .map(|forecast_day| {
            let condition = normalize_weatherapi_condition(&forecast_day.day.condition.text);
            let icon = map_weatherapi_icon(&forecast_day.day.condition.icon);

            DailyForecast {
                date: forecast_day.date.clone(),
                temp_max: forecast_day.day.maxtemp_c,
                temp_min: forecast_day.day.mintemp_c,
                temp_avg: forecast_day.day.avgtemp_c,
                condition,
                humidity: forecast_day.day.avghumidity,
                wind_speed: 0.0,
                icon,
            }
        })
        .collect();

    Ok(forecasts)
}

fn normalize_weatherapi_condition(condition: &str) -> String {
    // Normalisasi ke kategori standar untuk konsistensi ensemble
    match condition {
        c if c.contains("Thunder") || c.contains("Storm") => "Thunderstorm".to_string(),
        c if c.contains("Snow") || c.contains("Blizzard") => "Snow".to_string(),
        c if c.contains("Rain") || c.contains("Drizzle") || c.contains("Shower") => "Rainy".to_string(),
        c if c.contains("Fog") || c.contains("Mist") => "Foggy".to_string(),
        c if c.contains("Cloud") || c.contains("Overcast") => "Cloudy".to_string(),
        c if c.contains("Clear") || c.contains("Sunny") => "Clear".to_string(),
        c if c.contains("Partly") => "Cloudy".to_string(),  // Partly cloudy → Cloudy
        _ => "Clear".to_string(),  // Default ke Clear
    }
}

fn map_weatherapi_icon(icon_url: &str) -> String {
    // WeatherAPI return URL kayak https://cdn.weatherapi.com/weather/128x128/day/113.png
    // Extract kode icon dan map ke icon kita
    if icon_url.contains("day") || icon_url.contains("night") {
        if icon_url.contains("113") || icon_url.contains("122") {
            "sunny".to_string()
        } else if icon_url.contains("116") || icon_url.contains("119") || icon_url.contains("122") {
            "cloudy".to_string()
        } else if icon_url.contains("143") || icon_url.contains("248") || icon_url.contains("260") {
            "fog".to_string()
        } else if icon_url.contains("176") || icon_url.contains("179") || icon_url.contains("182") || icon_url.contains("185") || icon_url.contains("200") || icon_url.contains("227") || icon_url.contains("230") || icon_url.contains("233") || icon_url.contains("266") || icon_url.contains("281") || icon_url.contains("284") || icon_url.contains("293") || icon_url.contains("296") || icon_url.contains("299") || icon_url.contains("302") || icon_url.contains("305") || icon_url.contains("308") || icon_url.contains("311") || icon_url.contains("314") || icon_url.contains("317") || icon_url.contains("320") || icon_url.contains("323") || icon_url.contains("326") || icon_url.contains("329") || icon_url.contains("332") || icon_url.contains("335") || icon_url.contains("338") || icon_url.contains("350") || icon_url.contains("353") || icon_url.contains("356") || icon_url.contains("359") || icon_url.contains("362") || icon_url.contains("365") || icon_url.contains("368") || icon_url.contains("371") || icon_url.contains("374") || icon_url.contains("377") || icon_url.contains("380") || icon_url.contains("386") || icon_url.contains("389") || icon_url.contains("392") || icon_url.contains("395") {
            if icon_url.contains("snow") || icon_url.contains("sleet") || icon_url.contains("179") || icon_url.contains("182") || icon_url.contains("185") || icon_url.contains("227") || icon_url.contains("230") || icon_url.contains("233") || icon_url.contains("320") || icon_url.contains("323") || icon_url.contains("326") || icon_url.contains("329") || icon_url.contains("332") || icon_url.contains("335") || icon_url.contains("368") || icon_url.contains("371") || icon_url.contains("374") || icon_url.contains("377") {
                "snowy".to_string()
            } else {
                "rainy".to_string()
            }
        } else if icon_url.contains("386") || icon_url.contains("389") || icon_url.contains("392") || icon_url.contains("395") {
            "stormy".to_string()
        } else {
            "cloudy".to_string()
        }
    } else {
        "cloudy".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weatherapi_condition_mapping() {
        assert_eq!(normalize_weatherapi_condition("Sunny"), "Clear");
        assert_eq!(normalize_weatherapi_condition("Rainy"), "Rainy");
        assert_eq!(normalize_weatherapi_condition("Snowy"), "Snowy");
    }
}
