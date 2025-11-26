use serde::{Deserialize, Serialize};

/// Forecast request with optional period parameter
#[derive(Debug, Clone, Deserialize)]
pub struct ForecastRequestParams {
    pub city: String,
    pub period: Option<String>,  // "current_week" or "next_week"
    pub day: Option<u32>,        // 0=Mon, 1=Tue, ..., 6=Sun (for next_week)
}

/// Forecast period enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ForecastPeriodRequest {
    #[serde(rename = "current_week")]
    CurrentWeek,
    
    #[serde(rename = "next_week")]
    NextWeek {
        #[serde(rename = "base_day")]
        base_day: u32,
    },
}

impl ForecastPeriodRequest {
    /// Parse from query parameters
    pub fn from_query(period: Option<String>, day: Option<u32>) -> Result<Self, String> {
        match (period.as_deref(), day) {
            (Some("next_week"), Some(d)) if d < 7 => {
                Ok(ForecastPeriodRequest::NextWeek { base_day: d })
            }
            (Some("next_week"), Some(d)) => {
                Err(format!("Invalid day {} for next_week (must be 0-6)", d))
            }
            (Some("next_week"), None) => {
                Err("next_week requires 'day' parameter".to_string())
            }
            _ => Ok(ForecastPeriodRequest::CurrentWeek),
        }
    }

    /// Get display name
    pub fn display_name(&self) -> String {
        match self {
            ForecastPeriodRequest::CurrentWeek => "Current Week".to_string(),
            ForecastPeriodRequest::NextWeek { base_day } => {
                let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
                format!("Next Week {}", days[*base_day as usize])
            }
        }
    }
}

impl Default for ForecastPeriodRequest {
    fn default() -> Self {
        ForecastPeriodRequest::CurrentWeek
    }
}

/// Response wrapper for forecast data
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub period: String,
    pub city: String,
    pub requested_at: String,
    pub forecast_data: serde_json::Value,
}
