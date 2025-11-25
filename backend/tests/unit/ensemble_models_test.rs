use backend::models::ensemble::*;

#[test]
fn test_provider_forecast_creation() {
    let forecast = ProviderForecast::new(
        "2025-11-26".to_string(),
        32.0,
        25.0,
        "Cloudy".to_string(),
    );

    assert_eq!(forecast.date, "2025-11-26");
    assert_eq!(forecast.temp_max, 32.0);
    assert_eq!(forecast.temp_min, 25.0);
    assert_eq!(forecast.condition, "Cloudy");
}

#[test]
fn test_per_source_data_builder() {
    let om = ProviderForecast::new(
        "2025-11-26".to_string(),
        32.0,
        25.0,
        "Cloudy".to_string(),
    );

    let ow = ProviderForecast::new(
        "2025-11-26".to_string(),
        33.0,
        25.5,
        "Cloudy".to_string(),
    );

    let per_source = PerSourceData::new()
        .with_open_meteo(om)
        .with_open_weather(ow);

    assert!(per_source.open_meteo.is_some());
    assert!(per_source.open_weather.is_some());
    assert!(per_source.weather_api.is_none());
    assert_eq!(per_source.provider_count(), 2);
}

#[test]
fn test_per_source_provider_count() {
    let per_source = PerSourceData::new()
        .with_open_meteo(ProviderForecast::new(
            "2025-11-26".to_string(),
            32.0,
            25.0,
            "Cloudy".to_string(),
        ))
        .with_open_weather(ProviderForecast::new(
            "2025-11-26".to_string(),
            33.0,
            25.5,
            "Cloudy".to_string(),
        ))
        .with_weather_api(ProviderForecast::new(
            "2025-11-26".to_string(),
            31.5,
            24.9,
            "Rain".to_string(),
        ));

    assert_eq!(per_source.provider_count(), 3);
}

#[test]
fn test_final_forecast_creation() {
    let final_forecast = FinalForecast::new(
        32.1,
        25.1,
        "Cloudy".to_string(),
        "medium".to_string(),
    );

    assert_eq!(final_forecast.temp_max, 32.1);
    assert_eq!(final_forecast.temp_min, 25.1);
    assert_eq!(final_forecast.condition, "Cloudy");
    assert_eq!(final_forecast.confidence, "medium");
}

#[test]
fn test_day_ensemble_creation() {
    let per_source = PerSourceData::new();
    let final_forecast = FinalForecast::new(
        32.1,
        25.1,
        "Cloudy".to_string(),
        "medium".to_string(),
    );

    let day = DayEnsemble::new("2025-11-26".to_string(), per_source, final_forecast);

    assert_eq!(day.date, "2025-11-26");
    assert_eq!(day.per_source.provider_count(), 0);
}

#[test]
fn test_ensemble_forecast_creation() {
    let mut forecast = EnsembleForecast::new(
        "Jakarta".to_string(),
        "DKI Jakarta".to_string(),
        "Indonesia".to_string(),
        -6.2,
        106.8,
    );

    assert_eq!(forecast.city, "Jakarta");
    assert_eq!(forecast.province, "DKI Jakarta");
    assert_eq!(forecast.days.len(), 0);

    // Add days
    for i in 0..7 {
        let per_source = PerSourceData::new();
        let final_forecast = FinalForecast::new(
            32.0,
            25.0,
            "Cloudy".to_string(),
            "high".to_string(),
        );
        let day = DayEnsemble::new(
            format!("2025-11-{:02}", 26 + i),
            per_source,
            final_forecast,
        );
        forecast.add_day(day);
    }

    assert_eq!(forecast.days.len(), 7);
}

#[test]
fn test_ensemble_forecast_serialization() {
    let mut forecast = EnsembleForecast::new(
        "Jakarta".to_string(),
        "DKI Jakarta".to_string(),
        "Indonesia".to_string(),
        -6.2,
        106.8,
    );

    let per_source = PerSourceData::new();
    let final_forecast = FinalForecast::new(
        32.0,
        25.0,
        "Cloudy".to_string(),
        "high".to_string(),
    );
    let day = DayEnsemble::new("2025-11-26".to_string(), per_source, final_forecast);
    forecast.add_day(day);

    // Serialize to JSON
    let json = serde_json::to_string(&forecast).unwrap();
    assert!(json.contains("Jakarta"));
    assert!(json.contains("DKI Jakarta"));

    // Deserialize back
    let deserialized: EnsembleForecast = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.city, "Jakarta");
    assert_eq!(deserialized.days.len(), 1);
}

#[test]
fn test_provider_forecast_validation() {
    // Valid forecast
    let valid = ProviderForecast::new(
        "2025-11-26".to_string(),
        32.0,
        25.0,
        "Cloudy".to_string(),
    );
    assert!(valid.is_valid());

    // Invalid: temp_max < temp_min
    let invalid = ProviderForecast::new(
        "2025-11-26".to_string(),
        25.0,
        32.0,
        "Cloudy".to_string(),
    );
    assert!(!invalid.is_valid());

    // Invalid: temperature out of range
    let out_of_range = ProviderForecast::new(
        "2025-11-26".to_string(),
        100.0,
        25.0,
        "Cloudy".to_string(),
    );
    assert!(!out_of_range.is_valid());

    // Invalid: empty condition
    let no_condition = ProviderForecast::new(
        "2025-11-26".to_string(),
        32.0,
        25.0,
        "".to_string(),
    );
    assert!(!no_condition.is_valid());
}

#[test]
fn test_final_forecast_validation() {
    // Valid
    let valid = FinalForecast::new(
        32.0,
        25.0,
        "Cloudy".to_string(),
        "high".to_string(),
    );
    assert!(valid.is_valid());

    // Invalid confidence level
    let invalid_confidence = FinalForecast::new(
        32.0,
        25.0,
        "Cloudy".to_string(),
        "invalid".to_string(),
    );
    assert!(!invalid_confidence.is_valid());
}

#[test]
fn test_ensemble_forecast_validation() {
    let mut forecast = EnsembleForecast::new(
        "Jakarta".to_string(),
        "DKI Jakarta".to_string(),
        "Indonesia".to_string(),
        -6.2,
        106.8,
    );

    // Invalid: not 7 days
    assert!(!forecast.is_valid());

    // Add 7 valid days
    for i in 0..7 {
        let per_source = PerSourceData::new().with_open_meteo(ProviderForecast::new(
            format!("2025-11-{:02}", 26 + i),
            32.0,
            25.0,
            "Cloudy".to_string(),
        ));
        let final_forecast = FinalForecast::new(
            32.0,
            25.0,
            "Cloudy".to_string(),
            "high".to_string(),
        );
        let day = DayEnsemble::new(
            format!("2025-11-{:02}", 26 + i),
            per_source,
            final_forecast,
        );
        forecast.add_day(day);
    }

    assert!(forecast.is_valid());
}

#[test]
fn test_per_source_get_max_temperatures() {
    let per_source = PerSourceData::new()
        .with_open_meteo(ProviderForecast::new(
            "2025-11-26".to_string(),
            32.0,
            25.0,
            "Cloudy".to_string(),
        ))
        .with_open_weather(ProviderForecast::new(
            "2025-11-26".to_string(),
            33.0,
            25.5,
            "Cloudy".to_string(),
        ))
        .with_weather_api(ProviderForecast::new(
            "2025-11-26".to_string(),
            31.5,
            24.9,
            "Rain".to_string(),
        ));

    let max_temps = per_source.get_max_temperatures();
    assert_eq!(max_temps.len(), 3);
    assert!(max_temps.contains(&32.0));
    assert!(max_temps.contains(&33.0));
    assert!(max_temps.contains(&31.5));
}

#[test]
fn test_per_source_get_min_temperatures() {
    let per_source = PerSourceData::new()
        .with_open_meteo(ProviderForecast::new(
            "2025-11-26".to_string(),
            32.0,
            25.0,
            "Cloudy".to_string(),
        ))
        .with_open_weather(ProviderForecast::new(
            "2025-11-26".to_string(),
            33.0,
            25.5,
            "Cloudy".to_string(),
        ));

    let min_temps = per_source.get_min_temperatures();
    assert_eq!(min_temps.len(), 2);
    assert!(min_temps.contains(&25.0));
    assert!(min_temps.contains(&25.5));
}

#[test]
fn test_per_source_get_conditions() {
    let per_source = PerSourceData::new()
        .with_open_meteo(ProviderForecast::new(
            "2025-11-26".to_string(),
            32.0,
            25.0,
            "Cloudy".to_string(),
        ))
        .with_weather_api(ProviderForecast::new(
            "2025-11-26".to_string(),
            31.5,
            24.9,
            "Rain".to_string(),
        ));

    let conditions = per_source.get_conditions();
    assert_eq!(conditions.len(), 2);
    assert!(conditions.contains(&"Cloudy".to_string()));
    assert!(conditions.contains(&"Rain".to_string()));
}

#[test]
fn test_per_source_extract_temperatures() {
    let per_source = PerSourceData::new()
        .with_open_meteo(ProviderForecast::new(
            "2025-11-26".to_string(),
            32.0,
            25.0,
            "Cloudy".to_string(),
        ))
        .with_open_weather(ProviderForecast::new(
            "2025-11-26".to_string(),
            33.0,
            25.5,
            "Cloudy".to_string(),
        ));

    let (maxes, mins) = per_source.extract_temperatures();
    assert_eq!(maxes.len(), 2);
    assert_eq!(mins.len(), 2);
    assert!(maxes.contains(&32.0));
    assert!(maxes.contains(&33.0));
    assert!(mins.contains(&25.0));
    assert!(mins.contains(&25.5));
}

#[test]
fn test_ensemble_with_partial_provider_data() {
    let per_source = PerSourceData::new().with_open_meteo(ProviderForecast::new(
        "2025-11-26".to_string(),
        32.0,
        25.0,
        "Cloudy".to_string(),
    ));

    assert_eq!(per_source.provider_count(), 1);
    assert_eq!(per_source.get_max_temperatures().len(), 1);
    assert_eq!(per_source.get_min_temperatures().len(), 1);
    assert_eq!(per_source.get_conditions().len(), 1);
}
