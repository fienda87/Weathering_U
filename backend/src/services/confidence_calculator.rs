use crate::models::PerSourceData;
use log::debug;

/// Calculate confidence level based on provider agreement
/// 
/// Logic:
/// - HIGH: All 3 providers available AND (temperature agreement within ±2°C AND condition matches)
/// - MEDIUM: 2 providers available AND data consistent, OR 3 providers with slight variance (±3°C)
/// - LOW: Only 1 provider available, OR providers disagree significantly (>±3°C or different conditions)
pub fn calculate_confidence(per_source: &PerSourceData, final_temps: (f32, f32)) -> String {
    let provider_count = per_source.provider_count();
    
    debug!("[ConfidenceCalc] Provider count: {}", provider_count);
    
    // LOW: Only 1 provider or no providers
    if provider_count <= 1 {
        debug!("[ConfidenceCalc] Only {} provider(s), returning LOW", provider_count);
        return "low".to_string();
    }
    
    let max_temps = per_source.get_max_temperatures();
    let min_temps = per_source.get_min_temperatures();
    let conditions = per_source.get_conditions();
    
    if max_temps.is_empty() || min_temps.is_empty() {
        debug!("[ConfidenceCalc] Missing temperature data, returning LOW");
        return "low".to_string();
    }
    
    // Calculate temperature variance
    let max_variance = calculate_variance(&max_temps);
    let min_variance = calculate_variance(&min_temps);
    let avg_variance = (max_variance + min_variance) / 2.0;
    
    // Check condition agreement (do most providers agree?)
    let condition_agreement = check_condition_agreement(&conditions);
    
    debug!(
        "[ConfidenceCalc] max_variance: {:.2}°C, min_variance: {:.2}°C, avg_variance: {:.2}°C, condition_agreement: {}",
        max_variance, min_variance, avg_variance, condition_agreement
    );
    
    // HIGH: 3 providers + tight temp agreement (±2°C) + condition agreement
    if provider_count == 3 && avg_variance <= 2.0 && condition_agreement {
        debug!("[ConfidenceCalc] 3 providers with tight agreement, returning HIGH");
        return "high".to_string();
    }
    
    // MEDIUM: 2 providers with good agreement OR 3 providers with moderate variance (±3°C)
    if provider_count == 2 && avg_variance <= 3.0 {
        debug!("[ConfidenceCalc] 2 providers with good agreement, returning MEDIUM");
        return "medium".to_string();
    }
    
    if provider_count == 3 && avg_variance <= 3.0 {
        debug!("[ConfidenceCalc] 3 providers with moderate variance, returning MEDIUM");
        return "medium".to_string();
    }
    
    // LOW: Everything else (significant disagreement)
    debug!("[ConfidenceCalc] Significant disagreement detected, returning LOW");
    "low".to_string()
}

/// Calculate variance (max difference from mean) for a set of temperatures
fn calculate_variance(temps: &[f32]) -> f32 {
    if temps.is_empty() {
        return 0.0;
    }
    
    if temps.len() == 1 {
        return 0.0;
    }
    
    let sum: f32 = temps.iter().sum();
    let mean = sum / temps.len() as f32;
    
    // Calculate max deviation from mean
    let max_deviation = temps
        .iter()
        .map(|&t| (t - mean).abs())
        .fold(0.0, f32::max);
    
    max_deviation
}

/// Check if most conditions agree (simple majority for 3 providers, all for 2 providers)
fn check_condition_agreement(conditions: &[String]) -> bool {
    if conditions.is_empty() {
        return false;
    }
    
    if conditions.len() == 1 {
        return true;
    }
    
    // Normalize conditions for comparison (lowercase, basic keywords)
    let normalized: Vec<String> = conditions
        .iter()
        .map(|c| normalize_condition(c))
        .collect();
    
    // Count occurrences
    let mut counts = std::collections::HashMap::new();
    for cond in &normalized {
        *counts.entry(cond.clone()).or_insert(0) += 1;
    }
    
    // Find most common condition
    let max_count = counts.values().max().unwrap_or(&0);
    
    // For 2 providers, both must match (max_count == 2)
    // For 3 providers, at least 2 must match (max_count >= 2)
    if normalized.len() == 2 {
        *max_count == 2
    } else {
        *max_count >= 2
    }
}

/// Normalize condition strings for comparison
fn normalize_condition(condition: &str) -> String {
    let lower = condition.to_lowercase();
    
    // Map to basic categories
    if lower.contains("clear") || lower.contains("sunny") {
        "clear".to_string()
    } else if lower.contains("cloud") || lower.contains("overcast") {
        "cloudy".to_string()
    } else if lower.contains("rain") || lower.contains("drizzle") || lower.contains("shower") {
        "rainy".to_string()
    } else if lower.contains("storm") || lower.contains("thunder") {
        "stormy".to_string()
    } else if lower.contains("snow") || lower.contains("sleet") {
        "snowy".to_string()
    } else if lower.contains("fog") || lower.contains("mist") || lower.contains("haze") {
        "foggy".to_string()
    } else {
        // Return as-is if no match
        lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProviderForecast;

    #[test]
    fn test_confidence_high_all_providers_tight_agreement() {
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.0,
                22.0,
                "Sunny".to_string(),
            ))
            .with_open_weather(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.5,
                22.5,
                "Clear".to_string(),
            ))
            .with_weather_api(ProviderForecast::new(
                "2024-01-01".to_string(),
                29.5,
                21.5,
                "Clear Sky".to_string(),
            ));

        let confidence = calculate_confidence(&per_source, (30.0, 22.0));
        assert_eq!(confidence, "high");
    }

    #[test]
    fn test_confidence_medium_two_providers_good_agreement() {
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.0,
                22.0,
                "Sunny".to_string(),
            ))
            .with_open_weather(ProviderForecast::new(
                "2024-01-01".to_string(),
                31.0,
                23.0,
                "Clear".to_string(),
            ));

        let confidence = calculate_confidence(&per_source, (30.5, 22.5));
        assert_eq!(confidence, "medium");
    }

    #[test]
    fn test_confidence_medium_three_providers_moderate_variance() {
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.0,
                22.0,
                "Sunny".to_string(),
            ))
            .with_open_weather(ProviderForecast::new(
                "2024-01-01".to_string(),
                32.0,
                24.0,
                "Clear".to_string(),
            ))
            .with_weather_api(ProviderForecast::new(
                "2024-01-01".to_string(),
                31.0,
                23.0,
                "Partly Cloudy".to_string(),
            ));

        let confidence = calculate_confidence(&per_source, (31.0, 23.0));
        assert_eq!(confidence, "medium");
    }

    #[test]
    fn test_confidence_low_one_provider_only() {
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.0,
                22.0,
                "Sunny".to_string(),
            ));

        let confidence = calculate_confidence(&per_source, (30.0, 22.0));
        assert_eq!(confidence, "low");
    }

    #[test]
    fn test_confidence_low_large_temperature_disagreement() {
        let per_source = PerSourceData::new()
            .with_open_meteo(ProviderForecast::new(
                "2024-01-01".to_string(),
                30.0,
                22.0,
                "Sunny".to_string(),
            ))
            .with_open_weather(ProviderForecast::new(
                "2024-01-01".to_string(),
                35.0,
                27.0,
                "Clear".to_string(),
            ))
            .with_weather_api(ProviderForecast::new(
                "2024-01-01".to_string(),
                25.0,
                18.0,
                "Cloudy".to_string(),
            ));

        let confidence = calculate_confidence(&per_source, (30.0, 22.3));
        assert_eq!(confidence, "low");
    }

    #[test]
    fn test_variance_calculation_single_value() {
        let temps = vec![30.0];
        let variance = calculate_variance(&temps);
        assert_eq!(variance, 0.0);
    }

    #[test]
    fn test_variance_calculation_multiple_values() {
        let temps = vec![28.0, 30.0, 32.0];
        let variance = calculate_variance(&temps);
        // Mean is 30.0, max deviation is 2.0
        assert!((variance - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_condition_agreement_all_match() {
        let conditions = vec![
            "Sunny".to_string(),
            "Clear".to_string(),
            "Clear Sky".to_string(),
        ];
        assert!(check_condition_agreement(&conditions));
    }

    #[test]
    fn test_condition_agreement_majority() {
        let conditions = vec![
            "Sunny".to_string(),
            "Clear".to_string(),
            "Rainy".to_string(),
        ];
        assert!(check_condition_agreement(&conditions));
    }

    #[test]
    fn test_condition_agreement_no_match_two_providers() {
        let conditions = vec![
            "Sunny".to_string(),
            "Rainy".to_string(),
        ];
        assert!(!check_condition_agreement(&conditions));
    }

    #[test]
    fn test_normalize_condition_clear() {
        assert_eq!(normalize_condition("Sunny"), "clear");
        assert_eq!(normalize_condition("Clear Sky"), "clear");
    }

    #[test]
    fn test_normalize_condition_rainy() {
        assert_eq!(normalize_condition("Light Rain"), "rainy");
        assert_eq!(normalize_condition("Heavy Showers"), "rainy");
    }

    #[test]
    fn test_confidence_no_providers() {
        let per_source = PerSourceData::new();
        let confidence = calculate_confidence(&per_source, (30.0, 22.0));
        assert_eq!(confidence, "low");
    }
}
