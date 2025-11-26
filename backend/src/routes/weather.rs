use rocket::{get, State, serde::json::Json, http::Status};
use log::{info, warn, error, debug};
use crate::models::{WeatherForecast, City, EnsembleForecast, ForecastPeriodRequest};
use crate::services::{WeatherService, ForecastCache, EnsembleOrchestrator, find_city, validate_city_input, get_all_cities};
use crate::utils::Config;
use crate::errors::{ApiError, ErrorResponse};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Debug, Serialize, Deserialize)]
pub struct CitiesResponse {
    pub cities: Vec<CityResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityResponse {
    pub id: u32,
    pub name: String,
    pub province: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<&City> for CityResponse {
    fn from(city: &City) -> Self {
        Self {
            id: city.id,
            name: city.name.to_string(),
            province: city.province.to_string(),
            latitude: city.latitude,
            longitude: city.longitude,
        }
    }
}

#[get("/api/cities")]
pub fn get_cities() -> Json<CitiesResponse> {
    let all_cities = get_all_cities();
    info!("GET /api/cities - Fetching all {} cities", all_cities.len());
    let cities = all_cities.iter().map(CityResponse::from).collect();
    Json(CitiesResponse { cities })
}

#[get("/api/weather?<city>")]
pub async fn get_weather(
    city: Option<String>,
    weather_service: &State<WeatherService>,
    semaphore: &State<Arc<Semaphore>>,
) -> Result<Json<WeatherForecast>, (Status, Json<ErrorResponse>)> {
    // Validate city parameter exists
    let city_str = match city {
        Some(c) => c,
        None => {
            let err = ApiError::invalid_params("Missing required query parameter: city");
            warn!("[Weather] {}", err);
            return Err(err.to_response());
        }
    };

    // Validate city input format
    let city_name = match validate_city_input(&city_str) {
        Ok(valid) => valid,
        Err(e) => {
            warn!("[Weather] Invalid city input '{}': {}", city_str, e);
            return Err(e.to_response());
        }
    };

    // Find city in database
    let city_data = match find_city(&city_name) {
        Ok(c) => c,
        Err(e) => {
            warn!("[Weather] City not found: '{}'", city_name);
            return Err(e.to_response());
        }
    };

    info!("[Weather] GET /api/weather?city={} - Looking up forecast (rate-limited parallel)", city_name);
    debug!("[Weather] City coordinates: ({}, {})", city_data.latitude, city_data.longitude);

    // Fetch weather forecast with rate-limited parallel processing
    match weather_service.get_forecast_rate_limited(city_data, semaphore.inner().clone()).await {
        Ok(mut forecast) => {
            forecast.province = city_data.province.to_string();
            info!("[Weather] Successfully retrieved forecast for {} with {} days", city_name, forecast.forecast.len());
            Ok(Json(forecast))
        }
        Err(e) => {
            error!("[Weather] Provider error for city '{}': {}", city_name, e);
            let err = ApiError::provider_error(&e);
            Err(err.to_response())
        }
    }
}

#[get("/api/weather/parallel?<city>")]
pub async fn get_weather_parallel(
    city: Option<String>,
    weather_service: &State<WeatherService>,
) -> Result<Json<WeatherForecast>, (Status, Json<ErrorResponse>)> {
    // Validate city parameter exists
    let city_str = match city {
        Some(c) => c,
        None => {
            let err = ApiError::invalid_params("Missing required query parameter: city");
            warn!("[WeatherParallel] {}", err);
            return Err(err.to_response());
        }
    };

    // Validate city input format
    let city_name = match validate_city_input(&city_str) {
        Ok(valid) => valid,
        Err(e) => {
            warn!("[WeatherParallel] Invalid city input '{}': {}", city_str, e);
            return Err(e.to_response());
        }
    };

    // Find city in database
    let city_data = match find_city(&city_name) {
        Ok(c) => c,
        Err(e) => {
            warn!("[WeatherParallel] City not found: '{}'", city_name);
            return Err(e.to_response());
        }
    };

    info!("[WeatherParallel] GET /api/weather/parallel?city={} - Looking up forecast (unlimited parallel)", city_name);
    debug!("[WeatherParallel] City coordinates: ({}, {})", city_data.latitude, city_data.longitude);

    // Fetch weather forecast with unlimited parallel processing
    match weather_service.get_forecast_parallel(city_data).await {
        Ok(mut forecast) => {
            forecast.province = city_data.province.to_string();
            info!("[WeatherParallel] Successfully retrieved forecast for {} with {} days", city_name, forecast.forecast.len());
            Ok(Json(forecast))
        }
        Err(e) => {
            error!("[WeatherParallel] Provider error for city '{}': {}", city_name, e);
            let err = ApiError::provider_error(&e);
            Err(err.to_response())
        }
    }
}

#[get("/api/weather/ensemble?<city>&<period>&<day>")]
pub async fn get_ensemble_forecast(
    city: Option<String>,
    period: Option<String>,
    day: Option<u32>,
    cache: &State<Arc<ForecastCache<EnsembleForecast>>>,
    config: &State<Config>,
) -> Result<Json<EnsembleForecast>, (Status, Json<ErrorResponse>)> {
    // Validate city parameter exists
    let city_str = match city {
        Some(c) => c,
        None => {
            let err = ApiError::invalid_params("Missing required query parameter: city");
            warn!("[Ensemble] {}", err);
            return Err(err.to_response());
        }
    };

    // Validate city input format
    let city_name = match validate_city_input(&city_str) {
        Ok(valid) => valid,
        Err(e) => {
            warn!("[Ensemble] Invalid city input '{}': {}", city_str, e);
            return Err(e.to_response());
        }
    };

    // Find city in database
    let city_data = match find_city(&city_name) {
        Ok(c) => c,
        Err(e) => {
            error!("[Ensemble] City validation failed for: '{}'", city_name);
            return Err(e.to_response());
        }
    };

    // Log query parameters for debugging
    let period_str = period.as_deref().unwrap_or("current_week");
    debug!("[Ensemble] Query params - city: {}, period: {:?}, day: {:?}", city_name, &period, day);

    // Validate period and day combination
    if period_str == "next_week" {
        if day.is_none() {
            let err = ApiError::invalid_params("Day parameter required for next_week period");
            warn!("[Ensemble] {}", err);
            return Err(err.to_response());
        }
        
        if let Some(d) = day {
            if d > 6 {
                let err = ApiError::invalid_params("Day must be 0-6 (Monday-Sunday)");
                warn!("[Ensemble] Invalid day parameter: {}", d);
                return Err(err.to_response());
            }
        }
    }

    // Parse forecast period using the request model
    let forecast_period = match ForecastPeriodRequest::from_query(period.clone(), day) {
        Ok(p) => p,
        Err(e) => {
            warn!("[Ensemble] Invalid forecast period: {}", e);
            let err = ApiError::invalid_params(&e);
            return Err(err.to_response());
        }
    };

    info!(
        "[Ensemble] GET /api/weather/ensemble?city={}&period={} - Fetching ensemble forecast",
        city_name,
        period_str
    );
    debug!("[Ensemble] Forecast period: {:?}", forecast_period);

    // Create orchestrator with cache and API keys
    let orchestrator = EnsembleOrchestrator::new(
        cache.inner().clone(),
        config.openweather_key.clone(),
        config.weatherapi_key.clone(),
    );

    // Fetch real forecast data
    match orchestrator.get_forecast(city_data, forecast_period).await {
        Ok(ensemble) => {
            info!(
                "[Ensemble] Successfully fetched ensemble forecast for {} with {} days",
                city_name,
                ensemble.days.len()
            );
            Ok(Json(ensemble))
        }
        Err(e) => {
            error!("[Ensemble] Failed to fetch ensemble forecast for '{}': {}", city_name, e);
            let err = ApiError::provider_error(&e);
            Err(err.to_response())
        }
    }
}
