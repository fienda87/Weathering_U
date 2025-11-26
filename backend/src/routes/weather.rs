use rocket::{get, State, serde::json::Json, http::Status};
use log::{info, warn, error};
use crate::models::{WeatherForecast, City, EnsembleForecast, ForecastPeriodRequest};
use crate::services::{WeatherService, ForecastCache, EnsembleOrchestrator};
use crate::utils::Config;
use crate::cities::CITIES;
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
    info!("GET /api/cities - Fetching all {} cities", CITIES.len());
    let cities = CITIES.iter().map(CityResponse::from).collect();
    Json(CitiesResponse { cities })
}

fn find_city(name: &str) -> Option<&'static City> {
    let query = name.trim().to_lowercase();
    CITIES.iter().find(|city| city.name.to_lowercase() == query)
}

fn validate_city_input(city: &str) -> Result<String, ApiError> {
    let trimmed = city.trim();
    
    if trimmed.is_empty() {
        return Err(ApiError::InvalidInput(
            "City name is required".to_string(),
        ));
    }
    
    if trimmed.len() > 50 {
        return Err(ApiError::InvalidInput(
            "City name must not exceed 50 characters".to_string(),
        ));
    }
    
    Ok(trimmed.to_string())
}

#[get("/api/weather?<city>")]
pub async fn get_weather(
    city: Option<String>,
    weather_service: &State<WeatherService>,
    semaphore: &State<Arc<Semaphore>>,
) -> Result<Json<WeatherForecast>, (Status, Json<ErrorResponse>)> {
    let city_name = match city {
        Some(c) => {
            match validate_city_input(&c) {
                Ok(valid) => valid,
                Err(e) => {
                    warn!("Invalid city input: {}", e);
                    return Err(e.to_response());
                }
            }
        }
        None => {
            let err = ApiError::InvalidInput("Missing required query parameter: city".to_string());
            warn!("Missing city parameter");
            return Err(err.to_response());
        }
    };

    let city_data = match find_city(&city_name) {
        Some(c) => c,
        None => {
            let err = ApiError::CityNotFound(city_name.clone());
            warn!("City not found: {}", city_name);
            return Err(err.to_response());
        }
    };

    info!("GET /api/weather?city={} - Looking up forecast (using parallel processing)", city_name);

    // Use rate-limited parallel processing for better performance
    match weather_service.get_forecast_rate_limited(city_data, semaphore.inner().clone()).await {
        Ok(mut forecast) => {
            forecast.province = city_data.province.to_string();
            info!("Successfully retrieved parallel weather forecast for {}", city_name);
            Ok(Json(forecast))
        }
        Err(e) => {
            error!("Parallel weather service error for city {}: {}", city_name, e);
            let err = ApiError::WeatherProviderError(e);
            Err(err.to_response())
        }
    }
}

#[get("/api/weather/parallel?<city>")]
pub async fn get_weather_parallel(
    city: Option<String>,
    weather_service: &State<WeatherService>,
) -> Result<Json<WeatherForecast>, (Status, Json<ErrorResponse>)> {
    let city_name = match city {
        Some(c) => {
            match validate_city_input(&c) {
                Ok(valid) => valid,
                Err(e) => {
                    warn!("Invalid city input: {}", e);
                    return Err(e.to_response());
                }
            }
        }
        None => {
            let err = ApiError::InvalidInput("Missing required query parameter: city".to_string());
            warn!("Missing city parameter");
            return Err(err.to_response());
        }
    };

    let city_data = match find_city(&city_name) {
        Some(c) => c,
        None => {
            let err = ApiError::CityNotFound(city_name.clone());
            warn!("City not found: {}", city_name);
            return Err(err.to_response());
        }
    };

    info!("GET /api/weather/parallel?city={} - Looking up forecast (using unlimited parallel processing)", city_name);

    // Use unlimited parallel processing for testing
    match weather_service.get_forecast_parallel(city_data).await {
        Ok(mut forecast) => {
            forecast.province = city_data.province.to_string();
            info!("Successfully retrieved unlimited parallel weather forecast for {}", city_name);
            Ok(Json(forecast))
        }
        Err(e) => {
            error!("Unlimited parallel weather service error for city {}: {}", city_name, e);
            let err = ApiError::WeatherProviderError(e);
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
    // Validate city input
    let city_name = match city {
        Some(c) => {
            match validate_city_input(&c) {
                Ok(valid) => valid,
                Err(e) => {
                    warn!("Invalid city input: {}", e);
                    return Err(e.to_response());
                }
            }
        }
        None => {
            let err = ApiError::InvalidInput("Missing required query parameter: city".to_string());
            warn!("Missing city parameter");
            return Err(err.to_response());
        }
    };

    // Validate city exists
    let city_data = match find_city(&city_name) {
        Some(c) => c,
        None => {
            let err = ApiError::CityNotFound(city_name.clone());
            warn!("City not found: {}", city_name);
            return Err(err.to_response());
        }
    };

    // Parse forecast period
    let forecast_period = match ForecastPeriodRequest::from_query(period, day) {
        Ok(p) => p,
        Err(e) => {
            warn!("Invalid forecast period: {}", e);
            let err = ApiError::InvalidInput(e);
            return Err(err.to_response());
        }
    };

    info!(
        "GET /api/weather/ensemble?city={}&period={:?} - Fetching ensemble forecast",
        city_name,
        forecast_period
    );

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
                "Successfully fetched ensemble forecast for {} with {} days",
                city_name,
                ensemble.days.len()
            );
            Ok(Json(ensemble))
        }
        Err(e) => {
            error!("Failed to fetch ensemble forecast for {}: {}", city_name, e);
            let err = ApiError::WeatherProviderError(e);
            Err(err.to_response())
        }
    }
}
