use rocket::{get, State, serde::json::Json, http::Status};
use log::{info, warn, error};
use crate::models::{WeatherForecast, City};
use crate::services::WeatherService;
use crate::cities::CITIES;
use crate::errors::{ApiError, ErrorResponse};
use serde::{Serialize, Deserialize};

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

    info!("GET /api/weather?city={} - Looking up forecast", city_name);

    match weather_service.get_forecast(&city_name, city_data.latitude, city_data.longitude).await {
        Ok(mut forecast) => {
            forecast.province = city_data.province.to_string();
            info!("Successfully retrieved weather forecast for {}", city_name);
            Ok(Json(forecast))
        }
        Err(e) => {
            error!("Weather service error for city {}: {}", city_name, e);
            let err = ApiError::WeatherProviderError(e);
            Err(err.to_response())
        }
    }
}
