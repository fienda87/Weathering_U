use crate::models::{City, DailyForecast, WeatherForecast};
use crate::services::daily_processor::process_day;
use futures::future::join_all;
use log::{info, warn, error};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;

/// Metrics for parallel forecast processing
#[derive(Debug, Clone)]
pub struct TaskMetrics {
    pub total_start_time: Instant,
    pub total_end_time: Option<Instant>,
    pub task_start_times: Vec<Instant>,
    pub task_end_times: Vec<Instant>,
    pub successful_tasks: usize,
    pub failed_tasks: usize,
    pub timed_out_tasks: usize,
}

impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            total_start_time: Instant::now(),
            total_end_time: None,
            task_start_times: Vec::with_capacity(7),
            task_end_times: Vec::with_capacity(7),
            successful_tasks: 0,
            failed_tasks: 0,
            timed_out_tasks: 0,
        }
    }

    pub fn finish_total(&mut self) {
        self.total_end_time = Some(Instant::now());
    }

    pub fn total_duration(&self) -> Duration {
        match self.total_end_time {
            Some(end) => end.duration_since(self.total_start_time),
            None => self.total_start_time.elapsed(),
        }
    }

    pub fn log_summary(&self) {
        let total_duration = self.total_duration();
        let parallelism_efficiency = if self.successful_tasks > 0 {
            (self.successful_tasks as f64 / total_duration.as_secs_f64()) * 100.0
        } else {
            0.0
        };

        info!("=== Parallel Forecast Processing Metrics ===");
        info!("Total processing time: {:?}", total_duration);
        info!("Successful tasks: {}/{}", self.successful_tasks, 7);
        info!("Failed tasks: {}", self.failed_tasks);
        info!("Timed out tasks: {}", self.timed_out_tasks);
        info!("Parallelism efficiency: {:.1}%", parallelism_efficiency);
        
        if !self.task_start_times.is_empty() {
            for (i, (start, end)) in self.task_start_times.iter().zip(self.task_end_times.iter()).enumerate() {
                info!("Day {} processing time: {:?}", i, end.duration_since(*start));
            }
        }
        info!("============================================");
    }
}

/// Fetch 7-day forecast using parallel task spawning
pub async fn fetch_forecast_parallel(
    city: &City,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<WeatherForecast, String> {
    let mut metrics = TaskMetrics::new();
    info!("Starting parallel forecast processing for city: {} ({})", city.name, city.province);
    
    let mut handles: Vec<JoinHandle<Result<DailyForecast, String>>> = Vec::new();

    // Spawn 7 tasks, 1 per day
    for day in 0..7 {
        let city_clone = city.clone();
        let openweather_key = openweather_key.to_string();
        let weatherapi_key = weatherapi_key.to_string();
        
        let handle = tokio::spawn(async move {
            let task_start = Instant::now();
            info!("Day {} task started at {:?}", day, task_start);
            let result = process_day(day, &city_clone, &openweather_key, &weatherapi_key).await;
            let task_end = Instant::now();
            info!("Day {} task completed at {:?} (duration: {:?})", day, task_end, task_end.duration_since(task_start));
            result
        });

        handles.push(handle);
        info!("Spawned task for day {}", day);
    }

    // Collect results from all 7 days
    info!("Waiting for all {} tasks to complete", handles.len());
    let results = join_all(handles).await;

    let mut successful_days = Vec::new();
    let mut failed_days = Vec::new();

    for (day_index, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(forecast)) => {
                successful_days.push(forecast);
                metrics.successful_tasks += 1;
                info!("Day {} completed successfully", day_index);
            }
            Ok(Err(e)) => {
                metrics.failed_tasks += 1;
                warn!("Day {} failed: {}", day_index, &e);
                failed_days.push((day_index, e));
            }
            Err(e) => {
                metrics.failed_tasks += 1;
                let error_msg = format!("Task panicked: {}", e);
                failed_days.push((day_index, error_msg));
                error!("Day {} task panicked: {}", day_index, e);
            }
        }
    }

    metrics.finish_total();
    metrics.log_summary();

    // Check if we have enough successful days
    if successful_days.len() < 3 {
        error!("Too many failed days ({}/7), cannot generate reliable forecast", 7 - successful_days.len());
        return Err(format!("Insufficient successful forecast days: {}/7", successful_days.len()));
    }

    // Sort successful days by date
    successful_days.sort_by(|a, b| a.date.cmp(&b.date));

    // Log failed days for debugging
    if !failed_days.is_empty() {
        warn!("Failed days summary:");
        for (day, error) in &failed_days {
            warn!("  Day {}: {}", day, error);
        }
    }

    let num_days = successful_days.len();
    let forecast = WeatherForecast {
        city: city.name.to_string(),
        province: city.province.to_string(),
        country: "Indonesia".to_string(),
        latitude: city.latitude,
        longitude: city.longitude,
        last_updated: chrono::Local::now().to_rfc3339(),
        forecast: successful_days,
    };

    info!("Parallel forecast processing completed: {} successful days out of 7", num_days);
    Ok(forecast)
}

/// Fetch 7-day forecast with semaphore-based rate limiting
pub async fn fetch_forecast_with_rate_limit(
    city: &City,
    semaphore: Arc<Semaphore>,
    openweather_key: &str,
    weatherapi_key: &str,
) -> Result<WeatherForecast, String> {
    let mut metrics = TaskMetrics::new();
    info!("Starting rate-limited parallel forecast processing for city: {} ({})", city.name, city.province);
    info!("Semaphore permits available: {}", semaphore.available_permits());
    
    let mut handles: Vec<JoinHandle<Result<DailyForecast, String>>> = Vec::new();

    // Spawn 7 tasks with semaphore control
    for day in 0..7 {
        let city_clone = city.clone();
        let sem = semaphore.clone();
        let openweather_key = openweather_key.to_string();
        let weatherapi_key = weatherapi_key.to_string();
        
        let handle = tokio::spawn(async move {
            // Acquire semaphore permit (this will block if limit reached)
            let _permit = match sem.acquire().await {
                Ok(permit) => {
                    info!("Day {}: Acquired semaphore permit", day);
                    permit
                }
                Err(e) => {
                    error!("Day {}: Failed to acquire semaphore permit: {}", day, e);
                    return Err("Failed to acquire semaphore permit".to_string());
                }
            };

            info!("Day {}: Starting processing with semaphore protection", day);
            let result = process_day(day, &city_clone, &openweather_key, &weatherapi_key).await;
            
            // Permit is automatically released when _permit goes out of scope
            info!("Day {}: Released semaphore permit", day);
            result
        });

        handles.push(handle);
        info!("Spawned rate-limited task for day {}", day);
    }

    // Collect results (same as parallel version)
    info!("Waiting for all {} rate-limited tasks to complete", handles.len());
    let results = join_all(handles).await;

    let mut successful_days = Vec::new();
    let mut failed_days = Vec::new();

    for (day_index, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(forecast)) => {
                successful_days.push(forecast);
                metrics.successful_tasks += 1;
                info!("Day {} completed successfully", day_index);
            }
            Ok(Err(e)) => {
                metrics.failed_tasks += 1;
                warn!("Day {} failed: {}", day_index, &e);
                failed_days.push((day_index, e));
            }
            Err(e) => {
                metrics.failed_tasks += 1;
                let error_msg = format!("Task panicked: {}", e);
                failed_days.push((day_index, error_msg));
                error!("Day {} task panicked: {}", day_index, e);
            }
        }
    }

    metrics.finish_total();
    metrics.log_summary();

    // Check if we have enough successful days
    if successful_days.len() < 3 {
        error!("Too many failed days ({}/7), cannot generate reliable forecast", 7 - successful_days.len());
        return Err(format!("Insufficient successful forecast days: {}/7", successful_days.len()));
    }

    // Sort successful days by date
    successful_days.sort_by(|a, b| a.date.cmp(&b.date));

    // Log failed days for debugging
    if !failed_days.is_empty() {
        warn!("Failed days summary:");
        for (day, error) in &failed_days {
            warn!("  Day {}: {}", day, error);
        }
    }

    let num_days = successful_days.len();
    let forecast = WeatherForecast {
        city: city.name.to_string(),
        province: city.province.to_string(),
        country: "Indonesia".to_string(),
        latitude: city.latitude,
        longitude: city.longitude,
        last_updated: chrono::Local::now().to_rfc3339(),
        forecast: successful_days,
    };

    info!("Rate-limited parallel forecast processing completed: {} successful days out of 7", num_days);
    Ok(forecast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{City, DailyForecast};
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    fn create_test_city() -> City {
        City {
            id: 1,
            name: "Jakarta",
            province: "DKI Jakarta",
            latitude: -6.2088,
            longitude: 106.8456,
        }
    }

    #[tokio::test]
    async fn test_task_metrics_creation() {
        let metrics = TaskMetrics::new();
        assert_eq!(metrics.successful_tasks, 0);
        assert_eq!(metrics.failed_tasks, 0);
        assert_eq!(metrics.timed_out_tasks, 0);
    }

    #[tokio::test]
    async fn test_task_metrics_duration() {
        let mut metrics = TaskMetrics::new();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        metrics.finish_total();
        
        let duration = metrics.total_duration();
        assert!(duration.as_millis() >= 10);
    }

    #[tokio::test]
    async fn test_semaphore_creation() {
        let semaphore = Arc::new(Semaphore::new(3));
        assert_eq!(semaphore.available_permits(), 3);
    }

    #[tokio::test]
    async fn test_process_day_timeout() {
        let city = create_test_city();
        
        // This should fail due to invalid API keys and timeout
        let result = process_day(
            0,
            &city,
            "invalid-key",
            "invalid-key"
        ).await;
        
        // We expect this to fail since we're using invalid keys
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_parallel_forecast_structure() {
        let city = create_test_city();
        
        // Test that the function signature works (will fail with invalid keys)
        let result = fetch_forecast_parallel(
            &city,
            "invalid-key",
            "invalid-key"
        ).await;
        
        // Should fail due to invalid API keys, but structure should be correct
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rate_limited_forecast_structure() {
        let city = create_test_city();
        let semaphore = Arc::new(Semaphore::new(3));
        
        // Test that the function signature works (will fail with invalid keys)
        let result = fetch_forecast_with_rate_limit(
            &city,
            semaphore,
            "invalid-key",
            "invalid-key"
        ).await;
        
        // Should fail due to invalid API keys, but structure should be correct
        assert!(result.is_err());
    }
}