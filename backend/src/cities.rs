use crate::models::City;
use once_cell::sync::Lazy;

// Embed JSON file at compile-time (zero runtime I/O cost!)
const CITIES_JSON: &str = include_str!("../data/cities.json");

// Parse JSON once at startup and cache (lazy initialization)
pub static CITIES: Lazy<Vec<City>> = Lazy::new(|| {
    serde_json::from_str(CITIES_JSON)
        .expect("Failed to parse cities.json - check data/cities.json for valid JSON format")
});
