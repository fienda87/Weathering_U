#[tokio::test]
async fn test_cache_insert_and_get() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 100);
    
    cache.insert("key1".to_string(), "value1".to_string()).await;
    
    let result = cache.get("key1").await;
    assert_eq!(result, Some("value1".to_string()));
}

#[tokio::test]
async fn test_cache_miss() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 100);
    
    let result = cache.get("nonexistent").await;
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_cache_expiration() {
    use backend::services::cache::ForecastCache;
    use std::time::Duration;

    let cache = ForecastCache::new(1, 100); // 1 second TTL
    
    cache.insert("key1".to_string(), "value1".to_string()).await;
    
    // Should exist immediately
    assert!(cache.get("key1").await.is_some());
    
    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(1100)).await;
    
    // Should be expired
    assert!(cache.get("key1").await.is_none());
}

#[tokio::test]
async fn test_get_or_fetch_from_cache() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 100);
    
    // First call fetches
    let result = cache.get_or_fetch(
        "key1".to_string(),
        || async { Ok::<_, String>("fetched".to_string()) },
    ).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "fetched");
    
    // Second call should return cached
    let result2 = cache.get_or_fetch(
        "key1".to_string(),
        || async { Ok::<_, String>("new".to_string()) },
    ).await;
    
    assert_eq!(result2.unwrap(), "fetched"); // Cached value, not "new"
}

#[tokio::test]
async fn test_get_or_fetch_error() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 100);
    
    let result = cache.get_or_fetch(
        "key1".to_string(),
        || async { Err::<String, _>("fetch failed".to_string()) },
    ).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_cache_clear() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 100);
    
    cache.insert("key1".to_string(), "value1".to_string()).await;
    cache.insert("key2".to_string(), "value2".to_string()).await;
    
    assert_eq!(cache.size().await, 2);
    
    cache.clear().await;
    
    assert_eq!(cache.size().await, 0);
}

#[tokio::test]
async fn test_cache_capacity() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 3); // Max 3 entries
    
    cache.insert("key1".to_string(), "value1".to_string()).await;
    cache.insert("key2".to_string(), "value2".to_string()).await;
    cache.insert("key3".to_string(), "value3".to_string()).await;
    
    assert_eq!(cache.size().await, 3);
    
    // Add 4th entry (should remove oldest)
    cache.insert("key4".to_string(), "value4".to_string()).await;
    
    assert_eq!(cache.size().await, 3);
}

#[tokio::test]
async fn test_cache_stats() {
    use backend::services::cache::ForecastCache;

    let cache = ForecastCache::new(3600, 100);
    
    cache.insert("key1".to_string(), "value1".to_string()).await;
    cache.insert("key2".to_string(), "value2".to_string()).await;
    
    let stats = cache.stats().await;
    
    assert_eq!(stats.total_entries, 2);
    assert_eq!(stats.capacity, 100);
    assert_eq!(stats.ttl_seconds, 3600);
}

#[tokio::test]
async fn test_cache_cleanup() {
    use backend::services::cache::ForecastCache;
    use std::time::Duration;

    let cache = ForecastCache::new(1, 100); // 1 second TTL
    
    cache.insert("key1".to_string(), "value1".to_string()).await;
    
    assert_eq!(cache.size().await, 1);
    
    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(1100)).await;
    
    // Before cleanup
    assert_eq!(cache.size().await, 1); // Still in cache, just expired
    
    // After cleanup
    cache.cleanup().await;
    assert_eq!(cache.size().await, 0); // Expired entries removed
}

#[tokio::test]
async fn test_cached_entry_is_valid() {
    use backend::services::cache::CachedEntry;
    use std::time::{SystemTime, Duration};

    let entry = CachedEntry {
        data: "test".to_string(),
        timestamp: SystemTime::now(),
    };

    // Should be valid with large TTL
    assert!(entry.is_valid(Duration::from_secs(3600)));
}

#[tokio::test]
async fn test_cached_entry_expired() {
    use backend::services::cache::CachedEntry;
    use std::time::{SystemTime, Duration};

    let entry = CachedEntry {
        data: "test".to_string(),
        timestamp: SystemTime::now() - Duration::from_secs(10),
    };

    // Should be expired with short TTL
    assert!(!entry.is_valid(Duration::from_secs(5)));
}

#[tokio::test]
async fn test_cached_entry_remaining_ttl() {
    use backend::services::cache::CachedEntry;
    use std::time::{SystemTime, Duration};

    let entry = CachedEntry {
        data: "test".to_string(),
        timestamp: SystemTime::now(),
    };

    let remaining = entry.remaining_ttl(Duration::from_secs(3600));
    assert!(remaining.is_some());
    assert!(remaining.unwrap().as_secs() > 3500); // Should be close to 3600
}

#[tokio::test]
async fn test_cached_entry_no_remaining_ttl() {
    use backend::services::cache::CachedEntry;
    use std::time::{SystemTime, Duration};

    let entry = CachedEntry {
        data: "test".to_string(),
        timestamp: SystemTime::now() - Duration::from_secs(10),
    };

    let remaining = entry.remaining_ttl(Duration::from_secs(5));
    assert!(remaining.is_none()); // Expired, no remaining TTL
}

#[tokio::test]
async fn test_concurrent_access() {
    use backend::services::cache::ForecastCache;
    use std::sync::Arc;

    let cache = Arc::new(ForecastCache::new(3600, 100));
    let mut handles = vec![];

    // Spawn 10 concurrent tasks
    for i in 0..10 {
        let cache_clone = Arc::clone(&cache);
        let handle = tokio::spawn(async move {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            cache_clone.insert(key.clone(), value.clone()).await;
            cache_clone.get(&key).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await.unwrap();
        assert_eq!(result, Some(format!("value{}", i)));
    }

    // Verify all entries are in cache
    assert_eq!(cache.size().await, 10);
}

#[tokio::test]
async fn test_stats_with_expired_entries() {
    use backend::services::cache::ForecastCache;
    use std::time::Duration;

    let cache = ForecastCache::new(1, 100); // 1 second TTL

    cache.insert("key1".to_string(), "value1".to_string()).await;
    cache.insert("key2".to_string(), "value2".to_string()).await;

    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(1100)).await;

    // Add a new entry (still valid)
    cache.insert("key3".to_string(), "value3".to_string()).await;

    let stats = cache.stats().await;

    assert_eq!(stats.total_entries, 3);
    assert_eq!(stats.expired_entries, 2);
    assert_eq!(stats.valid_entries, 1);
}

#[tokio::test]
async fn test_get_or_fetch_after_expiration() {
    use backend::services::cache::ForecastCache;
    use std::time::Duration;

    let cache = ForecastCache::new(1, 100); // 1 second TTL

    // First fetch
    let result = cache.get_or_fetch(
        "key1".to_string(),
        || async { Ok::<_, String>("first".to_string()) },
    ).await.unwrap();

    assert_eq!(result, "first");

    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(1100)).await;

    // Should fetch again after expiration
    let result2 = cache.get_or_fetch(
        "key1".to_string(),
        || async { Ok::<_, String>("second".to_string()) },
    ).await.unwrap();

    assert_eq!(result2, "second"); // New value, not "first"
}

#[tokio::test]
async fn test_multiple_cache_instances() {
    use backend::services::cache::ForecastCache;

    let cache1 = ForecastCache::new(3600, 100);
    let cache2 = ForecastCache::new(3600, 100);

    cache1.insert("key1".to_string(), "value1".to_string()).await;
    cache2.insert("key1".to_string(), "value2".to_string()).await;

    // Each cache should have its own data
    assert_eq!(cache1.get("key1").await, Some("value1".to_string()));
    assert_eq!(cache2.get("key1").await, Some("value2".to_string()));
}

#[tokio::test]
async fn test_cache_with_complex_data() {
    use backend::services::cache::ForecastCache;
    use serde::{Serialize, Deserialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct ForecastData {
        city: String,
        temperature: f64,
        days: Vec<String>,
    }

    let cache = ForecastCache::new(3600, 100);

    let forecast = ForecastData {
        city: "Jakarta".to_string(),
        temperature: 28.5,
        days: vec!["Mon".to_string(), "Tue".to_string()],
    };

    cache.insert("forecast:jakarta".to_string(), forecast.clone()).await;

    let result = cache.get("forecast:jakarta").await;
    assert_eq!(result, Some(forecast));
}
