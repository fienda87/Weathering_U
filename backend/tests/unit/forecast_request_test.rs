#[cfg(test)]
mod tests {
    use backend::models::ForecastPeriodRequest;

    #[test]
    fn test_from_query_current_week_default() {
        let period = ForecastPeriodRequest::from_query(None, None).unwrap();
        match period {
            ForecastPeriodRequest::CurrentWeek => assert!(true),
            _ => panic!("Should be CurrentWeek"),
        }
    }

    #[test]
    fn test_from_query_explicit_current_week() {
        let period = ForecastPeriodRequest::from_query(
            Some("current_week".to_string()),
            None,
        ).unwrap();
        
        match period {
            ForecastPeriodRequest::CurrentWeek => assert!(true),
            _ => panic!("Should be CurrentWeek"),
        }
    }

    #[test]
    fn test_from_query_next_week_monday() {
        let period = ForecastPeriodRequest::from_query(
            Some("next_week".to_string()),
            Some(0),
        ).unwrap();
        
        match period {
            ForecastPeriodRequest::NextWeek { base_day } => {
                assert_eq!(base_day, 0);
            }
            _ => panic!("Should be NextWeek"),
        }
    }

    #[test]
    fn test_from_query_next_week_friday() {
        let period = ForecastPeriodRequest::from_query(
            Some("next_week".to_string()),
            Some(4),
        ).unwrap();
        
        match period {
            ForecastPeriodRequest::NextWeek { base_day } => {
                assert_eq!(base_day, 4);
            }
            _ => panic!("Should be NextWeek"),
        }
    }

    #[test]
    fn test_from_query_next_week_missing_day() {
        let result = ForecastPeriodRequest::from_query(
            Some("next_week".to_string()),
            None,
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 'day'"));
    }

    #[test]
    fn test_from_query_invalid_day_too_high() {
        let result = ForecastPeriodRequest::from_query(
            Some("next_week".to_string()),
            Some(7),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_from_query_invalid_day_100() {
        let result = ForecastPeriodRequest::from_query(
            Some("next_week".to_string()),
            Some(100),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_display_name_current_week() {
        let period = ForecastPeriodRequest::CurrentWeek;
        assert_eq!(period.display_name(), "Current Week");
    }

    #[test]
    fn test_display_name_next_week_monday() {
        let period = ForecastPeriodRequest::NextWeek { base_day: 0 };
        assert_eq!(period.display_name(), "Next Week Monday");
    }

    #[test]
    fn test_display_name_next_week_friday() {
        let period = ForecastPeriodRequest::NextWeek { base_day: 4 };
        assert_eq!(period.display_name(), "Next Week Friday");
    }

    #[test]
    fn test_display_name_next_week_sunday() {
        let period = ForecastPeriodRequest::NextWeek { base_day: 6 };
        assert_eq!(period.display_name(), "Next Week Sunday");
    }

    #[test]
    fn test_all_valid_days_next_week() {
        let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        
        for day_num in 0..7 {
            let result = ForecastPeriodRequest::from_query(
                Some("next_week".to_string()),
                Some(day_num),
            );
            
            assert!(result.is_ok(), "Day {} should be valid", day_num);
            assert!(result.unwrap().display_name().contains(&days[day_num as usize]));
        }
    }

    #[test]
    fn test_default_is_current_week() {
        let period = ForecastPeriodRequest::default();
        match period {
            ForecastPeriodRequest::CurrentWeek => assert!(true),
            _ => panic!("Default should be CurrentWeek"),
        }
    }

    #[test]
    fn test_forecast_period_request_json() {
        use backend::models::ForecastPeriodRequest;

        // CurrentWeek serialization
        let current = ForecastPeriodRequest::CurrentWeek;
        let json = serde_json::to_string(&current).unwrap();
        assert!(json.contains("current_week"));

        // NextWeek serialization
        let next = ForecastPeriodRequest::NextWeek { base_day: 0 };
        let json = serde_json::to_string(&next).unwrap();
        assert!(json.contains("next_week"));
        assert!(json.contains("base_day"));

        // Deserialization
        let deserialized: ForecastPeriodRequest = 
            serde_json::from_str(&json).unwrap();
        match deserialized {
            ForecastPeriodRequest::NextWeek { base_day } => {
                assert_eq!(base_day, 0);
            }
            _ => panic!("Deserialization failed"),
        }
    }
}
