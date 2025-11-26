use backend::services::{find_city, validate_city_input, get_all_cities, get_city_count};
use backend::errors::ApiError;

#[test]
fn test_find_city_success_exact_match() {
    let result = find_city("Jakarta");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Jakarta");
    assert_eq!(city.province, "DKI Jakarta");
    assert!(city.latitude < 0.0); // Jakarta is in southern hemisphere
}

#[test]
fn test_find_city_success_case_insensitive_lowercase() {
    let result = find_city("jakarta");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Jakarta");
}

#[test]
fn test_find_city_success_case_insensitive_uppercase() {
    let result = find_city("BANDUNG");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Bandung");
}

#[test]
fn test_find_city_success_mixed_case() {
    let result = find_city("SuRaBaYa");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Surabaya");
}

#[test]
fn test_find_city_success_with_leading_whitespace() {
    let result = find_city("  Medan");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Medan");
}

#[test]
fn test_find_city_success_with_trailing_whitespace() {
    let result = find_city("Makassar  ");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Makassar");
}

#[test]
fn test_find_city_success_with_surrounding_whitespace() {
    let result = find_city("  Semarang  ");
    assert!(result.is_ok());
    let city = result.unwrap();
    assert_eq!(city.name, "Semarang");
}

#[test]
fn test_find_city_not_found_invalid_city() {
    let result = find_city("InvalidCity");
    assert!(result.is_err());
    match result {
        Err(ApiError::CityNotFound(name)) => {
            assert_eq!(name, "InvalidCity");
        }
        _ => panic!("Expected CityNotFound error"),
    }
}

#[test]
fn test_find_city_not_found_empty_string() {
    let result = find_city("");
    assert!(result.is_err());
    match result {
        Err(ApiError::CityNotFound(_)) => {}
        _ => panic!("Expected CityNotFound error"),
    }
}

#[test]
fn test_find_city_not_found_whitespace_only() {
    let result = find_city("   ");
    assert!(result.is_err());
}

#[test]
fn test_find_city_not_found_partial_match() {
    let result = find_city("Jak"); // Partial match should not work
    assert!(result.is_err());
}

#[test]
fn test_find_city_coordinates_valid() {
    let result = find_city("Yogyakarta");
    assert!(result.is_ok());
    let city = result.unwrap();
    
    // Yogyakarta coordinates should be reasonable
    assert!(city.latitude < 0.0); // Southern hemisphere
    assert!(city.longitude > 100.0 && city.longitude < 120.0); // Indonesia longitude range
}

#[test]
fn test_validate_city_input_success() {
    let result = validate_city_input("Jakarta");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Jakarta");
}

#[test]
fn test_validate_city_input_trims_whitespace() {
    let result = validate_city_input("  Bandung  ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Bandung");
}

#[test]
fn test_validate_city_input_empty_string() {
    let result = validate_city_input("");
    assert!(result.is_err());
    match result {
        Err(ApiError::InvalidInput(msg)) => {
            assert_eq!(msg, "City name is required");
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_validate_city_input_whitespace_only() {
    let result = validate_city_input("   ");
    assert!(result.is_err());
    match result {
        Err(ApiError::InvalidInput(msg)) => {
            assert_eq!(msg, "City name is required");
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_validate_city_input_too_long() {
    let long_name = "A".repeat(51);
    let result = validate_city_input(&long_name);
    assert!(result.is_err());
    match result {
        Err(ApiError::InvalidInput(msg)) => {
            assert_eq!(msg, "City name must not exceed 50 characters");
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_validate_city_input_exactly_50_chars() {
    let name = "A".repeat(50);
    let result = validate_city_input(&name);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 50);
}

#[test]
fn test_validate_city_input_preserves_case() {
    let result = validate_city_input("JaKaRtA");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "JaKaRtA"); // Case should be preserved
}

#[test]
fn test_get_all_cities_returns_cities() {
    let cities = get_all_cities();
    assert!(!cities.is_empty());
}

#[test]
fn test_get_all_cities_has_expected_count() {
    let cities = get_all_cities();
    assert!(cities.len() >= 50, "Expected at least 50 cities, got {}", cities.len());
}

#[test]
fn test_get_all_cities_contains_jakarta() {
    let cities = get_all_cities();
    let jakarta = cities.iter().find(|c| c.name == "Jakarta");
    assert!(jakarta.is_some(), "Jakarta should be in the cities list");
}

#[test]
fn test_get_all_cities_all_have_ids() {
    let cities = get_all_cities();
    for city in cities {
        assert!(city.id > 0, "City {} should have a valid ID", city.name);
    }
}

#[test]
fn test_get_all_cities_all_have_coordinates() {
    let cities = get_all_cities();
    for city in cities {
        assert!(city.latitude.abs() <= 90.0, "City {} has invalid latitude", city.name);
        assert!(city.longitude.abs() <= 180.0, "City {} has invalid longitude", city.name);
    }
}

#[test]
fn test_get_city_count_returns_positive() {
    let count = get_city_count();
    assert!(count > 0, "City count should be positive");
}

#[test]
fn test_get_city_count_matches_cities_length() {
    let count = get_city_count();
    let cities = get_all_cities();
    assert_eq!(count, cities.len(), "City count should match cities array length");
}

#[test]
fn test_find_city_returns_static_reference() {
    let result1 = find_city("Jakarta");
    let result2 = find_city("Jakarta");
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    let city1 = result1.unwrap();
    let city2 = result2.unwrap();
    
    // Should be the same static reference
    assert_eq!(city1.name, city2.name);
    assert_eq!(city1.id, city2.id);
}

#[test]
fn test_validate_and_find_city_integration() {
    let input = "  jakarta  ";
    
    // First validate
    let validated = validate_city_input(input);
    assert!(validated.is_ok());
    
    // Then find
    let found = find_city(&validated.unwrap());
    assert!(found.is_ok());
    
    let city = found.unwrap();
    assert_eq!(city.name, "Jakarta");
}

#[test]
fn test_find_multiple_different_cities() {
    let cities_to_find = vec!["Jakarta", "Bandung", "Surabaya", "Medan", "Makassar"];
    
    for city_name in cities_to_find {
        let result = find_city(city_name);
        assert!(result.is_ok(), "Should find city: {}", city_name);
        let city = result.unwrap();
        assert_eq!(city.name, city_name);
    }
}

#[test]
fn test_validate_city_with_special_characters() {
    // Test with numbers and symbols (should still work as long as not too long)
    let result = validate_city_input("City-123");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "City-123");
}

#[test]
fn test_find_city_case_sensitivity_all_variations() {
    let variations = vec!["SOLO", "solo", "Solo", "SoLo", "sOlO"];
    
    for variation in variations {
        let result = find_city(variation);
        assert!(result.is_ok(), "Should find city with variation: {}", variation);
        let city = result.unwrap();
        assert_eq!(city.name, "Solo");
    }
}
