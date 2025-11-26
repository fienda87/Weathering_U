#[cfg(test)]
mod tests {
    use backend::services::ensemble::{
        calculate_stddev,
        calculate_cv,
        calculate_confidence,
        calculate_confidence_score,
    };

    #[test]
    fn test_stddev_identical_values() {
        let values = vec![32.0, 32.0, 32.0];
        let stddev = calculate_stddev(values);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn test_stddev_spread_values() {
        let values = vec![30.0, 32.0, 34.0];
        let stddev = calculate_stddev(values);
        // Mean = 32.0
        // Variance = ((30-32)^2 + (32-32)^2 + (34-32)^2) / 3
        //          = (4 + 0 + 4) / 3 = 2.667
        // Stddev = sqrt(2.667) ≈ 1.633
        assert!((stddev - 1.633).abs() < 0.01);
    }

    #[test]
    fn test_stddev_wide_spread() {
        let values = vec![20.0, 30.0, 40.0];
        let stddev = calculate_stddev(values);
        // Mean = 30.0
        // Variance = ((20-30)^2 + (30-30)^2 + (40-30)^2) / 3
        //          = (100 + 0 + 100) / 3 = 66.667
        // Stddev = sqrt(66.667) ≈ 8.165
        assert!((stddev - 8.165).abs() < 0.01);
    }

    #[test]
    fn test_stddev_single_value() {
        let values = vec![32.0];
        let stddev = calculate_stddev(values);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn test_stddev_empty() {
        let values: Vec<f32> = vec![];
        let stddev = calculate_stddev(values);
        assert_eq!(stddev, 0.0);
    }

    #[test]
    fn test_cv_calculation() {
        let values = vec![30.0, 32.0, 34.0];
        let cv = calculate_cv(values);
        // Mean = 32.0, stddev ≈ 1.633
        // CV = 1.633 / 32.0 ≈ 0.051
        assert!((cv - 0.051).abs() < 0.01);
    }

    #[test]
    fn test_cv_zero_mean() {
        let values = vec![0.0, 0.0];
        let cv = calculate_cv(values);
        assert_eq!(cv, 0.0);
    }

    #[test]
    fn test_confidence_high() {
        // Low stddev, high agreement → high confidence
        let temps = vec![32.0, 32.1, 32.2]; // stddev ≈ 0.082
        let confidence = calculate_confidence(temps, 0.95);
        assert_eq!(confidence, "high");
    }

    #[test]
    fn test_confidence_medium_low_stddev_low_agreement() {
        // Low stddev but low agreement → medium
        let temps = vec![32.0, 32.1, 32.2]; // stddev ≈ 0.082
        let confidence = calculate_confidence(temps, 0.5);
        assert_eq!(confidence, "medium");
    }

    #[test]
    fn test_confidence_medium_moderate_stddev() {
        // Moderate stddev → medium confidence
        let temps = vec![30.0, 32.0, 34.0]; // stddev ≈ 1.633
        let confidence = calculate_confidence(temps, 0.9);
        assert_eq!(confidence, "medium");
    }

    #[test]
    fn test_confidence_low_high_stddev() {
        // High stddev → low confidence
        let temps = vec![20.0, 30.0, 40.0]; // stddev ≈ 8.165
        let confidence = calculate_confidence(temps, 0.9);
        assert_eq!(confidence, "low");
    }

    #[test]
    fn test_confidence_low_poor_agreement() {
        // Poor agreement → low confidence
        let temps = vec![32.0, 32.1, 32.2];
        let confidence = calculate_confidence(temps, 0.33);
        assert_eq!(confidence, "low");
    }

    #[test]
    fn test_confidence_score_high() {
        let temps = vec![32.0, 32.1, 32.2];
        let score = calculate_confidence_score(temps, 0.95);
        assert!(score > 0.8); // Should be high
    }

    #[test]
    fn test_confidence_score_low() {
        let temps = vec![20.0, 30.0, 40.0];
        let score = calculate_confidence_score(temps, 0.3);
        assert!(score < 0.3); // Should be low
    }

    #[test]
    fn test_confidence_score_range() {
        for i in 0..10 {
            let temps = vec![32.0 + i as f32, 32.5 + i as f32];
            let score = calculate_confidence_score(temps.clone(), 0.7);
            assert!(score >= 0.0 && score <= 1.0, "Score out of range: {}", score);
        }
    }

    #[test]
    fn test_realistic_high_confidence() {
        // Realistic: all providers agree closely
        let temps = vec![32.0, 32.5, 31.8]; // OM, OW, WA
        let score = calculate_confidence_score(temps, 1.0);
        assert!(score > 0.7);
    }

    #[test]
    fn test_realistic_medium_confidence() {
        // Realistic: some disagreement
        let temps = vec![32.0, 33.0, 31.0];
        let score = calculate_confidence_score(temps, 0.67);
        assert!(score >= 0.3 && score <= 0.7);
    }

    #[test]
    fn test_realistic_low_confidence() {
        // Realistic: large disagreement
        let temps = vec![30.0, 35.0, 28.0];
        let score = calculate_confidence_score(temps, 0.33);
        assert!(score < 0.4);
    }

    #[test]
    fn test_negative_temperatures() {
        // Test with negative temperatures (winter scenario)
        let temps = vec![-5.0, -4.0, -6.0];
        let stddev = calculate_stddev(temps.clone());
        assert!(stddev > 0.0 && stddev < 2.0);
        
        let confidence = calculate_confidence(temps, 0.85);
        assert_eq!(confidence, "high");
    }

    #[test]
    fn test_extreme_temperature_difference() {
        // Test with very large disagreement
        let temps = vec![10.0, 30.0, 50.0];
        let stddev = calculate_stddev(temps.clone());
        assert!(stddev > 15.0);
        
        let confidence = calculate_confidence(temps, 0.8);
        assert_eq!(confidence, "low");
    }

    #[test]
    fn test_confidence_boundary_cases() {
        // Test exact boundary values
        // stddev exactly 2.0 with high agreement
        let temps = vec![30.0, 32.0, 34.0]; // stddev ≈ 1.633
        let confidence1 = calculate_confidence(temps.clone(), 0.75);
        assert_eq!(confidence1, "high");
        
        // Test agreement boundary
        let confidence2 = calculate_confidence(temps, 0.74);
        assert_eq!(confidence2, "medium");
    }
}
