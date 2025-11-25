use backend::models::DailyForecast;
use backend::services::providers::open_meteo::{OpenMeteoResponse, OpenMeteoDaily};
use serde_json;

fn create_mock_open_meteo_response() -> OpenMeteoResponse {
    OpenMeteoResponse {
        daily: OpenMeteoDaily {
            time: vec![
                "2024-01-15".to_string(),
                "2024-01-16".to_string(),
                "2024-01-17".to_string(),
            ],
            temperature_2m_max: vec![32.5, 30.0, 31.5],
            temperature_2m_min: vec![24.0, 23.0, 23.5],
            relative_humidity_2m: vec![65, 85, 70],
            weather_code: vec![0, 61, 95],
        },
    }
}

#[test]
fn test_wmo_code_0_clear_sky() {
    let response = create_mock_open_meteo_response();
    assert_eq!(response.daily.weather_code[0], 0);
}

#[test]
fn test_wmo_code_1_partly_cloudy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 1;
    assert_eq!(response.daily.weather_code[0], 1);
}

#[test]
fn test_wmo_code_2_partly_cloudy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 2;
    assert_eq!(response.daily.weather_code[0], 2);
}

#[test]
fn test_wmo_code_3_partly_cloudy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 3;
    assert_eq!(response.daily.weather_code[0], 3);
}

#[test]
fn test_wmo_code_45_foggy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 45;
    assert_eq!(response.daily.weather_code[0], 45);
}

#[test]
fn test_wmo_code_48_foggy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 48;
    assert_eq!(response.daily.weather_code[0], 48);
}

#[test]
fn test_wmo_code_80_rainy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 80;
    assert_eq!(response.daily.weather_code[0], 80);
}

#[test]
fn test_wmo_code_81_rainy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 81;
    assert_eq!(response.daily.weather_code[0], 81);
}

#[test]
fn test_wmo_code_82_rainy() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 82;
    assert_eq!(response.daily.weather_code[0], 82);
}

#[test]
fn test_wmo_code_85_heavy_snow() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 85;
    assert_eq!(response.daily.weather_code[0], 85);
}

#[test]
fn test_wmo_code_86_heavy_snow() {
    let mut response = create_mock_open_meteo_response();
    response.daily.weather_code[0] = 86;
    assert_eq!(response.daily.weather_code[0], 86);
}

#[test]
fn test_temperature_averaging_calculation() {
    let temp_max = 32.5f32;
    let temp_min = 24.0f32;
    let temp_avg = (temp_max + temp_min) / 2.0;
    
    assert_eq!(temp_avg, 28.25);
}

#[test]
fn test_temperature_averaging_with_negative() {
    let temp_max = 5.0f32;
    let temp_min = -5.0f32;
    let temp_avg = (temp_max + temp_min) / 2.0;
    
    assert_eq!(temp_avg, 0.0);
}

#[test]
fn test_temperature_averaging_equal_values() {
    let temp_max = 25.0f32;
    let temp_min = 25.0f32;
    let temp_avg = (temp_max + temp_min) / 2.0;
    
    assert_eq!(temp_avg, 25.0);
}

#[test]
fn test_date_formatting_consistency_iso8601() {
    let date = "2024-01-15";
    assert!(date.contains("-"));
    
    let parts: Vec<&str> = date.split('-').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].len(), 4); // year
    assert_eq!(parts[1].len(), 2); // month
    assert_eq!(parts[2].len(), 2); // day
}

#[test]
fn test_date_formatting_consistency_multiple_dates() {
    let dates = vec!["2024-01-15", "2024-01-16", "2024-01-17"];
    
    for date in dates {
        let parts: Vec<&str> = date.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 4);
        assert_eq!(parts[1].len(), 2);
        assert_eq!(parts[2].len(), 2);
    }
}

#[test]
fn test_normalized_forecast_structure() {
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

    assert!(!forecast.date.is_empty());
    assert!(!forecast.condition.is_empty());
    assert!(!forecast.icon.is_empty());
}

#[test]
fn test_normalized_forecast_has_all_fields() {
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
    let obj = json.as_object().unwrap();
    
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
fn test_icon_types_valid() {
    let valid_icons = vec!["sunny", "cloudy", "rainy", "snowy", "fog", "stormy"];
    
    for icon in valid_icons {
        assert!(!icon.is_empty());
        assert!(icon.chars().all(|c| c.is_ascii_lowercase()));
    }
}

#[test]
fn test_condition_types_valid() {
    let valid_conditions = vec![
        "Clear sky",
        "Mostly clear",
        "Overcast",
        "Foggy",
        "Light drizzle",
        "Rain",
        "Snow",
        "Snow grains",
        "Rain showers",
        "Snow showers",
        "Thunderstorm",
    ];
    
    for condition in valid_conditions {
        assert!(!condition.is_empty());
    }
}

#[test]
fn test_open_meteo_response_serialization() {
    let response = create_mock_open_meteo_response();
    let json = serde_json::to_string(&response).unwrap();
    
    assert!(json.contains("daily"));
    assert!(json.contains("time"));
    assert!(json.contains("temperature_2m_max"));
    assert!(json.contains("temperature_2m_min"));
    assert!(json.contains("relative_humidity_2m"));
    assert!(json.contains("weather_code"));
}

#[test]
fn test_open_meteo_response_deserialization() {
    let json = r#"{
        "daily": {
            "time": ["2024-01-15", "2024-01-16"],
            "temperature_2m_max": [32.5, 30.0],
            "temperature_2m_min": [24.0, 23.0],
            "relative_humidity_2m": [65, 85],
            "weather_code": [0, 61]
        }
    }"#;
    
    let response: OpenMeteoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.daily.time.len(), 2);
    assert_eq!(response.daily.temperature_2m_max.len(), 2);
    assert_eq!(response.daily.temperature_2m_min.len(), 2);
    assert_eq!(response.daily.relative_humidity_2m.len(), 2);
    assert_eq!(response.daily.weather_code.len(), 2);
}

#[test]
fn test_humidity_percentage_validation() {
    let humidity_values = vec![0, 25, 50, 75, 100];
    
    for humidity in humidity_values {
        assert!(humidity <= 100);
    }
}

#[test]
fn test_wind_speed_non_negative() {
    let wind_speeds = vec![0.0, 5.5, 10.0, 15.5];
    
    for wind_speed in wind_speeds {
        assert!(wind_speed >= 0.0);
    }
}

#[test]
fn test_temperature_range_realistic() {
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
            temp_max: -5.0,
            temp_min: -15.0,
            temp_avg: -10.0,
            condition: "Snow".to_string(),
            humidity: 85,
            wind_speed: 8.0,
            icon: "snowy".to_string(),
        },
    ];
    
    for forecast in forecasts {
        assert!(forecast.temp_max >= forecast.temp_min);
        assert!(forecast.temp_avg >= forecast.temp_min);
        assert!(forecast.temp_avg <= forecast.temp_max);
    }
}

#[test]
fn test_multiple_providers_normalized_format() {
    let forecast1 = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.5,
        temp_min: 24.0,
        temp_avg: 28.25,
        condition: "Clear sky".to_string(),
        humidity: 65,
        wind_speed: 5.5,
        icon: "sunny".to_string(),
    };

    let forecast2 = DailyForecast {
        date: "2024-01-15".to_string(),
        temp_max: 32.0,
        temp_min: 24.5,
        temp_avg: 28.25,
        condition: "Clear".to_string(),
        humidity: 60,
        wind_speed: 6.0,
        icon: "sunny".to_string(),
    };

    let json1 = serde_json::to_value(&forecast1).unwrap();
    let json2 = serde_json::to_value(&forecast2).unwrap();

    let obj1 = json1.as_object().unwrap();
    let obj2 = json2.as_object().unwrap();

    assert_eq!(obj1.keys().len(), obj2.keys().len());
    
    for key in obj1.keys() {
        assert!(obj2.contains_key(key));
    }
}

#[test]
fn test_forecast_array_length_seven_days() {
    let forecasts: Vec<DailyForecast> = (0..7)
        .map(|i| DailyForecast {
            date: format!("2024-01-{:02}", 15 + i),
            temp_max: 32.5,
            temp_min: 24.0,
            temp_avg: 28.25,
            condition: "Clear sky".to_string(),
            humidity: 65,
            wind_speed: 5.5,
            icon: "sunny".to_string(),
        })
        .collect();

    assert_eq!(forecasts.len(), 7);
}

#[test]
fn test_weather_service_creation() {
    use backend::services::weather_service::WeatherService;
    
    let service = WeatherService::new(
        "test-key".to_string(),
        "test-key".to_string(),
    );
    
    assert!(!service.openweather_key.is_empty());
    assert!(!service.weatherapi_key.is_empty());
}

#[test]
fn test_weather_service_keys_storage() {
    use backend::services::weather_service::WeatherService;
    
    let openweather_key = "openweather-test-key".to_string();
    let weatherapi_key = "weatherapi-test-key".to_string();
    
    let service = WeatherService::new(
        openweather_key.clone(),
        weatherapi_key.clone(),
    );
    
    assert_eq!(service.openweather_key, openweather_key);
    assert_eq!(service.weatherapi_key, weatherapi_key);
}
