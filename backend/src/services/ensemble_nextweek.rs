use crate::models::{City, EnsembleForecast};
use crate::services::cache::ForecastCache;
use crate::utils::date_utils::get_next_week_date;
use std::sync::Arc;

/// Orchestrate next-week ensemble forecast
pub async fn fetch_next_week_ensemble(
    city: &City,
    base_day: u32,
    cache: Arc<ForecastCache<EnsembleForecast>>,
) -> Result<EnsembleForecast, String> {
    // Build cache key
    let cache_key = format!(
        "forecast:{}:next_week:day_{}",
        city.name.to_lowercase(),
        base_day
    );

    // Try cache first
    if let Some(cached) = cache.get(&cache_key).await {
        log::info!("[NextWeek] Cache HIT for {}", city.name);
        return Ok(cached);
    }

    log::info!("[NextWeek] Fetching D+7 for {} (day {})", city.name, base_day);

    // Calculate target date
    let target_dates = get_next_week_date(base_day)
        .map_err(|e| format!("Date calculation error: {}", e))?;

    if target_dates.is_empty() {
        return Err("No target date calculated".to_string());
    }

    let target_date = &target_dates[0];
    log::debug!("[NextWeek] Target date: {}", target_date);

    // Fetch ensemble for that day
    let forecast = EnsembleForecast::new(
        city.name.to_string(),
        city.province.to_string(),
        "Indonesia".to_string(),
        city.latitude,
        city.longitude,
    );

    // In production, this would fetch the actual day's data from providers
    // For now, create a placeholder day
    // TODO: Integrate with actual ensemble fetching logic

    // Cache the result
    cache.insert(cache_key, forecast.clone()).await;

    Ok(forecast)
}

/// Fetch with explicit day offset (0-13 for up to 2 weeks)
pub async fn fetch_ensemble_day_offset(
    city: &City,
    day_offset: u32,
    cache: Arc<ForecastCache<EnsembleForecast>>,
) -> Result<EnsembleForecast, String> {
    if day_offset > 13 {
        return Err(format!("Day offset {} exceeds 2 weeks", day_offset));
    }

    let cache_key = format!(
        "forecast:{}:day_offset_{}",
        city.name.to_lowercase(),
        day_offset
    );

    // Try cache first
    if let Some(cached) = cache.get(&cache_key).await {
        return Ok(cached);
    }

    // Fetch (placeholder for now)
    let forecast = EnsembleForecast::new(
        city.name.to_string(),
        city.province.to_string(),
        "Indonesia".to_string(),
        city.latitude,
        city.longitude,
    );

    cache.insert(cache_key, forecast.clone()).await;

    Ok(forecast)
}
