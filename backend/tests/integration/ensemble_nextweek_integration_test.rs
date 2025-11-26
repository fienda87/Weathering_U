use backend::services::{
    cache::ForecastCache,
    ensemble_orchestrator::EnsembleOrchestrator,
};
use backend::models::{City, ForecastPeriodRequest};
use std::sync::Arc;

#[tokio::test]
async fn test_orchestrator_current_week() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache);

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let result = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::CurrentWeek,
    ).await;

    assert!(result.is_ok());
    let forecast = result.unwrap();
    assert_eq!(forecast.city, "Jakarta");
}

#[tokio::test]
async fn test_orchestrator_next_week() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache);

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let result = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::NextWeek { base_day: 0 },
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cache_hit_current_week() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache.clone());

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    // First call
    let _ = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::CurrentWeek,
    ).await;

    // Check cache has entry
    assert!(cache.size().await > 0);

    // Second call should hit cache
    let _ = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::CurrentWeek,
    ).await;

    // Cache still has entry
    assert!(cache.size().await > 0);
}

#[tokio::test]
async fn test_orchestrator_invalid_next_week_day() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache);

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let result = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::NextWeek { base_day: 7 },
    ).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Invalid day"));
}

#[tokio::test]
async fn test_orchestrator_cache_stats() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache.clone());

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    // Make a request to populate cache
    let _ = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::CurrentWeek,
    ).await;

    // Get stats
    let stats_string = orchestrator.cache_stats().await;
    
    // Verify stats string contains expected information
    assert!(stats_string.contains("Cache:"));
    assert!(stats_string.contains("entries"));
    assert!(stats_string.contains("TTL:"));
}

#[tokio::test]
async fn test_next_week_different_days() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache.clone());

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    // Request for Monday (day 0)
    let result_monday = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::NextWeek { base_day: 0 },
    ).await;

    assert!(result_monday.is_ok());

    // Request for Friday (day 4)
    let result_friday = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::NextWeek { base_day: 4 },
    ).await;

    assert!(result_friday.is_ok());

    // Cache should have 2 entries (different keys)
    assert_eq!(cache.size().await, 2);
}

#[tokio::test]
async fn test_orchestrator_multiple_cities() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache.clone());

    let jakarta = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let bandung = City {
        id: 2,
        name: "Bandung",
        province: "Jawa Barat",
        latitude: -6.9,
        longitude: 107.6,
    };

    // Request for Jakarta
    let result1 = orchestrator.get_forecast(
        &jakarta,
        ForecastPeriodRequest::CurrentWeek,
    ).await;

    assert!(result1.is_ok());

    // Request for Bandung
    let result2 = orchestrator.get_forecast(
        &bandung,
        ForecastPeriodRequest::CurrentWeek,
    ).await;

    assert!(result2.is_ok());

    // Cache should have 2 entries (different cities)
    assert_eq!(cache.size().await, 2);
}

#[tokio::test]
async fn test_cache_hit_next_week() {
    let cache = Arc::new(ForecastCache::new(3600, 100));
    let orchestrator = EnsembleOrchestrator::new(cache.clone());

    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    // First call - cache miss
    let result1 = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::NextWeek { base_day: 0 },
    ).await;

    assert!(result1.is_ok());
    assert_eq!(cache.size().await, 1);

    // Second call - cache hit
    let result2 = orchestrator.get_forecast(
        &city,
        ForecastPeriodRequest::NextWeek { base_day: 0 },
    ).await;

    assert!(result2.is_ok());
    // Cache size should still be 1
    assert_eq!(cache.size().await, 1);
}
