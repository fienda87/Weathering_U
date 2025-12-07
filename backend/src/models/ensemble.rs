use serde::{Deserialize, Serialize};

/// Forecast dari satu provider untuk satu hari
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderForecast {
    pub date: String,
    pub temp_max: f32,
    pub temp_min: f32,
    pub condition: String,
}

/// Data per-source dari 3 provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerSourceData {
    pub open_meteo: Option<ProviderForecast>,
    pub open_weather: Option<ProviderForecast>,
    pub weather_api: Option<ProviderForecast>,
}

/// Hasil ensemble akhir untuk satu hari
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalForecast {
    pub temp_max: f32,
    pub temp_min: f32,
    pub condition: String,
    pub confidence: String, // Tingkat confidence: "high", "medium", "low"
}

/// Gabungan forecast harian (per-source + final)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayEnsemble {
    pub date: String,
    pub per_source: PerSourceData,
    pub final_forecast: FinalForecast,
}

/// Ensemble forecast lengkap untuk 7 hari
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Hitung berapa provider yang kasih data
    pub fn provider_count(&self) -> usize {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter(|p| p.is_some())
            .count()
    }

    /// Ambil semua suhu maksimal yang valid
    pub fn get_max_temperatures(&self) -> Vec<f32> {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter_map(|p| p.as_ref().map(|pf| pf.temp_max))
            .collect()
    }

    /// Ambil semua suhu minimal yang valid
    pub fn get_min_temperatures(&self) -> Vec<f32> {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter_map(|p| p.as_ref().map(|pf| pf.temp_min))
            .collect()
    }

    /// Ambil semua kondisi cuaca
    pub fn get_conditions(&self) -> Vec<String> {
        [&self.open_meteo, &self.open_weather, &self.weather_api]
            .iter()
            .filter_map(|p| p.as_ref().map(|pf| pf.condition.clone()))
            .collect()
    }

    /// Extract semua suhu untuk dirata-ratakan
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
}
