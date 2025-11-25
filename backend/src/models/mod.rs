use serde::{Deserialize, Serialize};

pub mod city;
pub mod forecast;
pub mod ensemble;

#[allow(unused_imports)]
pub use city::City;
#[allow(unused_imports)]
pub use forecast::{DailyForecast, WeatherForecast};
#[allow(unused_imports)]
pub use ensemble::{
    DayEnsemble, EnsembleForecast, FinalForecast, PerSourceData, ProviderForecast,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[allow(dead_code)]
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(message),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_range: Option<String>,
}