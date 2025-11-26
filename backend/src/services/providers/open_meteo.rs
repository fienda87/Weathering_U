use serde::{Deserialize, Serialize};
use crate::models::DailyForecast;
use std::error::Error;
use reqwest::Client;
use std::time::Duration;
use log::info;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenMeteoDaily {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub relative_humidity_2m_mean: Vec<u32>,
    pub weather_code: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenMeteoResponse {
    pub daily: OpenMeteoDaily,
}

pub async fn fetch_open_meteo(
    lat: f64,
    lon: f64,
) -> Result<Vec<DailyForecast>, Box<dyn Error + Send + Sync>> {
    info!("Fetching weather from Open-Meteo provider for lat={}, lon={}", lat, lon);

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,relative_humidity_2m_mean,weather_code&timezone=Asia/Jakarta",
        lat, lon
    );

    let response = client.get(&url).send().await?;
    let data: OpenMeteoResponse = response.json().await?;

    info!("Successfully fetched Open-Meteo data");

    // Normalize to DailyForecast
    let forecasts = normalize_open_meteo(&data)?;
    Ok(forecasts)
}

fn normalize_open_meteo(data: &OpenMeteoResponse) -> Result<Vec<DailyForecast>, Box<dyn Error + Send + Sync>> {
    let mut forecasts = Vec::new();
    let daily = &data.daily;

    // Take only first 7 days
    let days_count = std::cmp::min(7, daily.time.len());

    for i in 0..days_count {
        let temp_max = daily.temperature_2m_max[i];
        let temp_min = daily.temperature_2m_min[i];
        let temp_avg = (temp_max + temp_min) / 2.0;
        let humidity = daily.relative_humidity_2m_mean[i];
        let weather_code = daily.weather_code[i];

        let (condition, icon) = map_wmo_code(weather_code);

        forecasts.push(DailyForecast {
            date: daily.time[i].clone(),
            temp_max,
            temp_min,
            temp_avg,
            condition,
            humidity,
            wind_speed: 0.0, // Open-Meteo free tier doesn't provide wind_speed in daily forecast
            icon,
        });
    }

    Ok(forecasts)
}

/// Maps WMO weather codes to readable conditions and icons
fn map_wmo_code(code: i32) -> (String, String) {
    let (condition, icon) = match code {
        0 => ("Clear sky", "sunny"),
        1 | 2 => ("Mostly clear", "sunny"),
        3 => ("Overcast", "cloudy"),
        45 | 48 => ("Foggy", "fog"),
        51 | 53 | 55 => ("Light drizzle", "rainy"),
        61 | 63 | 65 => ("Rain", "rainy"),
        71 | 73 | 75 => ("Snow", "snowy"),
        77 => ("Snow grains", "snowy"),
        80 | 82 | 81 => ("Rain showers", "rainy"),
        85 | 86 => ("Snow showers", "snowy"),
        95 | 96 | 99 => ("Thunderstorm", "stormy"),
        _ => ("Unknown", "cloudy"),
    };
    (condition.to_string(), icon.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wmo_code_mapping() {
        let (condition, icon) = map_wmo_code(0);
        assert_eq!(condition, "Clear sky");
        assert_eq!(icon, "sunny");

        let (condition, icon) = map_wmo_code(61);
        assert_eq!(condition, "Rain");
        assert_eq!(icon, "rainy");

        let (condition, icon) = map_wmo_code(95);
        assert_eq!(condition, "Thunderstorm");
        assert_eq!(icon, "stormy");
    }
}
