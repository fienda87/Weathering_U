use rocket::local::blocking::Client;
use rocket::local::blocking::LocalResponse;
use rocket::http::{Status, Header};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use backend::services::{WeatherService, ForecastCache};
use backend::models::EnsembleForecast;
use backend::utils::Config;
use backend::routes::routes;
use tokio::sync::Semaphore;
use chrono::NaiveDate;

/// Test client wrapper for making HTTP requests
pub struct TestClient {
    client: Client,
}

impl TestClient {
    /// Create a new test client with full rocket setup
    pub fn new() -> Self {
        let config = Config::from_env();
        
        let weather_service = WeatherService::new(
            config.openweather_key.clone(),
            config.weatherapi_key.clone(),
        );
        
        let ensemble_cache = Arc::new(ForecastCache::<EnsembleForecast>::new(3600, 100));
        let semaphore = Arc::new(Semaphore::new(3));
        
        let rocket = rocket::build()
            .manage(weather_service)
            .manage(semaphore)
            .manage(ensemble_cache)
            .manage(config)
            .mount("/", routes());
        
        let client = Client::tracked(rocket).expect("valid rocket instance");
        
        Self { client }
    }
    
    /// Make a GET request
    pub fn get(&self, uri: &str) -> TestResponse {
        let response = self.client.get(uri).dispatch();
        TestResponse { response }
    }
    
    /// Make a GET request with custom headers
    pub fn get_with_headers(&self, uri: &str, headers: Vec<(&str, &str)>) -> TestResponse {
        let mut req = self.client.get(uri);
        for (name, value) in headers {
            req.add_header(Header::new(name, value));
        }
        let response = req.dispatch();
        TestResponse { response }
    }
}

/// Test response wrapper with helper methods
pub struct TestResponse<'a> {
    response: LocalResponse<'a>,
}

impl<'a> TestResponse<'a> {
    /// Get the response status
    pub fn status(&self) -> Status {
        self.response.status()
    }
    
    /// Parse response body as JSON into type T
    pub fn json<T: DeserializeOwned>(&mut self) -> T {
        self.response.into_json::<T>().expect("Failed to parse JSON")
    }
    
    /// Parse response body as generic JSON object
    pub fn json_value(&mut self) -> Value {
        let body = self.response.into_string().expect("Failed to get body");
        serde_json::from_str(&body).expect("Failed to parse JSON")
    }
    
    /// Get the response body as string
    pub fn body(&mut self) -> String {
        self.response.into_string().expect("Failed to get body")
    }
    
    /// Check if response has a specific header
    pub fn has_header(&self, name: &str) -> bool {
        self.response.headers().get_one(name).is_some()
    }
    
    /// Get header value
    pub fn header(&self, name: &str) -> Option<String> {
        self.response.headers().get_one(name).map(|s| s.to_string())
    }
}

/// Assertion helpers for common validations
pub mod assertions {
    use super::*;
    use backend::models::{EnsembleForecast, DayEnsemble};
    
    /// Assert response is successful
    pub fn assert_success(status: Status) {
        assert!(status.class().is_success(), "Expected success status, got: {}", status);
    }
    
    /// Assert response is 200 OK
    pub fn assert_ok(status: Status) {
        assert_eq!(status, Status::Ok, "Expected 200 OK, got: {}", status);
    }
    
    /// Assert response is 404 Not Found
    pub fn assert_not_found(status: Status) {
        assert_eq!(status, Status::NotFound, "Expected 404 Not Found, got: {}", status);
    }
    
    /// Assert response is 400 Bad Request
    pub fn assert_bad_request(status: Status) {
        assert_eq!(status, Status::BadRequest, "Expected 400 Bad Request, got: {}", status);
    }
    
    /// Assert ensemble forecast structure is valid
    pub fn assert_valid_ensemble(forecast: &EnsembleForecast) {
        assert!(!forecast.city.is_empty(), "City should not be empty");
        assert!(!forecast.province.is_empty(), "Province should not be empty");
        assert!(!forecast.country.is_empty(), "Country should not be empty");
        assert!(!forecast.source_timestamp.is_empty(), "Timestamp should not be empty");
        
        // Validate coordinates (Indonesia roughly between -11 to 6 lat, 95 to 141 lon)
        assert!(forecast.latitude >= -12.0 && forecast.latitude <= 7.0, 
            "Latitude should be in valid range for Indonesia: {}", forecast.latitude);
        assert!(forecast.longitude >= 94.0 && forecast.longitude <= 142.0, 
            "Longitude should be in valid range for Indonesia: {}", forecast.longitude);
    }
    
    /// Assert day ensemble structure is valid
    pub fn assert_valid_day(day: &DayEnsemble, index: usize) {
        // Validate date format (YYYY-MM-DD)
        assert!(day.date.len() == 10, "Date should be in YYYY-MM-DD format at day {}", index);
        assert!(day.date.contains('-'), "Date should contain hyphens at day {}", index);
        
        // Validate final forecast
        let final_forecast = &day.final_forecast;
        assert!(final_forecast.temp_max >= -50.0 && final_forecast.temp_max <= 50.0, 
            "temp_max should be in valid range at day {}: {}", index, final_forecast.temp_max);
        assert!(final_forecast.temp_min >= -50.0 && final_forecast.temp_min <= 50.0, 
            "temp_min should be in valid range at day {}: {}", index, final_forecast.temp_min);
        assert!(final_forecast.temp_max >= final_forecast.temp_min, 
            "temp_max should be >= temp_min at day {}: {} >= {}", 
            index, final_forecast.temp_max, final_forecast.temp_min);
        assert!(!final_forecast.condition.is_empty(), 
            "Condition should not be empty at day {}", index);
        
        // Validate confidence
        assert!(
            ["high", "medium", "low"].contains(&final_forecast.confidence.as_str()),
            "Confidence should be high/medium/low at day {}, got: {}", 
            index, final_forecast.confidence
        );
        
        // At least one provider should have data
        let provider_count = day.per_source.provider_count();
        assert!(provider_count > 0, "At least one provider should have data at day {}", index);
    }
    
    /// Assert date format is ISO 8601 (YYYY-MM-DD)
    pub fn assert_iso_date_format(date: &str, label: &str) {
        assert_eq!(date.len(), 10, "{} should be 10 characters (YYYY-MM-DD): {}", label, date);
        let parts: Vec<&str> = date.split('-').collect();
        assert_eq!(parts.len(), 3, "{} should have 3 parts separated by hyphens: {}", label, date);
        assert_eq!(parts[0].len(), 4, "{} year should be 4 digits: {}", label, date);
        assert_eq!(parts[1].len(), 2, "{} month should be 2 digits: {}", label, date);
        assert_eq!(parts[2].len(), 2, "{} day should be 2 digits: {}", label, date);
        
        // Parse to ensure valid numbers
        let _year: u32 = parts[0].parse().expect(&format!("{} year should be numeric: {}", label, date));
        let month: u32 = parts[1].parse().expect(&format!("{} month should be numeric: {}", label, date));
        let day: u32 = parts[2].parse().expect(&format!("{} day should be numeric: {}", label, date));
        
        assert!(month >= 1 && month <= 12, "{} month should be 1-12: {}", label, date);
        assert!(day >= 1 && day <= 31, "{} day should be 1-31: {}", label, date);
    }
    
    /// Assert dates are sequential (one day apart)
    pub fn assert_sequential_dates(dates: &[String]) {
        for i in 1..dates.len() {
            let prev_date = NaiveDate::parse_from_str(&dates[i-1], "%Y-%m-%d")
                .expect(&format!("Failed to parse date: {}", dates[i-1]));
            let curr_date = NaiveDate::parse_from_str(&dates[i], "%Y-%m-%d")
                .expect(&format!("Failed to parse date: {}", dates[i]));
            
            let diff = curr_date.signed_duration_since(prev_date).num_days();
            assert_eq!(diff, 1, "Dates should be sequential (1 day apart) at index {}: {} -> {}", 
                i, dates[i-1], dates[i]);
        }
    }
    
    /// Assert first date is today (for current week)
    pub fn assert_first_date_is_today(first_date: &str) {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(first_date, today, "First date should be today for current week: expected {}, got {}", 
            today, first_date);
    }
    
    /// Assert error response has required fields
    pub fn assert_error_response(json: &Value, expected_status: u16) {
        assert!(json.get("error").is_some(), "Error response should have 'error' field");
        assert!(json.get("message").is_some(), "Error response should have 'message' field");
        assert!(json.get("timestamp").is_some(), "Error response should have 'timestamp' field");
        
        let status = json.get("status").and_then(|v| v.as_u64());
        if let Some(s) = status {
            assert_eq!(s, expected_status as u64, "Status code should match: expected {}, got {}", 
                expected_status, s);
        }
    }
}

/// Mock data generators for testing
pub mod mocks {
    use backend::models::{ProviderForecast, PerSourceData, FinalForecast, DayEnsemble, EnsembleForecast};
    
    /// Create a mock provider forecast
    pub fn mock_provider_forecast(date: &str, temp_max: f32, temp_min: f32, condition: &str) -> ProviderForecast {
        ProviderForecast::new(
            date.to_string(),
            temp_max,
            temp_min,
            condition.to_string(),
        )
    }
    
    /// Create mock per-source data with all providers
    pub fn mock_per_source_data(date: &str) -> PerSourceData {
        PerSourceData::new()
            .with_open_meteo(mock_provider_forecast(date, 32.0, 24.0, "Partly Cloudy"))
            .with_open_weather(mock_provider_forecast(date, 33.0, 25.0, "Sunny"))
            .with_weather_api(mock_provider_forecast(date, 31.0, 24.5, "Clear"))
    }
    
    /// Create a mock final forecast
    pub fn mock_final_forecast(temp_max: f32, temp_min: f32, condition: &str, confidence: &str) -> FinalForecast {
        FinalForecast::new(temp_max, temp_min, condition.to_string(), confidence.to_string())
    }
    
    /// Create a mock day ensemble
    pub fn mock_day_ensemble(date: &str) -> DayEnsemble {
        DayEnsemble::new(
            date.to_string(),
            mock_per_source_data(date),
            mock_final_forecast(32.0, 24.5, "Partly Cloudy", "high"),
        )
    }
    
    /// Create a mock ensemble forecast
    pub fn mock_ensemble_forecast(city: &str, num_days: usize) -> EnsembleForecast {
        let mut forecast = EnsembleForecast::new(
            city.to_string(),
            "Test Province".to_string(),
            "Indonesia".to_string(),
            -6.2088,
            106.8456,
        );
        
        let start_date = chrono::Local::now().naive_local().date();
        for i in 0..num_days {
            let date = start_date + chrono::Duration::days(i as i64);
            let date_str = date.format("%Y-%m-%d").to_string();
            forecast.add_day(mock_day_ensemble(&date_str));
        }
        
        forecast
    }
}
