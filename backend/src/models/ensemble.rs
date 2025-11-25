use serde::{Deserialize, Serialize};

/// Single provider's forecast for one day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderForecast {
    pub date: String,
    pub temp_max: f32,
    pub temp_min: f32,
    pub condition: String,
}

/// Per-source data from all 3 providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerSourceData {
    pub open_meteo: Option<ProviderForecast>,
    pub open_weather: Option<ProviderForecast>,
    pub weather_api: Option<ProviderForecast>,
}

/// Final ensemble result for one day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalForecast {
    pub temp_max: f32,
    pub temp_min: f32,
    pub condition: String,
    pub confidence: String, // "high", "medium", "low"
}

/// Combined daily forecast (per-source + final)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayEnsemble {
    pub date: String,
    pub per_source: PerSourceData,
    pub final_forecast: FinalForecast,
}

/// Complete 7-day ensemble forecast
#[derive(Debug, Serialize, Deserialize)]
pub struct EnsembleForecast {
    pub city: String,
    pub province: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub source_timestamp: String,
    pub days: Vec<DayEnsemble>,
}

impl ProviderForecast {
    pub fn new(date: String, temp_max: f32, temp_min: f32, condition: String) -> Self {
        Self {
            date,
            temp_max,
            temp_min,
            condition,
        }
    }

    /// Validate forecast data is within reasonable bounds
    pub fn is_valid(&self) -> bool {
        // Temperature should be between -50 and 50 for Indonesia
        self.temp_max >= -50.0
            && self.temp_max <= 50.0
            && self.temp_min >= -50.0
            && self.temp_min <= 50.0
            && self.temp_max >= self.temp_min
            && !self.condition.is_empty()
    }
}

impl PerSourceData {
    pub fn new() -> Self {
        Self {
            open_meteo: None,
            open_weather: None,
            weather_api: None,
        }
    }

    pub fn with_open_meteo(mut self, forecast: ProviderForecast) -> Self {
        self.open_meteo = Some(forecast);
        self
    }

    pub fn with_open_weather(mut self, forecast: ProviderForecast) -> Self {
        self.open_weather = Some(forecast);
        self
    }

    pub fn with_weather_api(mut self, forecast: ProviderForecast) -> Self {
        self.weather_api = Some(forecast);
        self
    }

    /// Count how many providers provided data
    pub fn provider_count(&self) -> usize {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter(|p| p.is_some())
            .count()
    }

    /// Get all valid temperatures (temp_max)
    pub fn get_max_temperatures(&self) -> Vec<f32> {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter_map(|p| p.as_ref().map(|pf| pf.temp_max))
            .collect()
    }

    /// Get all valid temperatures (temp_min)
    pub fn get_min_temperatures(&self) -> Vec<f32> {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter_map(|p| p.as_ref().map(|pf| pf.temp_min))
            .collect()
    }

    /// Get all conditions
    pub fn get_conditions(&self) -> Vec<String> {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter_map(|p| p.as_ref().map(|pf| pf.condition.clone()))
            .collect()
    }

    /// Extract all temperatures for averaging
    pub fn extract_temperatures(&self) -> (Vec<f32>, Vec<f32>) {
        let maxes = self.get_max_temperatures();
        let mins = self.get_min_temperatures();
        (maxes, mins)
    }
}

impl FinalForecast {
    pub fn new(temp_max: f32, temp_min: f32, condition: String, confidence: String) -> Self {
        Self {
            temp_max,
            temp_min,
            condition,
            confidence,
        }
    }

    /// Validate final forecast
    pub fn is_valid(&self) -> bool {
        self.temp_max >= -50.0
            && self.temp_max <= 50.0
            && self.temp_min >= -50.0
            && self.temp_min <= 50.0
            && self.temp_max >= self.temp_min
            && !self.condition.is_empty()
            && ["high", "medium", "low"].contains(&self.confidence.as_str())
    }
}

impl DayEnsemble {
    pub fn new(date: String, per_source: PerSourceData, final_forecast: FinalForecast) -> Self {
        Self {
            date,
            per_source,
            final_forecast,
        }
    }
}

impl EnsembleForecast {
    pub fn new(
        city: String,
        province: String,
        country: String,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            city,
            province,
            country,
            latitude,
            longitude,
            source_timestamp: chrono::Local::now().to_rfc3339(),
            days: Vec::new(),
        }
    }

    pub fn add_day(&mut self, day: DayEnsemble) {
        self.days.push(day);
    }

    /// Validate entire forecast
    pub fn is_valid(&self) -> bool {
        self.days.len() == 7
            && self.days.iter().all(|day| {
                day.final_forecast.is_valid() && day.per_source.provider_count() > 0
            })
    }
}
