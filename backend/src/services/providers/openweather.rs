use serde::{Deserialize, Serialize};
use crate::models::DailyForecast;
use std::error::Error;
use reqwest::Client;
use std::time::Duration;
use log::info;
use chrono::{DateTime, Utc};

macro_rules! api_struct {
    ($name:ident { $($field:ident: $type:ty),+ $(,)? }) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        pub struct $name {
            $(pub $field: $type),+
        }
    };
}

// OpenWeatherMap macro
api_struct!(OpenWeatherMain {
    temp_max: f32,
    temp_min: f32,
    humidity: u32,
});

api_struct!(OpenWeatherWeather {
    main: String,
    description: String,
});

api_struct!(OpenWeatherWind {
    speed: f32,
});

api_struct!(OpenWeatherListItem {
    dt: i64,
    main: OpenWeatherMain,
    weather: Vec<OpenWeatherWeather>,
    wind: Option<OpenWeatherWind>,
});

api_struct!(OpenWeatherResponse {
    list: Vec<OpenWeatherListItem>,
});

pub async fn fetch_openweather(
    lat: f64,
    lon: f64,
    api_key: &str,
) -> Result<Vec<DailyForecast>, Box<dyn Error + Send + Sync>> {
    info!("Fetching weather from OpenWeatherMap provider for lat={}, lon={}", lat, lon);

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}&units=metric",
        lat, lon, api_key
    );

    let response = client.get(&url).send().await?;
    let data: OpenWeatherResponse = response.json().await?;

    info!("Successfully fetched OpenWeatherMap data");

    let forecasts = normalize_openweather(&data)?;
    Ok(forecasts)
}

fn normalize_openweather(data: &OpenWeatherResponse) -> Result<Vec<DailyForecast>, Box<dyn Error + Send + Sync>> {
    use std::collections::HashMap;
    
    if data.list.is_empty() {
        return Err("No weather data available".into());
    }

    // Group items by date using fold 
    let grouped_by_date: HashMap<String, Vec<&OpenWeatherListItem>> = data.list.iter()
        .fold(HashMap::new(), |mut acc, item| {
            if let Some(dt) = DateTime::<Utc>::from_timestamp(item.dt, 0) {
                let date = dt.format("%Y-%m-%d").to_string();
                acc.entry(date).or_insert_with(Vec::new).push(item);
            }
            acc
        });

    // Konversi data yang sudah dikelompokkan menjadi forecast harian
    let daily_forecasts: Vec<DailyForecast> = grouped_by_date
        .into_iter()
        .map(|(date, items)| {
            // Agregasi data untuk sehari menggunakan fold (immutable accumulation)
            let (max_temp, min_temp, last_humidity, last_wind, last_condition, last_icon) = items.iter()
                .fold(
                    (f32::MIN, f32::MAX, 0u32, 0.0f32, String::new(), String::new()),
                    |(max_t, min_t, _, _, _, _), item| {
                        let humidity = item.main.humidity;
                        let wind = item.wind.as_ref().map(|w| w.speed).unwrap_or(0.0);
                        let (condition, icon) = if !item.weather.is_empty() {
                            let (c, i) = map_openweather_condition(&item.weather[0].main);
                            (c.to_string(), i.to_string())
                        } else {
                            (String::new(), String::new())
                        };
                        
                        (
                            max_t.max(item.main.temp_max),
                            min_t.min(item.main.temp_min),
                            humidity,
                            wind,
                            condition,
                            icon,
                        )
                    }
                );

            let temp_avg = (max_temp + min_temp) / 2.0;

            DailyForecast {
                date,
                temp_max: max_temp,
                temp_min: min_temp,
                temp_avg,
                condition: last_condition,
                humidity: last_humidity,
                wind_speed: last_wind,
                icon: last_icon,
            }
        })
        .collect();

    // Urutkan berdasarkan tanggal (immutable sort dengan sorted iterator)
    let mut daily_forecasts = daily_forecasts;
    daily_forecasts.sort_by(|a, b| a.date.cmp(&b.date));

    // Perpanjang sampai 7 hari jika diperlukan (functional approach)
    let daily_forecasts = if daily_forecasts.len() < 7 {
        let additional_days: Vec<DailyForecast> = (daily_forecasts.len()..7)
            .filter_map(|i| {
                daily_forecasts.last().and_then(|last_day| {
                    let days_to_add = (i as i64) - 4;
                    chrono::NaiveDate::parse_from_str(&last_day.date, "%Y-%m-%d")
                        .ok()
                        .and_then(|base_date| base_date.checked_add_signed(chrono::Duration::days(days_to_add)))
                        .map(|new_date| {
                            // Buat forecast baru dengan tanggal yang diperbarui (immutable update)
                            DailyForecast {
                                date: new_date.format("%Y-%m-%d").to_string(),
                                temp_max: last_day.temp_max,
                                temp_min: last_day.temp_min,
                                temp_avg: last_day.temp_avg,
                                condition: last_day.condition.clone(),
                                humidity: last_day.humidity,
                                wind_speed: last_day.wind_speed,
                                icon: last_day.icon.clone(),
                            }
                        })
                })
            })
            .collect();

        // Gabungkan forecast asli dengan hari tambahan
        daily_forecasts.into_iter()
            .chain(additional_days)
            .take(7)
            .collect()
    } else {
        daily_forecasts.into_iter().take(7).collect()
    };

    Ok(daily_forecasts)
}

fn map_openweather_condition(condition: &str) -> (&'static str, &'static str) {
    match condition.to_lowercase().as_str() {
        "clear" => ("Clear", "sunny"),
        "clouds" => ("Cloudy", "cloudy"),
        "rain" => ("Rainy", "rainy"),
        "snow" => ("Snow", "snowy"),
        "drizzle" => ("Rainy", "rainy"),  // Drizzle = Rainy untuk konsistensi
        "mist" | "smoke" | "haze" | "dust" | "fog" | "sand" | "ash" | "squall" | "tornado" => ("Foggy", "fog"),
        "thunderstorm" => ("Thunderstorm", "stormy"),
        _ => ("Clear", "sunny"),  // Default ke Clear (bukan Unknown)
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
