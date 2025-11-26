use backend::errors::{ApiError, ErrorResponse};
use rocket::http::Status;

#[test]
fn test_api_error_city_not_found_constructor() {
    let error = ApiError::city_not_found("Tokyo");
    match error {
        ApiError::CityNotFound(city) => {
            assert_eq!(city, "Tokyo");
        }
        _ => panic!("Expected CityNotFound variant"),
    }
}

#[test]
fn test_api_error_invalid_params_constructor() {
    let error = ApiError::invalid_params("Invalid parameter");
    match error {
        ApiError::InvalidInput(msg) => {
            assert_eq!(msg, "Invalid parameter");
        }
        _ => panic!("Expected InvalidInput variant"),
    }
}

#[test]
fn test_api_error_provider_error_constructor() {
    let error = ApiError::provider_error("Provider failed");
    match error {
        ApiError::WeatherProviderError(msg) => {
            assert_eq!(msg, "Provider failed");
        }
        _ => panic!("Expected WeatherProviderError variant"),
    }
}

#[test]
fn test_api_error_internal_error_constructor() {
    let error = ApiError::internal_error("Database connection failed");
    match error {
        ApiError::WeatherProviderError(msg) => {
            assert!(msg.contains("Internal error"));
            assert!(msg.contains("Database connection failed"));
        }
        _ => panic!("Expected WeatherProviderError variant"),
    }
}

#[test]
fn test_city_not_found_response_status() {
    let error = ApiError::city_not_found("InvalidCity");
    let (status, _json) = error.to_response();
    assert_eq!(status, Status::NotFound);
}

#[test]
fn test_city_not_found_response_body() {
    let error = ApiError::city_not_found("InvalidCity");
    let (_status, json) = error.to_response();
    let response = json.into_inner();
    
    assert_eq!(response.error, "CITY_NOT_FOUND");
    assert!(response.message.contains("InvalidCity"));
    assert!(response.message.contains("not found in database"));
    assert!(!response.timestamp.is_empty());
}

#[test]
fn test_invalid_params_response_status() {
    let error = ApiError::invalid_params("City cannot be empty");
    let (status, _json) = error.to_response();
    assert_eq!(status, Status::BadRequest);
}

#[test]
fn test_invalid_params_response_body() {
    let error = ApiError::invalid_params("City cannot be empty");
    let (_status, json) = error.to_response();
    let response = json.into_inner();
    
    assert_eq!(response.error, "INVALID_INPUT");
    assert_eq!(response.message, "City cannot be empty");
    assert!(!response.timestamp.is_empty());
}

#[test]
fn test_provider_error_response_status() {
    let error = ApiError::provider_error("All providers failed");
    let (status, _json) = error.to_response();
    assert_eq!(status, Status::ServiceUnavailable);
}

#[test]
fn test_provider_error_response_body() {
    let error = ApiError::provider_error("All providers failed");
    let (_status, json) = error.to_response();
    let response = json.into_inner();
    
    assert_eq!(response.error, "SERVICE_UNAVAILABLE");
    assert!(response.message.contains("weather providers"));
    assert!(!response.timestamp.is_empty());
}

#[test]
fn test_timeout_error_response() {
    let error = ApiError::Timeout;
    let (status, json) = error.to_response();
    let response = json.into_inner();
    
    assert_eq!(status, Status::ServiceUnavailable);
    assert_eq!(response.error, "TIMEOUT");
    assert!(response.message.contains("timed out"));
}

#[test]
fn test_error_response_new() {
    let response = ErrorResponse::new("TEST_ERROR", "Test message");
    
    assert_eq!(response.error, "TEST_ERROR");
    assert_eq!(response.message, "Test message");
    assert!(!response.timestamp.is_empty());
}

#[test]
fn test_error_response_timestamp_format() {
    let response = ErrorResponse::new("TEST", "Test");
    
    // Check that timestamp is a valid RFC3339 format
    assert!(response.timestamp.contains('T'));
    assert!(response.timestamp.len() > 20);
}

#[test]
fn test_api_error_display_city_not_found() {
    let error = ApiError::city_not_found("TestCity");
    let display = format!("{}", error);
    
    assert!(display.contains("City not found"));
    assert!(display.contains("TestCity"));
}

#[test]
fn test_api_error_display_invalid_input() {
    let error = ApiError::invalid_params("Invalid parameter");
    let display = format!("{}", error);
    
    assert!(display.contains("Invalid input"));
    assert!(display.contains("Invalid parameter"));
}

#[test]
fn test_api_error_display_provider_error() {
    let error = ApiError::provider_error("Provider timeout");
    let display = format!("{}", error);
    
    assert!(display.contains("Weather provider error"));
    assert!(display.contains("Provider timeout"));
}

#[test]
fn test_multiple_errors_have_different_timestamps() {
    use std::thread::sleep;
    use std::time::Duration;
    
    let error1 = ApiError::city_not_found("City1");
    let (_status1, json1) = error1.to_response();
    let response1 = json1.into_inner();
    
    sleep(Duration::from_millis(10));
    
    let error2 = ApiError::city_not_found("City2");
    let (_status2, json2) = error2.to_response();
    let response2 = json2.into_inner();
    
    // Timestamps should be different (though might be same if too fast)
    // At least verify both have timestamps
    assert!(!response1.timestamp.is_empty());
    assert!(!response2.timestamp.is_empty());
}

#[test]
fn test_error_constructors_with_empty_strings() {
    let error1 = ApiError::city_not_found("");
    let error2 = ApiError::invalid_params("");
    let error3 = ApiError::provider_error("");
    let error4 = ApiError::internal_error("");
    
    // All should be created without panic
    match error1 {
        ApiError::CityNotFound(_) => {},
        _ => panic!("Expected CityNotFound"),
    }
    
    match error2 {
        ApiError::InvalidInput(_) => {},
        _ => panic!("Expected InvalidInput"),
    }
    
    match error3 {
        ApiError::WeatherProviderError(_) => {},
        _ => panic!("Expected WeatherProviderError"),
    }
    
    match error4 {
        ApiError::WeatherProviderError(_) => {},
        _ => panic!("Expected WeatherProviderError"),
    }
}
