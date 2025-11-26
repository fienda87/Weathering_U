use std::sync::Arc;
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Cached forecast entry with metadata
#[derive(Clone)]
pub struct CachedEntry<T: Clone> {
    pub data: T,
    pub timestamp: SystemTime,
}

impl<T: Clone> CachedEntry<T> {
    /// Check if entry is still valid (not expired)
    pub fn is_valid(&self, ttl: Duration) -> bool {
        match self.timestamp.elapsed() {
            Ok(elapsed) => elapsed < ttl,
            Err(_) => false, // Clock went backwards, invalidate
        }
    }

    /// Get remaining TTL
    pub fn remaining_ttl(&self, ttl: Duration) -> Option<Duration> {
        match self.timestamp.elapsed() {
            Ok(elapsed) => {
                if elapsed < ttl {
                    Some(ttl - elapsed)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

/// Thread-safe forecast cache
pub struct ForecastCache<T: Clone> {
    cache: Arc<RwLock<HashMap<String, CachedEntry<T>>>>,
    ttl: Duration,
    max_entries: usize,
}

impl<T: Clone> ForecastCache<T> {
    /// Create new cache with TTL
    pub fn new(ttl_secs: u64, max_entries: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_secs),
            max_entries,
        }
    }

    /// Get cached value if exists and valid
    pub async fn get(&self, key: &str) -> Option<T> {
        let cache = self.cache.read().await;
        
        if let Some(entry) = cache.get(key) {
            if entry.is_valid(self.ttl) {
                log::debug!("[Cache] HIT: {}", key);
                return Some(entry.data.clone());
            } else {
                log::debug!("[Cache] EXPIRED: {}", key);
            }
        }
        
        None
    }

    /// Insert value into cache
    pub async fn insert(&self, key: String, data: T) {
        let mut cache = self.cache.write().await;
        
        // Simple size management: remove oldest entry if at capacity
        if cache.len() >= self.max_entries {
            log::warn!("[Cache] At capacity ({}), removing oldest entry", self.max_entries);
            // Remove first entry (simple FIFO)
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
            }
        }
        
        cache.insert(key.clone(), CachedEntry {
            data,
            timestamp: SystemTime::now(),
        });
        
        log::debug!("[Cache] WRITE: {}", key);
    }

    /// Get or fetch: return cached or call fetch function
    pub async fn get_or_fetch<F, Fut>(
        &self,
        key: String,
        fetch_fn: F,
    ) -> Result<T, String>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        // Try cache first
        if let Some(cached) = self.get(&key).await {
            return Ok(cached);
        }

        // Cache miss or expired, fetch fresh
        log::info!("[Cache] MISS: Fetching fresh data for {}", key);
        let data = fetch_fn().await?;
        
        // Store in cache
        self.insert(key, data.clone()).await;
        
        Ok(data)
    }

    /// Clear all entries
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        let count = cache.len();
        cache.clear();
        log::info!("[Cache] Cleared {} entries", count);
    }

    /// Get cache size
    pub async fn size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        
        let expired_count = cache.values()
            .filter(|entry| !entry.is_valid(self.ttl))
            .count();
        
        CacheStats {
            total_entries: cache.len(),
            expired_entries: expired_count,
            valid_entries: cache.len() - expired_count,
            capacity: self.max_entries,
            ttl_seconds: self.ttl.as_secs(),
        }
    }

    /// Remove expired entries
    pub async fn cleanup(&self) {
        let mut cache = self.cache.write().await;
        let initial_count = cache.len();
        
        cache.retain(|_, entry| entry.is_valid(self.ttl));
        
        let removed = initial_count - cache.len();
        if removed > 0 {
            log::info!("[Cache] Cleanup: Removed {} expired entries", removed);
        }
    }
}

/// Cache statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub valid_entries: usize,
    pub capacity: usize,
    pub ttl_seconds: u64,
}
