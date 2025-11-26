/// Calculate weighted average temperature
/// Weights: Open-Meteo=0.4, OpenWeatherMap=0.35, WeatherAPI=0.25
pub fn calculate_weighted_temperature(
    open_meteo: Option<f32>,
    open_weather: Option<f32>,
    weather_api: Option<f32>,
) -> f32 {
    const WEIGHT_OM: f32 = 0.4;
    const WEIGHT_OW: f32 = 0.35;
    const WEIGHT_WA: f32 = 0.25;

    let values_with_weights: Vec<(f32, f32)> = vec![
        (open_meteo, WEIGHT_OM),
        (open_weather, WEIGHT_OW),
        (weather_api, WEIGHT_WA),
    ]
    .into_iter()
    .filter_map(|(value, weight)| value.map(|v| (v, weight)))
    .collect();

    if values_with_weights.is_empty() {
        return 0.0; // No data available
    }

    let (weighted_sum, total_weight): (f32, f32) = values_with_weights
        .iter()
        .fold((0.0, 0.0), |(sum, weight), (val, w)| {
            (sum + val * w, weight + w)
        });

    if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.0
    }
}

/// Calculate weighted average for multiple temperature values
pub fn calculate_weighted_average_generic(
    values_with_weights: Vec<(f32, f32)>,
) -> f32 {
    if values_with_weights.is_empty() {
        return 0.0;
    }

    let (weighted_sum, total_weight): (f32, f32) = values_with_weights
        .iter()
        .fold((0.0, 0.0), |(sum, weight), (val, w)| {
            (sum + val * w, weight + w)
        });

    if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.0
    }
}
