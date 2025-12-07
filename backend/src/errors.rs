use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub timestamp: String,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    CityNotFound(String),
    InvalidInput(String),
    WeatherProviderError(String),
    #[allow(dead_code)]
    Timeout,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::CityNotFound(city) => write!(f, "City not found: {}", city),
            ApiError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ApiError::WeatherProviderError(msg) => write!(f, "Weather provider error: {}", msg),
            ApiError::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl ApiError {
    pub fn city_not_found(city: &str) -> Self {
        ApiError::CityNotFound(city.to_string())
    }


    pub fn invalid_params(message: &str) -> Self {
        ApiError::InvalidInput(message.to_string())
    }

    pub fn provider_error(message: &str) -> Self {
        ApiError::WeatherProviderError(message.to_string())
    }

    pub fn to_response(&self) -> (Status, Json<ErrorResponse>) {
        match self {
            ApiError::CityNotFound(city) => (
                Status::NotFound,
                Json(ErrorResponse::new(
                    "CITY_NOT_FOUND",
                    &format!("City '{}' not found in database", city),
                )),
            ),
            ApiError::InvalidInput(msg) => (
                Status::BadRequest,
                Json(ErrorResponse::new("INVALID_INPUT", msg)),
            ),
            ApiError::WeatherProviderError(_) => (
                Status::ServiceUnavailable,
                Json(ErrorResponse::new(
                    "SERVICE_UNAVAILABLE",
                    "All weather providers are currently unavailable. Please try again later.",
                )),
            ),
            ApiError::Timeout => (
                Status::ServiceUnavailable,
                Json(ErrorResponse::new(
                    "TIMEOUT",
                    "Request timed out. Please try again later.",
                )),
            ),
        }
    }
}
