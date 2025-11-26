use crate::test_utils::{TestClient, assertions::*};
use backend::models::EnsembleForecast;
use rocket::http::Status;
use serde_json::Value;
use std::time::Instant;
use chrono::NaiveDate;

// ============================================================================
// PART 1: Valid Request Tests
// ============================================================================

#[test]
fn test_current_week_valid_city_jakarta() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    // Verify 200 OK
    assert_ok(response.status());
    
    // Parse response
    let forecast: EnsembleForecast = response.json();
    
    // Validate structure
    assert_valid_ensemble(&forecast);
    assert_eq!(forecast.city, "Jakarta");
    assert_eq!(forecast.province, "DKI Jakarta");
    assert_eq!(forecast.country, "Indonesia");
    
    // Verify 7-day forecast
    assert_eq!(forecast.days.len(), 7, "Should have 7 days of forecast");
    
    // Validate each day
    for (i, day) in forecast.days.iter().enumerate() {
        assert_valid_day(day, i);
    }
    
    // Verify dates are in ISO format and sequential
    let dates: Vec<String> = forecast.days.iter().map(|d| d.date.clone()).collect();
    for (i, date) in dates.iter().enumerate() {
        assert_iso_date_format(date, &format!("Day {}", i));
    }
    assert_sequential_dates(&dates);
    
    // First date should be today
    assert_first_date_is_today(&dates[0]);
}

#[test]
fn test_current_week_valid_city_bandung() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Bandung");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    assert_eq!(forecast.city, "Bandung");
    assert_eq!(forecast.days.len(), 7);
    
    // Validate temperature logic
    for (i, day) in forecast.days.iter().enumerate() {
        let final_forecast = &day.final_forecast;
        assert!(
            final_forecast.temp_max > final_forecast.temp_min,
            "Day {} temp_max ({}) should be > temp_min ({})",
            i, final_forecast.temp_max, final_forecast.temp_min
        );
    }
}

#[test]
fn test_current_week_valid_city_surabaya() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Surabaya");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    assert_eq!(forecast.city, "Surabaya");
    
    // Check coordinates are present and valid
    assert!(forecast.latitude != 0.0, "Latitude should not be zero");
    assert!(forecast.longitude != 0.0, "Longitude should not be zero");
}

#[test]
fn test_next_week_valid_city_and_day_monday() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=next_week&day=0");
    
    // Verify 200 OK
    assert_ok(response.status());
    
    // Parse response
    let forecast: EnsembleForecast = response.json();
    
    // Should have 1 day for next week
    assert_eq!(forecast.days.len(), 1, "Next week forecast should have 1 day");
    
    // Validate the single day
    assert_valid_day(&forecast.days[0], 0);
    
    // Verify date is at least 7 days ahead
    let forecast_date = NaiveDate::parse_from_str(&forecast.days[0].date, "%Y-%m-%d")
        .expect("Failed to parse forecast date");
    let today = chrono::Local::now().naive_local().date();
    let diff = forecast_date.signed_duration_since(today).num_days();
    
    assert!(diff >= 7, "Next week date should be at least 7 days ahead, got {} days", diff);
}

#[test]
fn test_next_week_valid_city_and_day_friday() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Bandung&period=next_week&day=4");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    assert_eq!(forecast.days.len(), 1);
    
    // Check confidence is properly calculated
    let confidence = &forecast.days[0].final_forecast.confidence;
    assert!(
        ["high", "medium", "low"].contains(&confidence.as_str()),
        "Confidence should be high/medium/low, got: {}", confidence
    );
}

#[test]
fn test_case_insensitive_city_lowercase() {
    let client = TestClient::new();
    let mut response_lower = client.get("/api/weather/ensemble?city=jakarta");
    let mut response_upper = client.get("/api/weather/ensemble?city=JAKARTA");
    let mut response_mixed = client.get("/api/weather/ensemble?city=JaKaRtA");
    
    assert_ok(response_lower.status());
    assert_ok(response_upper.status());
    assert_ok(response_mixed.status());
    
    let forecast_lower: EnsembleForecast = response_lower.json();
    let forecast_upper: EnsembleForecast = response_upper.json();
    let forecast_mixed: EnsembleForecast = response_mixed.json();
    
    // All should return Jakarta
    assert_eq!(forecast_lower.city, "Jakarta");
    assert_eq!(forecast_upper.city, "Jakarta");
    assert_eq!(forecast_mixed.city, "Jakarta");
}

#[test]
fn test_all_valid_days_0_through_6() {
    let client = TestClient::new();
    
    for day in 0..=6 {
        let uri = format!("/api/weather/ensemble?city=Jakarta&period=next_week&day={}", day);
        let mut response = client.get(&uri);
        
        assert_ok(response.status());
        
        let forecast: EnsembleForecast = response.json();
        assert_eq!(forecast.days.len(), 1, "Day {} should return 1 day", day);
        
        // Verify date is correct for the weekday
        let forecast_date = NaiveDate::parse_from_str(&forecast.days[0].date, "%Y-%m-%d")
            .expect("Failed to parse forecast date");
        let today = chrono::Local::now().naive_local().date();
        let diff = forecast_date.signed_duration_since(today).num_days();
        assert!(diff >= 7, "Day {} should be at least 7 days ahead", day);
    }
}

#[test]
fn test_default_period_is_current_week() {
    let client = TestClient::new();
    
    // Without period parameter should default to current_week
    let mut response_default = client.get("/api/weather/ensemble?city=Jakarta");
    let mut response_explicit = client.get("/api/weather/ensemble?city=Jakarta&period=current_week");
    
    assert_ok(response_default.status());
    assert_ok(response_explicit.status());
    
    let forecast_default: EnsembleForecast = response_default.json();
    let forecast_explicit: EnsembleForecast = response_explicit.json();
    
    // Both should return 7 days
    assert_eq!(forecast_default.days.len(), 7);
    assert_eq!(forecast_explicit.days.len(), 7);
}

// ============================================================================
// PART 2: Error Scenario Tests
// ============================================================================

#[test]
fn test_city_not_found_404() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=InvalidCityName");
    
    assert_not_found(response.status());
    
    let json: Value = response.json_value();
    assert_error_response(&json, 404);
    
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(message.contains("InvalidCityName"), "Error message should mention the city name");
    assert!(message.contains("not found"), "Error message should mention 'not found'");
}

#[test]
fn test_empty_city_400() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=");
    
    assert_bad_request(response.status());
    
    let json: Value = response.json_value();
    assert_error_response(&json, 400);
    
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        message.contains("required") || message.contains("empty") || message.contains("cannot be empty"),
        "Error message should mention required or empty: {}", message
    );
}

#[test]
fn test_missing_city_parameter_400() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble");
    
    assert_bad_request(response.status());
    
    let json: Value = response.json_value();
    assert_error_response(&json, 400);
    
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        message.contains("required") || message.contains("Missing"),
        "Error message should mention required parameter: {}", message
    );
}

#[test]
fn test_next_week_missing_day_parameter_400() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=next_week");
    
    assert_bad_request(response.status());
    
    let json: Value = response.json_value();
    assert_error_response(&json, 400);
    
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(message.contains("Day"), "Error message should mention Day parameter");
    assert!(message.contains("required"), "Error message should mention required");
}

#[test]
fn test_invalid_day_greater_than_6() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=next_week&day=10");
    
    assert_bad_request(response.status());
    
    let json: Value = response.json_value();
    assert_error_response(&json, 400);
    
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(message.contains("0-6") || message.contains("Monday-Sunday"), 
        "Error message should mention valid day range: {}", message);
}

#[test]
fn test_invalid_day_value_7() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=next_week&day=7");
    
    assert_bad_request(response.status());
    
    let json: Value = response.json_value();
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(message.contains("0-6") || message.contains("must be"), 
        "Error should mention valid range");
}

#[test]
fn test_invalid_period_parameter() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=invalid_period");
    
    // Should either return 400 or default to current_week gracefully
    let status = response.status();
    assert!(
        status == Status::BadRequest || status == Status::Ok,
        "Should either reject invalid period (400) or default to current_week (200)"
    );
    
    if status == Status::Ok {
        let forecast: EnsembleForecast = response.json();
        // If it defaults, should return 7 days (current week)
        assert_eq!(forecast.days.len(), 7, "Invalid period should default to current week");
    }
}

#[test]
fn test_special_characters_in_city() {
    let client = TestClient::new();
    
    // Test various special characters
    let test_cases = vec![
        "/api/weather/ensemble?city=Jakarta<script>",
        "/api/weather/ensemble?city=Jakarta';DROP TABLE",
        "/api/weather/ensemble?city=../../../etc/passwd",
    ];
    
    for uri in test_cases {
        let mut response = client.get(uri);
        
        // Should return 400 (invalid input) or 404 (not found), never 200
        let status = response.status();
        assert!(
            status == Status::BadRequest || status == Status::NotFound,
            "Special characters should be rejected or not found: {} returned {}",
            uri, status
        );
    }
}

#[test]
fn test_city_name_too_long() {
    let client = TestClient::new();
    let long_city = "A".repeat(51);
    let uri = format!("/api/weather/ensemble?city={}", long_city);
    let mut response = client.get(&uri);
    
    assert_bad_request(response.status());
    
    let json: Value = response.json_value();
    let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        message.contains("50 characters") || message.contains("too long"),
        "Error should mention length limit: {}", message
    );
}

// ============================================================================
// PART 3: Response Contract Tests
// ============================================================================

#[test]
fn test_response_has_all_required_fields() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let json: Value = response.json_value();
    
    // Top-level required fields
    assert!(json.get("city").is_some(), "Response should have 'city' field");
    assert!(json.get("province").is_some(), "Response should have 'province' field");
    assert!(json.get("country").is_some(), "Response should have 'country' field");
    assert!(json.get("latitude").is_some(), "Response should have 'latitude' field");
    assert!(json.get("longitude").is_some(), "Response should have 'longitude' field");
    assert!(json.get("source_timestamp").is_some(), "Response should have 'source_timestamp' field");
    assert!(json.get("days").is_some(), "Response should have 'days' field");
    
    // Validate days array
    let days = json.get("days").and_then(|v| v.as_array()).expect("days should be an array");
    assert_eq!(days.len(), 7, "Should have 7 days");
    
    // Check first day structure
    let first_day = &days[0];
    assert!(first_day.get("date").is_some(), "Day should have 'date' field");
    assert!(first_day.get("per_source").is_some(), "Day should have 'per_source' field");
    assert!(first_day.get("final_forecast").is_some(), "Day should have 'final_forecast' field");
    
    // Check per_source structure
    let per_source = first_day.get("per_source").unwrap();
    assert!(per_source.is_object(), "per_source should be an object");
    
    // Check final_forecast structure
    let final_forecast = first_day.get("final_forecast").unwrap();
    assert!(final_forecast.get("temp_max").is_some(), "final_forecast should have 'temp_max'");
    assert!(final_forecast.get("temp_min").is_some(), "final_forecast should have 'temp_min'");
    assert!(final_forecast.get("condition").is_some(), "final_forecast should have 'condition'");
    assert!(final_forecast.get("confidence").is_some(), "final_forecast should have 'confidence'");
}

#[test]
fn test_no_null_values_in_required_fields() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let json: Value = response.json_value();
    
    // Check top-level fields are not null
    assert!(!json.get("city").unwrap().is_null(), "city should not be null");
    assert!(!json.get("province").unwrap().is_null(), "province should not be null");
    assert!(!json.get("country").unwrap().is_null(), "country should not be null");
    assert!(!json.get("latitude").unwrap().is_null(), "latitude should not be null");
    assert!(!json.get("longitude").unwrap().is_null(), "longitude should not be null");
    
    // Check days array elements
    let days = json.get("days").and_then(|v| v.as_array()).unwrap();
    for (i, day) in days.iter().enumerate() {
        assert!(!day.get("date").unwrap().is_null(), "Day {} date should not be null", i);
        assert!(!day.get("final_forecast").unwrap().is_null(), "Day {} final_forecast should not be null", i);
    }
}

#[test]
fn test_proper_data_types() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let json: Value = response.json_value();
    
    // String types
    assert!(json.get("city").unwrap().is_string(), "city should be string");
    assert!(json.get("province").unwrap().is_string(), "province should be string");
    assert!(json.get("country").unwrap().is_string(), "country should be string");
    assert!(json.get("source_timestamp").unwrap().is_string(), "source_timestamp should be string");
    
    // Number types
    assert!(json.get("latitude").unwrap().is_number(), "latitude should be number");
    assert!(json.get("longitude").unwrap().is_number(), "longitude should be number");
    
    // Array type
    assert!(json.get("days").unwrap().is_array(), "days should be array");
    
    // Check day structure types
    let days = json.get("days").and_then(|v| v.as_array()).unwrap();
    let first_day = &days[0];
    
    assert!(first_day.get("date").unwrap().is_string(), "date should be string");
    assert!(first_day.get("per_source").unwrap().is_object(), "per_source should be object");
    assert!(first_day.get("final_forecast").unwrap().is_object(), "final_forecast should be object");
    
    // Check final_forecast types
    let final_forecast = first_day.get("final_forecast").unwrap();
    assert!(final_forecast.get("temp_max").unwrap().is_number(), "temp_max should be number");
    assert!(final_forecast.get("temp_min").unwrap().is_number(), "temp_min should be number");
    assert!(final_forecast.get("condition").unwrap().is_string(), "condition should be string");
    assert!(final_forecast.get("confidence").unwrap().is_string(), "confidence should be string");
}

#[test]
fn test_dates_in_iso_format() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    for (i, day) in forecast.days.iter().enumerate() {
        assert_iso_date_format(&day.date, &format!("Day {}", i));
    }
}

#[test]
fn test_temperatures_are_valid_numbers() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    for (i, day) in forecast.days.iter().enumerate() {
        let final_forecast = &day.final_forecast;
        
        // Check temperatures are valid f32
        assert!(final_forecast.temp_max.is_finite(), "Day {} temp_max should be finite", i);
        assert!(final_forecast.temp_min.is_finite(), "Day {} temp_min should be finite", i);
        
        // Check temperatures are in reasonable range for Indonesia
        assert!(final_forecast.temp_max >= 15.0 && final_forecast.temp_max <= 45.0,
            "Day {} temp_max should be in reasonable range: {}", i, final_forecast.temp_max);
        assert!(final_forecast.temp_min >= 15.0 && final_forecast.temp_min <= 45.0,
            "Day {} temp_min should be in reasonable range: {}", i, final_forecast.temp_min);
    }
}

#[test]
fn test_confidence_values_are_valid() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    for (i, day) in forecast.days.iter().enumerate() {
        let confidence = &day.final_forecast.confidence;
        assert!(
            ["high", "medium", "low"].contains(&confidence.as_str()),
            "Day {} confidence should be high/medium/low, got: {}", i, confidence
        );
    }
}

#[test]
fn test_per_source_data_validation() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    for (i, day) in forecast.days.iter().enumerate() {
        let per_source = &day.per_source;
        
        // At least one provider should have data
        let has_data = per_source.open_meteo.is_some() 
            || per_source.open_weather.is_some() 
            || per_source.weather_api.is_some();
        assert!(has_data, "Day {} should have data from at least one provider", i);
        
        // Validate provider data if present
        if let Some(ref om) = per_source.open_meteo {
            assert!(om.temp_max >= om.temp_min, "Day {} Open-Meteo temp_max >= temp_min", i);
            assert!(!om.condition.is_empty(), "Day {} Open-Meteo condition not empty", i);
        }
        
        if let Some(ref ow) = per_source.open_weather {
            assert!(ow.temp_max >= ow.temp_min, "Day {} OpenWeather temp_max >= temp_min", i);
            assert!(!ow.condition.is_empty(), "Day {} OpenWeather condition not empty", i);
        }
        
        if let Some(ref wa) = per_source.weather_api {
            assert!(wa.temp_max >= wa.temp_min, "Day {} WeatherAPI temp_max >= temp_min", i);
            assert!(!wa.condition.is_empty(), "Day {} WeatherAPI condition not empty", i);
        }
    }
}

#[test]
fn test_final_forecast_validation() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    for (i, day) in forecast.days.iter().enumerate() {
        let final_forecast = &day.final_forecast;
        
        // temp_max > temp_min
        assert!(
            final_forecast.temp_max > final_forecast.temp_min,
            "Day {} temp_max ({}) should be > temp_min ({})",
            i, final_forecast.temp_max, final_forecast.temp_min
        );
        
        // Valid confidence
        assert!(
            ["high", "medium", "low"].contains(&final_forecast.confidence.as_str()),
            "Day {} confidence should be valid: {}", i, final_forecast.confidence
        );
        
        // Non-empty condition
        assert!(
            !final_forecast.condition.is_empty(),
            "Day {} condition should not be empty", i
        );
    }
}

#[test]
fn test_date_consistency_sequential_days() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    let dates: Vec<String> = forecast.days.iter().map(|d| d.date.clone()).collect();
    assert_sequential_dates(&dates);
}

#[test]
fn test_date_consistency_first_date_is_today() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=current_week");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    assert_first_date_is_today(&forecast.days[0].date);
}

#[test]
fn test_date_consistency_next_week_is_d_plus_7() {
    let client = TestClient::new();
    let mut response = client.get("/api/weather/ensemble?city=Jakarta&period=next_week&day=0");
    
    assert_ok(response.status());
    
    let forecast: EnsembleForecast = response.json();
    
    // Calculate expected date (at least 7 days from today)
    let forecast_date = NaiveDate::parse_from_str(&forecast.days[0].date, "%Y-%m-%d")
        .expect("Failed to parse forecast date");
    let today = chrono::Local::now().naive_local().date();
    let diff = forecast_date.signed_duration_since(today).num_days();
    
    assert!(diff >= 7 && diff <= 14, 
        "Next week date should be 7-14 days ahead (D+7), got {} days", diff);
}

// ============================================================================
// PART 4: Performance Tests
// ============================================================================

#[test]
fn test_cache_effectiveness_second_call_faster() {
    let client = TestClient::new();
    
    // First call (cache miss)
    let start1 = Instant::now();
    let mut response1 = client.get("/api/weather/ensemble?city=Jakarta");
    let duration1 = start1.elapsed();
    assert_ok(response1.status());
    let _forecast1: EnsembleForecast = response1.json();
    
    // Small delay to ensure first request completes
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Second call (should hit cache)
    let start2 = Instant::now();
    let mut response2 = client.get("/api/weather/ensemble?city=Jakarta");
    let duration2 = start2.elapsed();
    assert_ok(response2.status());
    let _forecast2: EnsembleForecast = response2.json();
    
    // Second call should be faster (cached)
    println!("First call: {:?}, Second call: {:?}", duration1, duration2);
    assert!(
        duration2 < duration1,
        "Second call should be faster due to cache: {:?} vs {:?}",
        duration2, duration1
    );
}

#[test]
fn test_cached_request_response_time() {
    let client = TestClient::new();
    
    // Prime the cache
    let mut response_prime = client.get("/api/weather/ensemble?city=Bandung");
    assert_ok(response_prime.status());
    let _: EnsembleForecast = response_prime.json();
    
    // Small delay
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    // Measure cached request
    let start = Instant::now();
    let mut response = client.get("/api/weather/ensemble?city=Bandung");
    let duration = start.elapsed();
    assert_ok(response.status());
    let _: EnsembleForecast = response.json();
    
    // Cached requests should be very fast (< 500ms)
    println!("Cached request took: {:?}", duration);
    assert!(
        duration.as_millis() < 500,
        "Cached request should be < 500ms, took {:?}",
        duration
    );
}

#[test]
fn test_fresh_request_response_time() {
    let client = TestClient::new();
    
    // Use a unique city to avoid cache hits
    let start = Instant::now();
    let mut response = client.get("/api/weather/ensemble?city=Surabaya");
    let duration = start.elapsed();
    
    assert_ok(response.status());
    let _: EnsembleForecast = response.json();
    
    // Fresh requests should complete in reasonable time (< 2000ms)
    // This might take longer on first call due to provider fetching
    println!("Fresh request took: {:?}", duration);
    assert!(
        duration.as_secs() < 5,
        "Fresh request should be < 5s, took {:?}",
        duration
    );
}

#[test]
fn test_multiple_cities_cache_independence() {
    let client = TestClient::new();
    
    let cities = vec!["Jakarta", "Bandung", "Surabaya"];
    
    for city in cities {
        let uri = format!("/api/weather/ensemble?city={}", city);
        let mut response = client.get(&uri);
        
        assert_ok(response.status());
        let forecast: EnsembleForecast = response.json();
        assert_eq!(forecast.city, city, "Cached city should match request");
    }
}
