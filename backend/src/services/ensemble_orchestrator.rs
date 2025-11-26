use crate::models::{City, EnsembleForecast, ForecastPeriodRequest};
use crate::services::cache::ForecastCache;
use std::sync::Arc;

/// Main orchestrator for ensemble forecasts
pub struct EnsembleOrchestrator {
    cache: Arc<ForecastCache<EnsembleForecast>>,
}

impl EnsembleOrchestrator {
    pub fn new(cache: Arc<ForecastCache<EnsembleForecast>>) -> Self {
        Self { cache }
    }

    /// Get forecast based on period (current week or next week)
    pub async fn get_forecast(
        &self,
        city: &City,
        period: ForecastPeriodRequest,
    ) -> Result<EnsembleForecast, String> {
        match period {
            ForecastPeriodRequest::CurrentWeek => {
                self.get_current_week(city).await
            }
            ForecastPeriodRequest::NextWeek { base_day } => {
                self.get_next_week(city, base_day).await
            }
        }
    }

    /// Get current week forecast (7 days)
    async fn get_current_week(&self, city: &City) -> Result<EnsembleForecast, String> {
        let cache_key = format!("forecast:{}:current_week", city.name.to_lowercase());

        // Try cache
        if let Some(cached) = self.cache.get(&cache_key).await {
            log::info!("[Orchestrator] Cache HIT: {}", city.name);
            return Ok(cached);
        }

        log::info!("[Orchestrator] Fetching current week for {}", city.name);

        // TODO: Implement 7-day parallel fetching
        let forecast = EnsembleForecast::new(
            city.name.to_string(),
            city.province.to_string(),
            "Indonesia".to_string(),
            city.latitude,
            city.longitude,
        );

        self.cache.insert(cache_key, forecast.clone()).await;

        Ok(forecast)
    }

    /// Get next week forecast (single day)
    async fn get_next_week(&self, city: &City, base_day: u32) -> Result<EnsembleForecast, String> {
        if base_day > 6 {
            return Err(format!("Invalid day: {}", base_day));
        }

        let cache_key = format!(
            "forecast:{}:next_week:{}",
            city.name.to_lowercase(),
            base_day
        );

        // Try cache
        if let Some(cached) = self.cache.get(&cache_key).await {
            log::info!("[Orchestrator] Cache HIT: next week");
            return Ok(cached);
        }

        log::info!("[Orchestrator] Fetching next week day {} for {}", base_day, city.name);

        // TODO: Implement D+7 fetching
        let forecast = EnsembleForecast::new(
            city.name.to_string(),
            city.province.to_string(),
            "Indonesia".to_string(),
            city.latitude,
            city.longitude,
        );

        self.cache.insert(cache_key, forecast.clone()).await;

        Ok(forecast)
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> String {
        let stats = self.cache.stats().await;
        format!("Cache: {} entries, {} valid, {} expired, TTL: {}s",
            stats.total_entries,
            stats.valid_entries,
            stats.expired_entries,
            stats.ttl_seconds
        )
    }
}
