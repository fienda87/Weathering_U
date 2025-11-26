use backend::errors::{ApiError, ErrorResponse};
use backend::services::{find_city, validate_city_input};
use rocket::http::Status;

#[tokio::test]
async fn test_end_to_end_valid_city_lookup() {
    // Simulate the full validation chain
    let city_input = "Jakarta";
    
    // Step 1: Validate input
    let validated = validate_city_input(city_input);
    assert!(validated.is_ok());
    
    // Step 2: Find city
    let city_name = validated.unwrap();
    let city = find_city(&city_name);
    assert!(city.is_ok());
    
    let city_data = city.unwrap();
    assert_eq!(city_data.name, "Jakarta");
    assert_eq!(city_data.province, "DKI Jakarta");
}

#[tokio::test]
async fn test_end_to_end_invalid_city_not_found() {
    let city_input = "InvalidCity";
    
    // Step 1: Validate input (should pass)
    let validated = validate_city_input(city_input);
    assert!(validated.is_ok());
    
    // Step 2: Find city (should fail with 404)
    let city_name = validated.unwrap();
    let result = find_city(&city_name);
    assert!(result.is_err());
    
    match result {
        Err(ApiError::CityNotFound(name)) => {
            assert_eq!(name, "InvalidCity");
            
            // Verify response format
            let error = ApiError::city_not_found(&name);
            let (status, json) = error.to_response();
            assert_eq!(status, Status::NotFound);
            
            let response = json.into_inner();
            assert_eq!(response.error, "CITY_NOT_FOUND");
            assert!(response.message.contains("InvalidCity"));
        }
        _ => panic!("Expected CityNotFound error"),
    }
}

#[tokio::test]
async fn test_end_to_end_empty_city_parameter() {
    let city_input = "";
    
    // Step 1: Validate input (should fail with 400)
    let result = validate_city_input(city_input);
    assert!(result.is_err());
    
    match result {
        Err(ApiError::InvalidInput(msg)) => {
            assert_eq!(msg, "City name is required");
            
            // Verify response format
            let error = ApiError::invalid_params(&msg);
            let (status, json) = error.to_response();
            assert_eq!(status, Status::BadRequest);
            
            let response = json.into_inner();
            assert_eq!(response.error, "INVALID_INPUT");
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[tokio::test]
async fn test_end_to_end_city_name_too_long() {
    let city_input = "A".repeat(51);
    
    // Step 1: Validate input (should fail with 400)
    let result = validate_city_input(&city_input);
    assert!(result.is_err());
    
    match result {
        Err(ApiError::InvalidInput(msg)) => {
            assert_eq!(msg, "City name must not exceed 50 characters");
            
            // Verify response format
            let error = ApiError::invalid_params(&msg);
            let (status, json) = error.to_response();
            assert_eq!(status, Status::BadRequest);
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[tokio::test]
async fn test_end_to_end_case_insensitive_lookup() {
    let variations = vec!["jakarta", "JAKARTA", "Jakarta", "JaKaRtA"];
    
    for city_input in variations {
        // Validate and find
        let validated = validate_city_input(city_input);
        assert!(validated.is_ok(), "Validation failed for: {}", city_input);
        
        let city_name = validated.unwrap();
        let city = find_city(&city_name);
        assert!(city.is_ok(), "City lookup failed for: {}", city_input);
        
        let city_data = city.unwrap();
        assert_eq!(city_data.name, "Jakarta");
    }
}

#[tokio::test]
async fn test_end_to_end_whitespace_handling() {
    let inputs = vec!["  Jakarta", "Jakarta  ", "  Jakarta  ", "\tJakarta\t"];
    
    for city_input in inputs {
        let validated = validate_city_input(city_input);
        assert!(validated.is_ok(), "Validation failed for: '{}'", city_input);
        
        let city_name = validated.unwrap();
        assert_eq!(city_name, "Jakarta", "Whitespace not trimmed properly");
        
        let city = find_city(&city_name);
        assert!(city.is_ok(), "City lookup failed after trimming");
    }
}

#[tokio::test]
async fn test_provider_error_response_format() {
    let error = ApiError::provider_error("All weather providers failed");
    let (status, json) = error.to_response();
    let response = json.into_inner();
    
    assert_eq!(status, Status::ServiceUnavailable);
    assert_eq!(response.error, "SERVICE_UNAVAILABLE");
    assert!(response.message.contains("weather providers"));
    assert!(!response.timestamp.is_empty());
}

#[tokio::test]
async fn test_multiple_error_types_have_correct_status_codes() {
    // Test 404
    let error_404 = ApiError::city_not_found("TestCity");
    let (status_404, _) = error_404.to_response();
    assert_eq!(status_404, Status::NotFound);
    
    // Test 400
    let error_400 = ApiError::invalid_params("Invalid param");
    let (status_400, _) = error_400.to_response();
    assert_eq!(status_400, Status::BadRequest);
    
    // Test 503
    let error_503 = ApiError::provider_error("Provider failed");
    let (status_503, _) = error_503.to_response();
    assert_eq!(status_503, Status::ServiceUnavailable);
    
    // Test timeout (also 503)
    let error_timeout = ApiError::Timeout;
    let (status_timeout, _) = error_timeout.to_response();
    assert_eq!(status_timeout, Status::ServiceUnavailable);
}

#[tokio::test]
async fn test_error_response_consistency() {
    let errors = vec![
        ApiError::city_not_found("City1"),
        ApiError::invalid_params("Param error"),
        ApiError::provider_error("Provider error"),
    ];
    
    for error in errors {
        let (_status, json) = error.to_response();
        let response = json.into_inner();
        
        // All responses should have these fields
        assert!(!response.error.is_empty(), "Error code should not be empty");
        assert!(!response.message.is_empty(), "Message should not be empty");
        assert!(!response.timestamp.is_empty(), "Timestamp should not be empty");
        assert!(response.timestamp.contains('T'), "Timestamp should be RFC3339 format");
    }
}

#[tokio::test]
async fn test_validation_chain_multiple_cities() {
    let test_cases = vec![
        ("Jakarta", true),
        ("Bandung", true),
        ("InvalidCity", false),
        ("", false),
        ("  Solo  ", true),
        ("A".repeat(51).as_str(), false),
    ];
    
    for (city_input, should_find) in test_cases {
        let validated = validate_city_input(city_input);
        
        if validated.is_err() {
            assert!(!should_find, "Validation should fail for: {}", city_input);
            continue;
        }
        
        let city_name = validated.unwrap();
        let result = find_city(&city_name);
        
        if should_find {
            assert!(result.is_ok(), "Should find city: {}", city_input);
        } else {
            assert!(result.is_err(), "Should not find city: {}", city_input);
        }
    }
}

#[tokio::test]
async fn test_error_messages_are_user_friendly() {
    // City not found
    let error1 = ApiError::city_not_found("Tokyo");
    let (_, json1) = error1.to_response();
    let response1 = json1.into_inner();
    assert!(response1.message.contains("Tokyo"));
    assert!(response1.message.contains("not found"));
    
    // Invalid input
    let error2 = ApiError::invalid_params("City name is required");
    let (_, json2) = error2.to_response();
    let response2 = json2.into_inner();
    assert_eq!(response2.message, "City name is required");
    
    // Provider error
    let error3 = ApiError::provider_error("Connection timeout");
    let (_, json3) = error3.to_response();
    let response3 = json3.into_inner();
    assert!(response3.message.contains("weather providers"));
    assert!(response3.message.contains("unavailable"));
}

#[tokio::test]
async fn test_no_stack_traces_in_error_responses() {
    let errors = vec![
        ApiError::city_not_found("Test"),
        ApiError::invalid_params("Test error"),
        ApiError::provider_error("Provider failed"),
        ApiError::Timeout,
    ];
    
    for error in errors {
        let (_, json) = error.to_response();
        let response = json.into_inner();
        
        // Error messages should not contain stack traces
        assert!(!response.message.contains("panicked"));
        assert!(!response.message.contains("stack backtrace"));
        assert!(!response.message.contains(".rs:"));
        assert!(!response.error.contains("panicked"));
    }
}
