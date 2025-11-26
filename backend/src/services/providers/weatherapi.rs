use serde::{Deserialize, Serialize};
use crate::models::DailyForecast;
use reqwest::Client;
use std::time::Duration;
use log::info;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherAPIDay {
    pub maxtemp_c: f32,
    pub mintemp_c: f32,
    pub avgtemp_c: f32,
    pub avghumidity: u32,
    pub condition: WeatherAPICondition,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherAPICondition {
    pub text: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherAPIForecastDay {
    pub date: String,
    pub day: WeatherAPIDay,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherAPIForecast {
    pub forecastday: Vec<WeatherAPIForecastDay>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherAPIResponse {
    pub forecast: WeatherAPIForecast,
}

pub async fn fetch_weatherapi(
    city: &str,
    api_key: &str,
) -> Result<Vec<DailyForecast>, String> {
    info!("Fetching weather from WeatherAPI provider for city={}", city);

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7&aqi=no",
        api_key, city
    );

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let data: WeatherAPIResponse = response.json().await.map_err(|e| e.to_string())?;

    info!("Successfully fetched WeatherAPI data");

    // Normalize to DailyForecast
    let forecasts = normalize_weatherapi(&data)?;
    Ok(forecasts)
}

fn normalize_weatherapi(data: &WeatherAPIResponse) -> Result<Vec<DailyForecast>, String> {
    let mut forecasts = Vec::new();

    for forecast_day in &data.forecast.forecastday {
        let condition = normalize_weatherapi_condition(&forecast_day.day.condition.text);
        let icon = map_weatherapi_icon(&forecast_day.day.condition.icon);

        forecasts.push(DailyForecast {
            date: forecast_day.date.clone(),
            temp_max: forecast_day.day.maxtemp_c,
            temp_min: forecast_day.day.mintemp_c,
            temp_avg: forecast_day.day.avgtemp_c,
            condition,
            humidity: forecast_day.day.avghumidity,
            wind_speed: 0.0, // WeatherAPI provides this, but for simplicity we'll keep it 0
            icon,
        });
    }

    Ok(forecasts)
}

fn normalize_weatherapi_condition(condition: &str) -> String {
    match condition {
        c if c.contains("Clear") || c.contains("Sunny") => "Clear".to_string(),
        c if c.contains("Cloud") => "Cloudy".to_string(),
        c if c.contains("Rain") => "Rainy".to_string(),
        c if c.contains("Snow") => "Snowy".to_string(),
        c if c.contains("Sleet") => "Sleet".to_string(),
        c if c.contains("Fog") => "Foggy".to_string(),
        c if c.contains("Thunder") => "Thunderstorm".to_string(),
        c if c.contains("Mist") => "Misty".to_string(),
        _ => condition.to_string(),
    }
}

fn map_weatherapi_icon(icon_url: &str) -> String {
    // WeatherAPI returns URLs like https://cdn.weatherapi.com/weather/128x128/day/113.png
    // Extract the icon code and map to our icons
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
