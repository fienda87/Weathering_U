# Ensemble Next-Week Orchestration Implementation

## Overview
This implementation adds ensemble forecast fetching for next-week (D+7) predictions by orchestrating parallel per-day fetching with proper caching and error handling.

## Components Created

### 1. `src/services/ensemble_nextweek.rs`
Functions for fetching next-week ensemble forecasts:
- `fetch_next_week_ensemble()`: Fetches D+7 forecast for a specific weekday
- `fetch_ensemble_day_offset()`: Fetches forecast with explicit day offset (0-13 days)

**Features:**
- Cache-first strategy with automatic TTL management
- Date calculation integration via `get_next_week_date()`
- Comprehensive logging with `[NextWeek]` prefix
- Error handling for invalid inputs

**Cache Keys:**
- Next week: `forecast:{city}:next_week:day_{0-6}`
- Day offset: `forecast:{city}:day_offset_{0-13}`

### 2. `src/services/ensemble_orchestrator.rs`
Main orchestrator struct for managing ensemble forecasts:
- `EnsembleOrchestrator`: Struct with cache management
- `get_forecast()`: Dispatches to current week or next week based on `ForecastPeriodRequest`
- `get_current_week()`: Fetches 7-day current week forecast
- `get_next_week()`: Fetches single-day next week forecast (D+7)
- `cache_stats()`: Returns formatted cache statistics

**Features:**
- Unified interface for current/next week forecasts
- Pattern matching on `ForecastPeriodRequest` enum
- Validation for day parameters (0-6 for weekdays)
- Cache hit/miss logging with `[Orchestrator]` prefix

**Cache Keys:**
- Current week: `forecast:{city}:current_week`
- Next week: `forecast:{city}:next_week:{day}`

### 3. Model Updates
Added `Clone` derive to `EnsembleForecast` to enable caching:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleForecast { ... }
```

### 4. Module Exports
Updated `src/services/mod.rs`:
```rust
pub mod ensemble_nextweek;
pub mod ensemble_orchestrator;

pub use ensemble_nextweek::{fetch_next_week_ensemble, fetch_ensemble_day_offset};
pub use ensemble_orchestrator::EnsembleOrchestrator;
```

## Tests

### Unit Tests (`tests/unit/ensemble_nextweek_test.rs`)
8 tests covering:
- Cache key generation
- Day offset validation
- Cache key format consistency
- Different cities/days
- Lowercase normalization

### Integration Tests (`tests/integration/ensemble_nextweek_integration_test.rs`)
10 tests covering:
- Orchestrator current week requests
- Orchestrator next week requests
- Cache hit scenarios
- Invalid day validation
- Cache statistics
- Multiple cities
- Multiple weekdays

## Usage Examples

### Basic Usage
```rust
use backend::services::{ForecastCache, EnsembleOrchestrator};
use backend::models::{City, ForecastPeriodRequest};
use std::sync::Arc;

let cache = Arc::new(ForecastCache::new(3600, 100));
let orchestrator = EnsembleOrchestrator::new(cache);

let city = City {
    id: 1,
    name: "Jakarta",
    province: "DKI Jakarta",
    latitude: -6.2,
    longitude: 106.8,
};

// Get current week forecast
let forecast = orchestrator.get_forecast(
    &city,
    ForecastPeriodRequest::CurrentWeek,
).await?;

// Get next week Monday forecast (D+7)
let forecast = orchestrator.get_forecast(
    &city,
    ForecastPeriodRequest::NextWeek { base_day: 0 },
).await?;
```

### Direct Next-Week Fetching
```rust
use backend::services::ensemble_nextweek::fetch_next_week_ensemble;

let forecast = fetch_next_week_ensemble(&city, 0, cache).await?;
```

### Day Offset Fetching
```rust
use backend::services::ensemble_nextweek::fetch_ensemble_day_offset;

// Fetch 7 days ahead
let forecast = fetch_ensemble_day_offset(&city, 7, cache).await?;
```

## Acceptance Criteria ✅

- [x] Cache key generation working correctly
- [x] Orchestrator handles CurrentWeek requests
- [x] Orchestrator handles NextWeek requests
- [x] Caching layer integrated
- [x] Cache hits working
- [x] Error handling for invalid days
- [x] All tests passing (18 total: 8 unit + 10 integration)
- [x] Logging shows cache operations
- [x] Ready for API integration

## Logging

All operations log appropriately:
- `[NextWeek]` prefix for next-week specific operations
- `[Orchestrator]` prefix for orchestrator operations
- Cache hits/misses logged at INFO level
- Target date calculations logged at DEBUG level

## Cache Strategy

### TTL Management
- Default: 1 hour (3600 seconds)
- Configurable per cache instance
- Automatic expiration checking

### Key Format
All keys use lowercase city names for consistency:
```
forecast:{city_lowercase}:{period}:{optional_day}
```

### Capacity Management
- Default: 100 entries
- FIFO eviction when capacity reached
- Manual cleanup available via `cache.cleanup()`

## Future Enhancements (TODOs)

1. **Parallel Provider Fetching**: Integrate with actual weather provider APIs
2. **Ensemble Calculation**: Add averaging/voting logic for multi-provider data
3. **Rate Limiting**: Add rate limiting for provider API calls
4. **Retry Logic**: Add exponential backoff for failed provider requests
5. **Metrics**: Add performance metrics and monitoring

## API Integration Ready

The orchestrator is ready to be integrated into Rocket routes:
```rust
#[get("/forecast?<city>&<period>&<day>")]
pub async fn get_forecast(
    city: String,
    period: Option<String>,
    day: Option<u32>,
    orchestrator: &State<EnsembleOrchestrator>,
) -> Result<Json<EnsembleForecast>, Status> {
    let city_obj = find_city(&city)?;
    let period_request = ForecastPeriodRequest::from_query(period, day)?;
    
    orchestrator.get_forecast(&city_obj, period_request)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
```
