#[tokio::test]
async fn test_weighted_averaging_integration() {
    use backend::services::ensemble::calculate_weighted_temperature;

    // Simulate 3 API responses
    let om_temp: f32 = 32.0;
    let ow_temp: f32 = 33.0;
    let wa_temp: f32 = 31.5;

    let avg = calculate_weighted_temperature(
        Some(om_temp),
        Some(ow_temp),
        Some(wa_temp),
    );

    // Verify result is between min and max
    let min = om_temp.min(ow_temp).min(wa_temp);
    let max = om_temp.max(ow_temp).max(wa_temp);

    assert!(avg >= min && avg <= max);
    println!("Weighted average: {}, between {} and {}", avg, min, max);
}

#[tokio::test]
async fn test_voting_integration() {
    use backend::services::ensemble::majority_vote_condition;

    // Simulate voting from 3 providers
    let conditions = vec![
        Some("Partly Cloudy".to_string()),
        Some("Cloudy".to_string()),
        Some("Cloudy".to_string()),
    ];

    let (winner, agreement) = majority_vote_condition(conditions);

    assert!(!winner.is_empty());
    assert!(agreement > 0.0 && agreement <= 1.0);
    println!("Voting winner: {} with {:.1}% agreement", winner, agreement * 100.0);
}

#[tokio::test]
async fn test_confidence_workflow() {
    use backend::services::ensemble::{
        get_confidence_details,
        calculate_confidence,
    };

    let max_temps = vec![32.0, 33.0, 31.5];
    let min_temps = vec![25.0, 25.5, 24.9];

    let details = get_confidence_details(
        max_temps,
        min_temps,
        0.9,
    );

    assert!(!details.tier.is_empty());
    assert!(details.score >= 0.0 && details.score <= 1.0);
    assert!(details.condition_agreement >= 0.0 && details.condition_agreement <= 1.0);
    
    println!("Confidence Details: {:?}", details);
}
