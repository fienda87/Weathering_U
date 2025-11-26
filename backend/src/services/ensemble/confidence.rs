use serde::Serialize;
use std::f32;

/// Calculate standard deviation of temperature values
pub fn calculate_stddev(values: Vec<f32>) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    if values.len() == 1 {
        return 0.0; // No variance with single value
    }

    // Calculate mean
    let mean = values.iter().sum::<f32>() / values.len() as f32;

    // Calculate variance
    let variance = values
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f32>() / values.len() as f32;

    // Return standard deviation
    variance.sqrt()
}

/// Calculate coefficient of variation (stddev / mean)
pub fn calculate_cv(values: Vec<f32>) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<f32>() / values.len() as f32;

    if mean.abs() < f32::EPSILON {
        return 0.0;
    }

    let stddev = calculate_stddev(values);
    stddev / mean.abs()
}

/// Determine confidence level based on stddev and agreement
pub fn calculate_confidence(
    temperatures: Vec<f32>,
    condition_agreement: f32,
) -> String {
    let stddev = calculate_stddev(temperatures.clone());
    
    // Thresholds
    const STDDEV_HIGH: f32 = 2.0;
    const STDDEV_MEDIUM: f32 = 4.0;
    const AGREEMENT_HIGH: f32 = 0.75;
    const AGREEMENT_MEDIUM: f32 = 0.5;

    match (stddev, condition_agreement) {
        // High confidence: low stddev AND high agreement
        (s, a) if s < STDDEV_HIGH && a >= AGREEMENT_HIGH => "high".to_string(),
        
        // Medium confidence: moderate stddev OR moderate agreement
        (s, a) if (s >= STDDEV_HIGH && s <= STDDEV_MEDIUM) || 
                   (a >= AGREEMENT_MEDIUM && a < AGREEMENT_HIGH) => "medium".to_string(),
        
        // Low confidence: high stddev OR low agreement
        _ => "low".to_string(),
    }
}

/// Confidence score 0-1 (for detailed metrics)
pub fn calculate_confidence_score(
    temperatures: Vec<f32>,
    condition_agreement: f32,
) -> f32 {
    let stddev = calculate_stddev(temperatures);
    
    // Normalize stddev to 0-1 (0=best, 1=worst)
    let stddev_norm = (stddev / 10.0).min(1.0); // Cap at 1.0
    
    // Factor in agreement
    // Agreement of 1.0 = 100% confidence, 0.0 = 0% confidence
    
    // Combine: score = agreement * (1 - normalized_stddev)
    let score = condition_agreement * (1.0 - stddev_norm);
    
    score.max(0.0).min(1.0) // Clamp to 0-1
}

/// Get confidence tier with detailed metrics
pub fn get_confidence_details(
    max_temps: Vec<f32>,
    min_temps: Vec<f32>,
    condition_agreement: f32,
) -> ConfidenceDetails {
    let avg_temps = [max_temps.clone(), min_temps.clone()]
        .concat();
    
    let max_stddev = calculate_stddev(max_temps);
    let min_stddev = calculate_stddev(min_temps);
    let _avg_stddev = (max_stddev + min_stddev) / 2.0;
    
    let confidence_tier = calculate_confidence(
        avg_temps.clone(),
        condition_agreement,
    );
    
    let score = calculate_confidence_score(avg_temps, condition_agreement);
    
    ConfidenceDetails {
        tier: confidence_tier,
        score,
        max_temp_stddev: max_stddev,
        min_temp_stddev: min_stddev,
        condition_agreement,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfidenceDetails {
    pub tier: String,
    pub score: f32,
    pub max_temp_stddev: f32,
    pub min_temp_stddev: f32,
    pub condition_agreement: f32,
}
