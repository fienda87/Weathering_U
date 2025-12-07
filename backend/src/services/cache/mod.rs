use std::sync::Arc;
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use tokio::sync::RwLock;


/// Entry cache forecast dengan metadata
#[derive(Clone)]
pub struct CachedEntry<T: Clone> {
    pub data: T,
    pub timestamp: SystemTime,
}

impl<T: Clone> CachedEntry<T> {
    /// Cek apakah entry masih valid (belum expired)
    pub fn is_valid(&self, ttl: Duration) -> bool {
        match self.timestamp.elapsed() {
            Ok(elapsed) => elapsed < ttl,
            Err(_) => false,
        }
    }
}

/// Cache forecast yang thread-safe
pub struct ForecastCache<T: Clone> {
    cache: Arc<RwLock<HashMap<String, CachedEntry<T>>>>,
    ttl: Duration,
    max_entries: usize,
}

impl<T: Clone> ForecastCache<T> {
    /// Buat cache baru dengan TTL
    pub fn new(ttl_secs: u64, max_entries: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_secs),
            max_entries,
        }
    }

    /// Ambil value dari cache kalau ada dan masih valid
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

    /// Masukin value ke cache
    pub async fn insert(&self, key: String, data: T) {
        let mut cache = self.cache.write().await;
        
        // Manajemen ukuran cache: hapus entry terlama kalau udah penuh
        if cache.len() >= self.max_entries {
            log::warn!("[Cache] At capacity ({}), removing oldest entry", self.max_entries);
            // Hapus entry pertama (FIFO simpel)
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

}
