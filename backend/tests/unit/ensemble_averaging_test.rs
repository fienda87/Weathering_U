#[cfg(test)]
mod tests {
    use backend::services::ensemble::{
        calculate_weighted_temperature,
        calculate_weighted_average_generic,
    };

    #[test]
    fn test_weighted_temperature_all_present() {
        let result = calculate_weighted_temperature(
            Some(32.0), // Open-Meteo (0.4)
            Some(33.0), // OpenWeatherMap (0.35)
            Some(31.5), // WeatherAPI (0.25)
        );

        // Expected: 32.0*0.4 + 33.0*0.35 + 31.5*0.25
        //         = 12.8 + 11.55 + 7.875
        //         = 32.225
        assert!((result - 32.225).abs() < 0.01);
    }

    #[test]
    fn test_weighted_temperature_missing_one() {
        let result = calculate_weighted_temperature(
            Some(32.0), // Open-Meteo (0.4)
            Some(33.0), // OpenWeatherMap (0.35)
            None,       // WeatherAPI missing
        );

        // With 3rd missing, renormalize weights to 0.4/(0.4+0.35) and 0.35/(0.4+0.35)
        // New weights: 0.533 and 0.467
        // Expected: 32.0*0.533 + 33.0*0.467 = 17.056 + 15.411 = 32.467
        assert!((result - 32.467).abs() < 0.01);
    }

    #[test]
    fn test_weighted_temperature_only_one() {
        let result = calculate_weighted_temperature(
            Some(32.0), // Open-Meteo only
            None,
            None,
        );

        // Only Open-Meteo available, should return 32.0
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_weighted_temperature_all_none() {
        let result = calculate_weighted_temperature(None, None, None);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_weighted_temperature_realistic() {
        // Realistic scenario: Jakarta forecast
        let result = calculate_weighted_temperature(
            Some(32.1), // OM
            Some(32.5), // OW
            Some(31.8), // WA
        );

        // Should be around 32.1-32.2
        assert!(result >= 32.0 && result <= 32.3);
    }

    #[test]
    fn test_weighted_average_generic() {
        let values = vec![
            (32.0, 0.4),
            (33.0, 0.35),
            (31.5, 0.25),
        ];

        let result = calculate_weighted_average_generic(values);
        assert!((result - 32.225).abs() < 0.01);
    }

    #[test]
    fn test_weighted_average_empty() {
        let values: Vec<(f32, f32)> = vec![];
        let result = calculate_weighted_average_generic(values);
        assert_eq!(result, 0.0);
    }
}
