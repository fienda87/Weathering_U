use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub city: String,
    pub province: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub last_updated: String,
    pub forecast: Vec<DailyForecast>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyForecast {
    pub date: String,
    pub temp_max: f32,
    pub temp_min: f32,
    pub temp_avg: f32,
    pub condition: String,
    pub humidity: u32,
    pub wind_speed: f32,
    pub icon: String,
}
