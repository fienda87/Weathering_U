use crate::models::PerSourceData;
use log::debug;

/// HIGH: 3 provider, ±2°C, kondisi sama | MEDIUM: 2 provider atau 3 dengan ±4°C | LOW: 1 provider atau beda >4°C
pub fn calculate_confidence(per_source: &PerSourceData, _final_temps: (f32, f32)) -> String {
    let provider_count = per_source.provider_count();
    
    debug!("[ConfidenceCalc] Provider count: {}", provider_count);
    
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
    
    let max_variance = calculate_variance(&max_temps);
    let min_variance = calculate_variance(&min_temps);
    let avg_variance = (max_variance + min_variance) / 2.0;
    
    let condition_agreement = check_condition_agreement(&conditions);
    
    debug!(
        "[ConfidenceCalc] max_variance: {:.2}°C, min_variance: {:.2}°C, avg_variance: {:.2}°C, condition_agreement: {}",
        max_variance, min_variance, avg_variance, condition_agreement
    );
    
    // HIGH: 3 providers dengan variance rendah (<= 2°C) DAN kondisi sama
    if provider_count == 3 && avg_variance <= 2.0 && condition_agreement {
        debug!("[ConfidenceCalc] HIGH - 3 providers, variance {:.2}°C <= 2.0°C, conditions agree", avg_variance);
        return "high".to_string();
    }
    
    // HIGH: 3 providers dengan variance sangat rendah (<= 1.5°C) meskipun kondisi beda
    if provider_count == 3 && avg_variance <= 1.5 {
        debug!("[ConfidenceCalc] HIGH - 3 providers, excellent variance {:.2}°C <= 1.5°C", avg_variance);
        return "high".to_string();
    }
    
    // MEDIUM: 3 providers dengan variance moderat (2-4°C)
    if provider_count == 3 && avg_variance > 2.0 && avg_variance <= 4.0 {
        debug!("[ConfidenceCalc] MEDIUM - 3 providers, moderate variance {:.2}°C (2-4°C)", avg_variance);
        return "medium".to_string();
    }
    
    // MEDIUM: 2 providers dengan variance baik (<= 3°C)
    if provider_count == 2 && avg_variance <= 3.0 {
        debug!("[ConfidenceCalc] MEDIUM - 2 providers, variance {:.2}°C <= 3.0°C", avg_variance);
        return "medium".to_string();
    }
    
    // LOW: Variance tinggi atau disagreement besar
    debug!("[ConfidenceCalc] LOW - variance {:.2}°C > 4.0°C or significant disagreement", avg_variance);
    "low".to_string()
}

fn calculate_variance(temps: &[f32]) -> f32 {
    if temps.is_empty() {
        return 0.0;
    }
    
    if temps.len() == 1 {
        return 0.0;
    }
    
    let sum: f32 = temps.iter().sum();
    let mean = sum / temps.len() as f32;
    
    let max_deviation = temps
        .iter()
        .map(|&t| (t - mean).abs())
        .fold(0.0, f32::max);
    
    max_deviation
}

fn check_condition_agreement(conditions: &[String]) -> bool {
    if conditions.is_empty() {
        return false;
    }
    
    if conditions.len() == 1 {
        return true;
    }
    
    let normalized: Vec<String> = conditions
        .iter()
        .map(|c| normalize_condition(c))
        .collect();
    
    let mut counts = std::collections::HashMap::new();
    for cond in &normalized {
        *counts.entry(cond.clone()).or_insert(0) += 1;
    }
    
    let max_count = counts.values().max().unwrap_or(&0);
    
    if normalized.len() == 2 {
        *max_count == 2
    } else {
        *max_count >= 2
    }
}


fn normalize_condition(condition: &str) -> String {
    let lower = condition.to_lowercase();
    
    const PATTERNS: &[(&[&str], &str)] = &[
        (&["clear", "sunny"], "clear"),
        (&["cloud", "overcast"], "cloudy"),
        (&["rain", "drizzle", "shower"], "rainy"),
        (&["storm", "thunder"], "stormy"),
        (&["snow", "sleet"], "snowy"),
        (&["fog", "mist", "haze"], "foggy"),
    ];
    
    // Find first matching pattern
    PATTERNS
        .iter()
        .find(|(keywords, _)| keywords.iter().any(|kw| lower.contains(kw)))
        .map(|(_, category)| category.to_string())
        .unwrap_or(lower)
}



