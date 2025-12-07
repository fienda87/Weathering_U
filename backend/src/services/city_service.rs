use crate::models::City;
use crate::cities::CITIES;
use crate::errors::ApiError;
use log::debug;

/// Finds a city by name in the database (case-insensitive)
/// 
/// # Arguments
/// * `name` - The city name to search for
/// 
/// # Returns
/// * `Ok(&City)` - Reference to the found city
/// * `Err(ApiError)` - CityNotFound error if not found
/// 
/// # Example
/// ```
/// let city = find_city("Jakarta")?;
/// println!("Found city: {} at ({}, {})", city.name, city.latitude, city.longitude);
/// ```
pub fn find_city(name: &str) -> Result<City, ApiError> {
    let query = name.trim().to_lowercase();
    
    debug!("[CityService] Searching for city: '{}'", query);
    
    CITIES.iter()
        .find(|city| city.name.to_lowercase() == query)
        .cloned()
        .ok_or_else(|| {
            debug!("[CityService] City not found: '{}'", name);
            ApiError::city_not_found(name)
        })
}

/// Validates city name input before searching
/// 
/// # Arguments
/// * `city` - The city name to validate
/// 
/// # Returns
/// * `Ok(String)` - Trimmed, validated city name
/// * `Err(ApiError)` - InvalidInput error if validation fails
/// 
/// # Validation Rules
/// * City name cannot be empty or only whitespace
/// * City name must not exceed 50 characters
pub fn validate_city_input(city: &str) -> Result<String, ApiError> {
    let trimmed = city.trim();
    
    if trimmed.is_empty() {
        return Err(ApiError::invalid_params("City name is required"));
    }
    
    if trimmed.len() > 50 {
        return Err(ApiError::invalid_params(
            "City name must not exceed 50 characters"
        ));
    }
    
    Ok(trimmed.to_string())
}

/// Gets all available cities from the database
/// 
/// # Returns
/// * `Vec<City>` - Vector of all cities (cloned from static data)
pub fn get_all_cities() -> Vec<City> {
    CITIES.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_city_success() {
        let result = find_city("Jakarta");
        assert!(result.is_ok());
        let city = result.unwrap();
        assert_eq!(city.name, "Jakarta");
        assert_eq!(city.province, "DKI Jakarta");
    }

    #[test]
    fn test_find_city_case_insensitive() {
        let result = find_city("jakarta");
        assert!(result.is_ok());
        let city = result.unwrap();
        assert_eq!(city.name, "Jakarta");
    }

    #[test]
    fn test_find_city_with_whitespace() {
        let result = find_city("  Bandung  ");
        assert!(result.is_ok());
        let city = result.unwrap();
        assert_eq!(city.name, "Bandung");
    }

    #[test]
    fn test_find_city_not_found() {
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
    fn test_validate_city_input_empty() {
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
    fn test_validate_city_input_only_whitespace() {
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
    fn test_get_all_cities() {
        let cities = get_all_cities();
        assert!(!cities.is_empty());
        assert!(cities.len() >= 50);
    }

    #[test]
    fn test_get_city_count() {
        let cities = get_all_cities();
        assert!(cities.len() >= 50);
        assert_eq!(cities.len(), CITIES.len());
    }
}
