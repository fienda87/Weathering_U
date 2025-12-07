use crate::models::{City, EnsembleForecast, ForecastPeriodRequest, DayEnsemble, FinalForecast};
use crate::services::cache::ForecastCache;
use crate::services::ensemble_fetcher::{fetch_ensemble_week, calculate_final_forecast};
use crate::services::confidence_calculator::calculate_confidence;
use crate::utils::date_utils::{get_forecast_dates, ForecastPeriod};
use std::sync::Arc;

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

    async fn get_current_week(&self, city: &City) -> Result<EnsembleForecast, String> {
        let cache_key = format!("forecast:{}:current_week", city.name.to_lowercase());

        if let Some(cached) = self.cache.get(&cache_key).await {
            log::info!("[Orchestrator] Cache HIT: {}", city.name);
            return Ok(cached);
        }

        log::info!("[Orchestrator] Fetching current week for {}", city.name);

        let per_source_days = fetch_ensemble_week(
            city,
            &self.openweather_key,
            &self.weatherapi_key,
        ).await?;

        let dates = get_forecast_dates(ForecastPeriod::CurrentWeek)
            .map_err(|e| format!("Date calculation error: {}", e))?;

        let mut forecast = EnsembleForecast::new(
            city.name.to_string(),
            city.province.to_string(),
            "Indonesia".to_string(),
            city.latitude,
            city.longitude,
        );

        for (idx, per_source) in per_source_days.iter().enumerate() {
            let date: String = dates.get(idx)
                .ok_or_else(|| format!("Missing date for day {}", idx))?
                .clone();

            let (temp_max, temp_min, condition) = calculate_final_forecast(per_source, date.clone())?;

            let confidence = calculate_confidence(per_source, (temp_max, temp_min));

            let final_forecast = FinalForecast::new(temp_max, temp_min, condition, confidence);
            let day_ensemble = DayEnsemble::new(date, per_source.clone(), final_forecast);

            forecast.add_day(day_ensemble);
        }

        self.cache.insert(cache_key, forecast.clone()).await;

        log::info!("[Orchestrator] Successfully built ensemble forecast for {} with {} days", 
            city.name, forecast.days.len());

        Ok(forecast)
    }

    /// Ambil forecast minggu depan (satu hari)
    async fn get_next_week(&self, city: &City, base_day: u32) -> Result<EnsembleForecast, String> {
        if base_day > 6 {
            return Err(format!("Invalid day: {}", base_day));
        }

        let cache_key = format!(
            "forecast:{}:next_week:{}",
            city.name.to_lowercase(),
            base_day
        );

        // Coba cek cache dulu
        if let Some(cached) = self.cache.get(&cache_key).await {
            log::info!("[Orchestrator] Cache HIT: next week");
            return Ok(cached);
        }

        log::info!("[Orchestrator] Fetching next week day {} for {}", base_day, city.name);

        // Ambil tanggal target (D+7 minggu depan)
        let dates = get_forecast_dates(ForecastPeriod::NextWeek { base_day })
            .map_err(|e| format!("Date calculation error: {}", e))?;

        if dates.is_empty() {
            return Err("No target date calculated for next week".to_string());
        }

        let target_date = &dates[0];
        log::info!("[Orchestrator] Target D+7 date: {}", target_date);

        // Ambil data ensemble untuk 7 hari ke depan (biar dapet D+7)
        // Harus fetch 7 hari karena provider kasih forecast berurutan
        let per_source_days = fetch_ensemble_week(
            city,
            &self.openweather_key,
            &self.weatherapi_key,
        ).await?;

        // Buat EnsembleForecast cuma untuk satu hari ini
        // Catatan: Untuk D+7, kita cuma tampilkan forecast 1 hari
        // Bisa lebih advanced kalau fetch forecast 7-14 hari
        let mut forecast = EnsembleForecast::new(
            city.name.to_string(),
            city.province.to_string(),
            "Indonesia".to_string(),
            city.latitude,
            city.longitude,
        );

        // Untuk sementara, pakai hari ke-7 (index 6) sebagai proxy D+7
        // Ini limitasi API gratis yang biasanya cuma kasih 7 hari
        let day_idx = 6; // Hari terakhir dari forecast 7 hari
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
}
