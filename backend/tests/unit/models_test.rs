use backend::models::{DailyForecast, WeatherForecast, ApiResponse};
use serde_json;

#[test]
fn test_daily_forecast_creation() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    assert_eq!(forecast.date, "2024-01-15");
    assert_eq!(forecast.temp_max, 32.5);
    assert_eq!(forecast.temp_min, 24.0);
    assert_eq!(forecast.temp_avg, 28.25);
    assert_eq!(forecast.condition, "Clear sky");
    assert_eq!(forecast.humidity, 65);
    assert_eq!(forecast.wind_speed, 5.5);
    assert_eq!(forecast.icon, "sunny");
}

#[test]
fn test_weather_forecast_creation() {
    let forecast = WeatherForecast {
        city: "Jakarta".to_string(),
        province: "DKI Jakarta".to_string(),
        country: "Indonesia".to_string(),
        latitude: -6.2088,
        longitude: 106.8456,
        last_updated: "2024-01-15T10:00:00+00:00".to_string(),
        forecast: vec![],
    };

    assert_eq!(forecast.city, "Jakarta");
    assert_eq!(forecast.province, "DKI Jakarta");
    assert_eq!(forecast.country, "Indonesia");
    assert_eq!(forecast.latitude, -6.2088);
    assert_eq!(forecast.longitude, 106.8456);
    assert_eq!(forecast.forecast.len(), 0);
}

#[test]
fn test_daily_forecast_serialization() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    let json = serde_json::to_string(&forecast).unwrap();
    assert!(json.contains("\"date\":\"2024-01-15\""));
    assert!(json.contains("\"temp_max\":32.5"));
    assert!(json.contains("\"temp_min\":24.0"));
    assert!(json.contains("\"temp_avg\":28.25"));
    assert!(json.contains("\"condition\":\"Clear sky\""));
    assert!(json.contains("\"humidity\":65"));
    assert!(json.contains("\"wind_speed\":5.5"));
    assert!(json.contains("\"icon\":\"sunny\""));
}

#[test]
fn test_daily_forecast_deserialization() {
    let json = r#"{
        "date": "2024-01-15",
        "temp_max": 32.5,
        "temp_min": 24.0,
        "temp_avg": 28.25,
        "condition": "Clear sky",
        "humidity": 65,
        "wind_speed": 5.5,
        "icon": "sunny"
    }"#;

    let forecast: DailyForecast = serde_json::from_str(json).unwrap();
    assert_eq!(forecast.date, "2024-01-15");
    assert_eq!(forecast.temp_max, 32.5);
    assert_eq!(forecast.temp_min, 24.0);
    assert_eq!(forecast.temp_avg, 28.25);
    assert_eq!(forecast.condition, "Clear sky");
    assert_eq!(forecast.humidity, 65);
    assert_eq!(forecast.wind_speed, 5.5);
    assert_eq!(forecast.icon, "sunny");
}

#[test]
fn test_weather_forecast_serialization() {
    let daily_forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    let forecast = WeatherForecast {
        city: "Jakarta".to_string(),
        province: "DKI Jakarta".to_string(),
        country: "Indonesia".to_string(),
        latitude: -6.2088,
        longitude: 106.8456,
        last_updated: "2024-01-15T10:00:00+00:00".to_string(),
        forecast: vec![daily_forecast],
    };

    let json = serde_json::to_string(&forecast).unwrap();
    assert!(json.contains("\"city\":\"Jakarta\""));
    assert!(json.contains("\"province\":\"DKI Jakarta\""));
    assert!(json.contains("\"country\":\"Indonesia\""));
    assert!(json.contains("\"latitude\":-6.2088"));
    assert!(json.contains("\"longitude\":106.8456"));
    assert!(json.contains("\"forecast\":["));
}

#[test]
fn test_weather_forecast_deserialization() {
    let json = r#"{
        "city": "Jakarta",
        "province": "DKI Jakarta",
        "country": "Indonesia",
        "latitude": -6.2088,
        "longitude": 106.8456,
        "last_updated": "2024-01-15T10:00:00+00:00",
        "forecast": [{
            "date": "2024-01-15",
            "temp_max": 32.5,
            "temp_min": 24.0,
            "temp_avg": 28.25,
            "condition": "Clear sky",
            "humidity": 65,
            "wind_speed": 5.5,
            "icon": "sunny"
        }]
    }"#;

    let forecast: WeatherForecast = serde_json::from_str(json).unwrap();
    assert_eq!(forecast.city, "Jakarta");
    assert_eq!(forecast.province, "DKI Jakarta");
    assert_eq!(forecast.country, "Indonesia");
    assert_eq!(forecast.latitude, -6.2088);
    assert_eq!(forecast.longitude, 106.8456);
    assert_eq!(forecast.forecast.len(), 1);
    assert_eq!(forecast.forecast[0].date, "2024-01-15");
}

#[test]
fn test_temperature_validation_max_greater_than_min() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    assert!(forecast.temp_max > forecast.temp_min, "temp_max should be greater than temp_min");
}

#[test]
fn test_temperature_validation_avg_between_min_max() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    assert!(forecast.temp_avg >= forecast.temp_min, "temp_avg should be >= temp_min");
    assert!(forecast.temp_avg <= forecast.temp_max, "temp_avg should be <= temp_max");
}

#[test]
fn test_humidity_validation_within_range() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    assert!(forecast.humidity <= 100, "humidity should be <= 100");
}

#[test]
fn test_humidity_edge_case_zero() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 0,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    assert_eq!(forecast.humidity, 0);
}

#[test]
fn test_humidity_edge_case_hundred() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 100,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    assert_eq!(forecast.humidity, 100);
}

#[test]
fn test_api_response_success() {
    let response: ApiResponse<String> = ApiResponse::success("test data".to_string());
    assert!(response.success);
    assert_eq!(response.data, Some("test data".to_string()));
    assert!(response.error.is_none());
    assert!(response.message.is_none());
}

#[test]
fn test_api_response_error() {
    let response: ApiResponse<String> = ApiResponse::error("test error".to_string());
    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("test error".to_string()));
    assert!(response.message.is_none());
}

#[test]
fn test_json_format_matches_expected() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    let json = serde_json::to_value(&forecast).unwrap();
    assert!(json.is_object());
    
    let obj = json.as_object().unwrap();
    assert_eq!(obj.len(), 8);
    assert!(obj.contains_key("date"));
    assert!(obj.contains_key("temp_max"));
    assert!(obj.contains_key("temp_min"));
    assert!(obj.contains_key("temp_avg"));
    assert!(obj.contains_key("condition"));
    assert!(obj.contains_key("humidity"));
    assert!(obj.contains_key("wind_speed"));
    assert!(obj.contains_key("icon"));
}

#[test]
fn test_weather_forecast_with_multiple_days() {
    let forecasts = vec![
        DailyForecast {
            date: "2024-01-15".to_string(),
            temp_max: 32.5,
            temp_min: 24.0,
            temp_avg: 28.25,
            condition: "Clear sky".to_string(),
            humidity: 65,
            wind_speed: 5.5,
            icon: "sunny".to_string(),
        },
        DailyForecast {
            date: "2024-01-16".to_string(),
            temp_max: 30.0,
            temp_min: 23.0,
            temp_avg: 26.5,
            condition: "Rainy".to_string(),
            humidity: 85,
            wind_speed: 8.0,
            icon: "rainy".to_string(),
        },
    ];

    let weather = WeatherForecast {
        city: "Jakarta".to_string(),
        province: "DKI Jakarta".to_string(),
        country: "Indonesia".to_string(),
        latitude: -6.2088,
        longitude: 106.8456,
        last_updated: "2024-01-15T10:00:00+00:00".to_string(),
        forecast: forecasts,
    };

    assert_eq!(weather.forecast.len(), 2);
    assert_eq!(weather.forecast[0].date, "2024-01-15");
    assert_eq!(weather.forecast[1].date, "2024-01-16");
}

#[test]
fn test_daily_forecast_clone() {
    let forecast = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    let cloned = forecast.clone();
    assert_eq!(forecast.date, cloned.date);
    assert_eq!(forecast.temp_max, cloned.temp_max);
    assert_eq!(forecast.condition, cloned.condition);
}

#[test]
fn test_weather_forecast_clone() {
    let forecast = WeatherForecast {
        city: "Jakarta".to_string(),
        province: "DKI Jakarta".to_string(),
        country: "Indonesia".to_string(),
        latitude: -6.2088,
        longitude: 106.8456,
        last_updated: "2024-01-15T10:00:00+00:00".to_string(),
        forecast: vec![],
    };

    let cloned = forecast.clone();
    assert_eq!(forecast.city, cloned.city);
    assert_eq!(forecast.latitude, cloned.latitude);
    assert_eq!(forecast.longitude, cloned.longitude);
}
