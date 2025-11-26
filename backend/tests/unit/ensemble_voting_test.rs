#[cfg(test)]
mod tests {
    use backend::services::ensemble::{
        majority_vote_condition,
        vote_condition,
    };

    #[test]
    fn test_unanimous_agreement() {
        let conditions = vec![
            Some("Cloudy".to_string()),
            Some("Cloudy".to_string()),
            Some("Cloudy".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        assert_eq!(result, "Cloudy");
        assert_eq!(agreement, 1.0); // 3/3 = 100%
    }

    #[test]
    fn test_two_thirds_majority() {
        let conditions = vec![
            Some("Cloudy".to_string()),
            Some("Cloudy".to_string()),
            Some("Rainy".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        assert_eq!(result, "Cloudy");
        assert!((agreement - 0.667).abs() < 0.01); // 2/3
    }

    #[test]
    fn test_one_third_minority() {
        let conditions = vec![
            Some("Rainy".to_string()),
            Some("Cloudy".to_string()),
            Some("Cloudy".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        assert_eq!(result, "Cloudy");
        assert!((agreement - 0.667).abs() < 0.01); // 2/3
    }

    #[test]
    fn test_split_decision_tie() {
        let conditions = vec![
            Some("Cloudy".to_string()),
            Some("Rainy".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        // Should return first winner (deterministic based on HashMap iteration)
        assert!(result == "Cloudy" || result == "Rainy");
        assert_eq!(agreement, 0.5); // 1/2
    }

    #[test]
    fn test_all_different() {
        let conditions = vec![
            Some("Cloudy".to_string()),
            Some("Rainy".to_string()),
            Some("Sunny".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        // Any one would be returned as "winner"
        assert!(["Cloudy", "Rainy", "Sunny"].contains(&result.as_str()));
        assert!((agreement - 0.333).abs() < 0.01); // 1/3
    }

    #[test]
    fn test_with_none_values() {
        let conditions = vec![
            Some("Cloudy".to_string()),
            None,
            Some("Cloudy".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        assert_eq!(result, "Cloudy");
        assert_eq!(agreement, 1.0); // 2/2 valid votes
    }

    #[test]
    fn test_all_none() {
        let conditions: Vec<Option<String>> = vec![None, None, None];

        let (result, agreement) = majority_vote_condition(conditions);
        assert_eq!(result, "Unknown");
        assert_eq!(agreement, 0.0);
    }

    #[test]
    fn test_vote_condition_simple() {
        let conditions = vec![
            Some("Cloudy".to_string()),
            Some("Cloudy".to_string()),
            Some("Rainy".to_string()),
        ];

        let result = vote_condition(conditions);
        assert_eq!(result, "Cloudy");
    }

    #[test]
    fn test_realistic_scenario() {
        // Real scenario: providers disagree
        let conditions = vec![
            Some("Partly Cloudy".to_string()),
            Some("Cloudy".to_string()),
            Some("Cloudy".to_string()),
        ];

        let (result, agreement) = majority_vote_condition(conditions);
        // Could return either depending on exact match
        assert!(result.contains("Cloudy"));
        assert!(agreement >= 0.333);
    }
}
