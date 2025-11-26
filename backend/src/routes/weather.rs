use rocket::{get, State, serde::json::Json, http::Status};
use log::{info, warn, error};
use crate::models::{WeatherForecast, City, EnsembleForecast, ForecastPeriodRequest};
use crate::services::WeatherService;
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
) -> Result<Json<EnsembleForecast>, (Status, Json<ErrorResponse>)> {
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

    // For now, return mock data
    let mut ensemble = EnsembleForecast::new(
        city_data.name.to_string(),
        city_data.province.to_string(),
        "Indonesia".to_string(),
        city_data.latitude,
        city_data.longitude,
    );

    // Create mock 7-day forecast
    use crate::models::{DayEnsemble, PerSourceData, ProviderForecast, FinalForecast};
    use chrono::{Local, Duration};

    let start_date = Local::now().date_naive();
    
    for i in 0..7 {
        let date = start_date + Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        
        // Mock per-source data
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                date_str.clone(),
                28.0 + i as f32,
                22.0 + i as f32 * 0.5,
                "Partly Cloudy".to_string(),
            ))
            .with_open_weather(ProviderForecast::new(
                date_str.clone(),
                27.5 + i as f32,
                21.5 + i as f32 * 0.5,
                "Cloudy".to_string(),
            ))
            .with_weather_api(ProviderForecast::new(
                date_str.clone(),
                28.5 + i as f32,
                22.5 + i as f32 * 0.5,
                "Sunny".to_string(),
            ));

        // Mock final forecast
        let confidence = if i < 2 {
            "high"
        } else if i < 5 {
            "medium"
        } else {
            "low"
        };

        let final_forecast = FinalForecast::new(
            28.0 + i as f32,
            22.0 + i as f32 * 0.5,
            "Partly Cloudy".to_string(),
            confidence.to_string(),
        );

        ensemble.add_day(DayEnsemble::new(date_str, per_source, final_forecast));
    }

    info!("Successfully generated mock ensemble forecast for {}", city_name);
    Ok(Json(ensemble))
}
