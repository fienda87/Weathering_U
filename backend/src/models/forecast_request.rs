use serde::{Deserialize, Serialize};

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
}

impl Default for ForecastPeriodRequest {
    fn default() -> Self {
        ForecastPeriodRequest::CurrentWeek
    }
}
