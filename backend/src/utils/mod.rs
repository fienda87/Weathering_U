use std::env;

pub mod city_search;
pub mod date_utils;

pub use date_utils::{
    ForecastPeriod,
    get_forecast_dates,
    get_current_week_dates,
    get_next_week_date,
    get_weekday_name,
    get_day_of_week_from_date,
    is_today_weekday,
    days_to_weekday,
    get_dates_between,
};

#[derive(Clone)]
pub struct Config {
    pub server_port: u16,
    #[allow(dead_code)]
    pub api_base_url: String,
    #[allow(dead_code)]
    pub api_key: Option<String>,
    pub cors_origins: Vec<String>,
    pub openweather_key: String,
    pub weatherapi_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000);

        let api_base_url = env::var("API_BASE_URL")
            .unwrap_or_else(|_| "https://api.example.com".to_string());

        let api_key = env::var("API_KEY").ok();

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let openweather_key = env::var("OPENWEATHER_API_KEY")
            .unwrap_or_else(|_| "your-key-here".to_string());

        let weatherapi_key = env::var("WEATHERAPI_KEY")
            .unwrap_or_else(|_| "your-key-here".to_string());

        Self {
            server_port,
            api_base_url,
            api_key,
            cors_origins,
            openweather_key,
            weatherapi_key,
        }
    }
}

pub fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
}