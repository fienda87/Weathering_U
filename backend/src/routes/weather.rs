use rocket::{get, State, serde::json::Json, http::Status};
use log::{info, warn, error, debug};
use crate::models::{WeatherForecast, City, EnsembleForecast, ForecastPeriodRequest};
use crate::services::{WeatherService, ForecastCache, EnsembleOrchestrator, find_city, validate_city_input, get_all_cities};
use crate::utils::Config;
use crate::errors::{ApiError, ErrorResponse};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Semaphore;

// Helper: Validasi dan cari city dari query parameter
fn validate_and_find_city(
    city_param: Option<String>,
    context: &str,
) -> Result<City, (Status, Json<ErrorResponse>)> {
    let city_str = city_param.ok_or_else(|| {
        let err = ApiError::invalid_params("Missing required query parameter: city");
        warn!("[{}] {}", context, err);
        err.to_response()
    })?;

    let city_name = validate_city_input(&city_str).map_err(|e| {
        warn!("[{}] Invalid city input '{}': {}", context, city_str, e);
        e.to_response()
    })?;

    find_city(&city_name).map_err(|e| {
        warn!("[{}] City not found: '{}'", context, city_name);
        e.to_response()
    })
}

// Helper: Validasi parameter period next_week
fn validate_next_week_params(
    period_str: &str,
    day: Option<u32>,
    context: &str,
) -> Result<(), (Status, Json<ErrorResponse>)> {
    if period_str != "next_week" {
        return Ok(());
    }

    // Validasi day parameter wajib untuk next_week
    let day_value = day.ok_or_else(|| {
        let err = ApiError::invalid_params("Day parameter required for next_week period");
        warn!("[{}] {}", context, err);
        err.to_response()
    })?;

    // Validasi range day (0-6 untuk Monday-Sunday)
    if day_value > 6 {
        let err = ApiError::invalid_params("Day must be 0-6 (Monday-Sunday)");
        warn!("[{}] Invalid day parameter: {}", context, day_value);
        return Err(err.to_response());
    }

    Ok(())
}

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
    info!("GET /api/cities - Returning {} cities", get_all_cities().len());
    
    // Direct functional pipeline: get → map → collect → wrap → json
    Json(CitiesResponse {
        cities: get_all_cities().iter().map(CityResponse::from).collect()
    })
}
#[get("/api/weather?<city>")]
pub async fn get_weather(
    city: Option<String>,
    weather_service: &State<WeatherService>,
    semaphore: &State<Arc<Semaphore>>,
) -> Result<Json<WeatherForecast>, (Status, Json<ErrorResponse>)> {
    // Validasi dan cari city (chained validation)
    let city_data = validate_and_find_city(city, "Weather")?;

    info!("[Weather] GET /api/weather?city={} - Looking up forecast (rate-limited parallel)", city_data.name);
    debug!("[Weather] City coordinates: ({}, {})", city_data.latitude, city_data.longitude);

    weather_service.get_forecast_rate_limited(&city_data, semaphore.inner().clone()).await
        .map(|forecast| {
            
            let updated_forecast = WeatherForecast {
                province: city_data.province.to_string(),
                ..forecast
            };
            info!("[Weather] Successfully retrieved forecast for {} with {} days", city_data.name, updated_forecast.forecast.len());
            Json(updated_forecast)
        })
        .map_err(|e| {
            error!("[Weather] Provider error for city '{}': {}", city_data.name, e);
            ApiError::provider_error(&e).to_response()
        })
}

#[get("/api/weather/parallel?<city>")]
pub async fn get_weather_parallel(
    city: Option<String>,
    weather_service: &State<WeatherService>,
) -> Result<Json<WeatherForecast>, (Status, Json<ErrorResponse>)> {
    // Validasi dan cari city (chained validation)
    let city_data = validate_and_find_city(city, "WeatherParallel")?;

    info!("[WeatherParallel] GET /api/weather/parallel?city={} - Looking up forecast (unlimited parallel)", city_data.name);
    debug!("[WeatherParallel] City coordinates: ({}, {})", city_data.latitude, city_data.longitude);

    weather_service.get_forecast_parallel(&city_data).await
        .map(|forecast| {
  
            let updated_forecast = WeatherForecast {
                province: city_data.province.to_string(),
                ..forecast
            };
            info!("[WeatherParallel] Successfully retrieved forecast for {} with {} days", city_data.name, updated_forecast.forecast.len());
            Json(updated_forecast)
        })
        .map_err(|e| {
            error!("[WeatherParallel] Provider error for city '{}': {}", city_data.name, e);
            ApiError::provider_error(&e).to_response()
        })
}

#[get("/api/weather/ensemble?<city>&<period>&<day>")]
pub async fn get_ensemble_forecast(
    city: Option<String>,
    period: Option<String>,
    day: Option<u32>,
    cache: &State<Arc<ForecastCache<EnsembleForecast>>>,
    config: &State<Config>,
) -> Result<Json<EnsembleForecast>, (Status, Json<ErrorResponse>)> {
    // Validasi dan cari city (chained validation)
    let city_data = validate_and_find_city(city, "Ensemble")?;

    let period_str = period.as_deref().unwrap_or("current_week");
    debug!("[Ensemble] Query params - city: {}, period: {:?}, day: {:?}", city_data.name, &period, day);

    // Validasi parameter next_week (functional approach)
    validate_next_week_params(period_str, day, "Ensemble")?;

    // Parse forecast period dengan functional error handling
    let forecast_period = ForecastPeriodRequest::from_query(period.clone(), day).map_err(|e| {
        warn!("[Ensemble] Invalid forecast period: {}", e);
        ApiError::invalid_params(&e).to_response()
    })?;

    info!(
        "[Ensemble] GET /api/weather/ensemble?city={}&period={} - Fetching ensemble forecast",
        city_data.name,
        period_str
    );
    debug!("[Ensemble] Forecast period: {:?}", forecast_period);

    let orchestrator = EnsembleOrchestrator::new(
        cache.inner().clone(),
        config.openweather_key.clone(),
        config.weatherapi_key.clone(),
    );

    orchestrator.get_forecast(&city_data, forecast_period).await
        .map(|ensemble| {
            info!(
                "[Ensemble] Successfully fetched ensemble forecast for {} with {} days",
                city_data.name,
                ensemble.days.len()
            );
            Json(ensemble)
        })
        .map_err(|e| {
            error!("[Ensemble] Failed to fetch ensemble forecast for '{}': {}", city_data.name, e);
            ApiError::provider_error(&e).to_response()
        })
}
