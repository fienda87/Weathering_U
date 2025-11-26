#[tokio::test]
async fn test_cache_integration_with_forecast() {
    use backend::services::cache::ForecastCache;

    let cache: ForecastCache<String> = ForecastCache::new(3600, 10);
    
    let cache_key = "forecast:jakarta:current_week".to_string();
    
    // Simulate fetching forecast
    let data = cache.get_or_fetch(
        cache_key.clone(),
        || async {
            // Simulate API call
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            Ok("{\"city\":\"Jakarta\",\"days\":7}".to_string())
        },
    ).await.unwrap();
    
    assert!(data.contains("Jakarta"));
    
    // Next call should be instant (from cache)
    let start = std::time::Instant::now();
    let cached_data = cache.get_or_fetch(
        cache_key,
        || async {
            // This shouldn't run
            Err::<String, _>("Should not fetch".to_string())
        },
    ).await.unwrap();
    let elapsed = start.elapsed();
    
    assert_eq!(data, cached_data);
    assert!(elapsed.as_millis() < 50); // Cache should be fast
}
