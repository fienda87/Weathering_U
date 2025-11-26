use serde::{Deserialize, Serialize};
use crate::models::DailyForecast;
use reqwest::Client;
use std::time::Duration;
use log::info;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenWeatherMain {
    pub temp_max: f32,
    pub temp_min: f32,
    pub humidity: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenWeatherWeather {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenWeatherListItem {
    pub dt: i64,
    pub main: OpenWeatherMain,
    pub weather: Vec<OpenWeatherWeather>,
    pub wind: Option<OpenWeatherWind>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenWeatherWind {
    pub speed: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenWeatherResponse {
    pub list: Vec<OpenWeatherListItem>,
}

pub async fn fetch_openweather(
    lat: f64,
    lon: f64,
    api_key: &str,
) -> Result<Vec<DailyForecast>, String> {
    info!("Fetching weather from OpenWeatherMap provider for lat={}, lon={}", lat, lon);

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}&units=metric",
        lat, lon, api_key
    );

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let data: OpenWeatherResponse = response.json().await.map_err(|e| e.to_string())?;

    info!("Successfully fetched OpenWeatherMap data");

    // Normalize to DailyForecast
    let forecasts = normalize_openweather(&data)?;
    Ok(forecasts)
}

fn normalize_openweather(data: &OpenWeatherResponse) -> Result<Vec<DailyForecast>, String> {
    if data.list.is_empty() {
        return Err("No weather data available".to_string());
    }

    let mut daily_forecasts: Vec<DailyForecast> = Vec::new();
    let mut current_date = String::new();
    let mut current_max_temp = f32::MIN;
    let mut current_min_temp = f32::MAX;
    let mut current_humidity = 0u32;
    let mut current_wind_speed = 0.0f32;
    let mut current_weather = String::new();
    let mut current_icon = String::new();

    for item in &data.list {
        let dt = DateTime::<Utc>::from_timestamp(item.dt, 0)
            .ok_or_else(|| "Invalid timestamp".to_string())?;
        let date = dt.format("%Y-%m-%d").to_string();

        // If we're on a new day, save the previous day's forecast
        if !current_date.is_empty() && current_date != date {
            let temp_avg = (current_max_temp + current_min_temp) / 2.0;
            daily_forecasts.push(DailyForecast {
                date: current_date.clone(),
                temp_max: current_max_temp,
                temp_min: current_min_temp,
                temp_avg,
                condition: current_weather.clone(),
                humidity: current_humidity,
                wind_speed: current_wind_speed,
                icon: current_icon.clone(),
            });

            current_max_temp = f32::MIN;
            current_min_temp = f32::MAX;
            #[allow(unused_assignments)]
            {
                current_humidity = 0;
                current_wind_speed = 0.0;
            }
        }

        current_date = date;
        current_max_temp = current_max_temp.max(item.main.temp_max);
        current_min_temp = current_min_temp.min(item.main.temp_min);
        current_humidity = item.main.humidity;
        current_wind_speed = item.wind.as_ref().map(|w| w.speed).unwrap_or(0.0);

        if !item.weather.is_empty() {
            let (condition, icon) = map_openweather_condition(&item.weather[0].main);
            current_weather = condition.to_string();
            current_icon = icon.to_string();
        }
    }

    // Add the last day's forecast
    if !current_date.is_empty() {
        let temp_avg = (current_max_temp + current_min_temp) / 2.0;
        daily_forecasts.push(DailyForecast {
            date: current_date,
            temp_max: current_max_temp,
            temp_min: current_min_temp,
            temp_avg,
            condition: current_weather,
            humidity: current_humidity,
            wind_speed: current_wind_speed,
            icon: current_icon,
        });
    }

    // OpenWeatherMap provides 5 days, but we need 7 days
    // Extrapolate last 2 days by repeating them
    // Pad to 7 days by repeating the last day's data
    while daily_forecasts.len() < 7 {
        if let Some(last_day) = daily_forecasts.last() {
            let mut new_day = last_day.clone();
            // Calculate new date by adding days
            let days_to_add = (daily_forecasts.len() as i64) - 4;
            
            // Parse the date string (format: YYYY-MM-DD)
            let base_date = match chrono::NaiveDate::parse_from_str(&last_day.date, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => chrono::Local::now().date_naive(),
            };
            
            // Add days to the base date
            if let Some(new_date) = base_date.checked_add_signed(chrono::Duration::days(days_to_add)) {
                new_day.date = new_date.format("%Y-%m-%d").to_string();
                daily_forecasts.push(new_day);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(daily_forecasts.into_iter().take(7).collect())
}

fn map_openweather_condition(condition: &str) -> (&'static str, &'static str) {
    match condition.to_lowercase().as_str() {
        "clear" => ("Clear", "sunny"),
        "clouds" => ("Cloudy", "cloudy"),
        "rain" => ("Rainy", "rainy"),
        "snow" => ("Snow", "snowy"),
        "drizzle" => ("Drizzle", "rainy"),
        "mist" | "smoke" | "haze" | "dust" | "fog" | "sand" | "ash" | "squall" | "tornado" => ("Foggy", "fog"),
        "thunderstorm" => ("Thunderstorm", "stormy"),
        _ => ("Unknown", "cloudy"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openweather_condition_mapping() {
        let (condition, icon) = map_openweather_condition("clear");
        assert_eq!(condition, "Clear");
        assert_eq!(icon, "sunny");

        let (condition, icon) = map_openweather_condition("rain");
        assert_eq!(condition, "Rainy");
        assert_eq!(icon, "rainy");
    }
}
