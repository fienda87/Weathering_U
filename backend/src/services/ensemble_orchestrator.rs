use crate::models::{City, EnsembleForecast, ForecastPeriodRequest, DayEnsemble, FinalForecast};
use crate::services::cache::ForecastCache;
use crate::services::ensemble::{fetch_ensemble_week, calculate_final_forecast};
use crate::services::confidence_calculator::calculate_confidence;
use crate::utils::date_utils::{get_forecast_dates, ForecastPeriod};
use std::sync::Arc;

/// Main orchestrator for ensemble forecasts
pub struct EnsembleOrchestrator {
    cache: Arc<ForecastCache<EnsembleForecast>>,
    openweather_key: String,
    weatherapi_key: String,
}

impl EnsembleOrchestrator {
    pub fn new(
        cache: Arc<ForecastCache<EnsembleForecast>>,
        openweather_key: String,
        weatherapi_key: String,
    ) -> Self {
        Self {
            cache,
            openweather_key,
            weatherapi_key,
        }
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

        // Fetch ensemble data from all providers for 7 days
        let per_source_days = fetch_ensemble_week(
            city,
            &self.openweather_key,
            &self.weatherapi_key,
        ).await?;

        // Get dates for current week
        let dates = get_forecast_dates(ForecastPeriod::CurrentWeek)
            .map_err(|e| format!("Date calculation error: {}", e))?;

        // Build EnsembleForecast
        let mut forecast = EnsembleForecast::new(
            city.name.to_string(),
            city.province.to_string(),
            "Indonesia".to_string(),
            city.latitude,
            city.longitude,
        );

        for (idx, per_source) in per_source_days.iter().enumerate() {
            let date = dates.get(idx)
                .ok_or_else(|| format!("Missing date for day {}", idx))?
                .clone();

            // Calculate final forecast values
            let (temp_max, temp_min, condition) = calculate_final_forecast(per_source, date.clone())?;

            // Calculate confidence level
            let confidence = calculate_confidence(per_source, (temp_max, temp_min));

            let final_forecast = FinalForecast::new(temp_max, temp_min, condition, confidence);
            let day_ensemble = DayEnsemble::new(date, per_source.clone(), final_forecast);

            forecast.add_day(day_ensemble);
        }

        // Cache the result
        self.cache.insert(cache_key, forecast.clone()).await;

        log::info!("[Orchestrator] Successfully built ensemble forecast for {} with {} days", 
            city.name, forecast.days.len());

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

        // Get the target date (D+7)
        let dates = get_forecast_dates(ForecastPeriod::NextWeek { base_day })
            .map_err(|e| format!("Date calculation error: {}", e))?;

        if dates.is_empty() {
            return Err("No target date calculated for next week".to_string());
        }

        let target_date = &dates[0];
        log::info!("[Orchestrator] Target D+7 date: {}", target_date);

        // Fetch ensemble data from all providers for the next 7 days (to get D+7)
        // We need to fetch all 7 days because providers give us sequential forecasts
        let per_source_days = fetch_ensemble_week(
            city,
            &self.openweather_key,
            &self.weatherapi_key,
        ).await?;

        // Build EnsembleForecast with just this one day
        // Note: For D+7, we're still showing a single-day forecast
        // In a more advanced implementation, you might fetch 7-14 day forecasts
        let mut forecast = EnsembleForecast::new(
            city.name.to_string(),
            city.province.to_string(),
            "Indonesia".to_string(),
            city.latitude,
            city.longitude,
        );

        // For now, use the 7th day (index 6) as a proxy for D+7
        // This is a limitation of free weather APIs that typically provide 7-day forecasts
        let day_idx = 6; // Last day of the 7-day forecast
        if let Some(per_source) = per_source_days.get(day_idx) {
            // Calculate final forecast values
            let (temp_max, temp_min, condition) = calculate_final_forecast(per_source, target_date.clone())?;

            // Calculate confidence level
            let confidence = calculate_confidence(per_source, (temp_max, temp_min));

            let final_forecast = FinalForecast::new(temp_max, temp_min, condition, confidence);
            let day_ensemble = DayEnsemble::new(target_date.clone(), per_source.clone(), final_forecast);

            forecast.add_day(day_ensemble);
        } else {
            return Err("Failed to fetch next week forecast".to_string());
        }

        // Cache the result
        self.cache.insert(cache_key, forecast.clone()).await;

        log::info!("[Orchestrator] Successfully built next week ensemble forecast for {}", city.name);

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
