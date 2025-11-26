#[cfg(test)]
mod tests {
    use backend::utils::*;
    use chrono::{Local, Datelike};

    #[test]
    fn test_get_current_week_dates() {
        let dates = get_current_week_dates().unwrap();
        
        assert_eq!(dates.len(), 7);
        
        // Verify consecutive dates
        for i in 0..6 {
            let d1 = dates[i].clone();
            let d2 = dates[i + 1].clone();
            
            // Parse dates and verify they're consecutive
            let date1 = chrono::NaiveDate::parse_from_str(&d1, "%Y-%m-%d").unwrap();
            let date2 = chrono::NaiveDate::parse_from_str(&d2, "%Y-%m-%d").unwrap();
            
            let diff = (date2 - date1).num_days();
            assert_eq!(diff, 1, "Dates should be consecutive");
        }
    }

    #[test]
    fn test_get_current_week_starts_today() {
        let dates = get_current_week_dates().unwrap();
        let today = Local::now().date_naive();
        let today_str = today.format("%Y-%m-%d").to_string();
        
        assert_eq!(dates[0], today_str, "First date should be today");
    }

    #[test]
    fn test_next_week_monday() {
        // Get Monday next week
        let result = get_next_week_date(0); // 0 = Monday
        assert!(result.is_ok());
        
        let dates = result.unwrap();
        assert_eq!(dates.len(), 1);
        
        let next_week = chrono::NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d").unwrap();
        let today = Local::now().date_naive();
        
        let diff = (next_week - today).num_days();
        // Should be between 1-13 days (depending on current weekday)
        assert!(diff >= 1 && diff <= 13, "Next week Monday should be 1-13 days away, got {}", diff);
    }

    #[test]
    fn test_next_week_friday() {
        let result = get_next_week_date(4); // 4 = Friday
        assert!(result.is_ok());
        
        let dates = result.unwrap();
        assert_eq!(dates.len(), 1);
        
        let next_week = chrono::NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d").unwrap();
        let today = Local::now().date_naive();
        
        let diff = (next_week - today).num_days();
        assert!(diff >= 1 && diff <= 13);
    }

    #[test]
    fn test_next_week_sunday() {
        let result = get_next_week_date(6); // 6 = Sunday
        assert!(result.is_ok());
        
        let dates = result.unwrap();
        assert_eq!(dates.len(), 1);
    }

    #[test]
    fn test_invalid_day_number() {
        let result = get_next_week_date(7); // Invalid
        assert!(result.is_err());
        
        let result = get_next_week_date(100);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_weekday_name() {
        assert_eq!(get_weekday_name(0).unwrap(), "Monday");
        assert_eq!(get_weekday_name(1).unwrap(), "Tuesday");
        assert_eq!(get_weekday_name(2).unwrap(), "Wednesday");
        assert_eq!(get_weekday_name(3).unwrap(), "Thursday");
        assert_eq!(get_weekday_name(4).unwrap(), "Friday");
        assert_eq!(get_weekday_name(5).unwrap(), "Saturday");
        assert_eq!(get_weekday_name(6).unwrap(), "Sunday");
    }

    #[test]
    fn test_invalid_weekday_name() {
        assert!(get_weekday_name(7).is_err());
        assert!(get_weekday_name(100).is_err());
    }

    #[test]
    fn test_get_day_of_week_from_date() {
        // Use a known date: 2025-11-24 is a Monday
        let day = get_day_of_week_from_date("2025-11-24").unwrap();
        assert_eq!(day, 0); // Monday
    }

    #[test]
    fn test_get_day_of_week_from_date_invalid_format() {
        let result = get_day_of_week_from_date("24-11-2025");
        assert!(result.is_err());
    }

    #[test]
    fn test_days_to_weekday() {
        let days = days_to_weekday(0).unwrap(); // Monday
        // Should be 0-6 depending on today
        assert!(days >= 0 && days <= 6);
    }

    #[test]
    fn test_days_to_weekday_invalid() {
        let result = days_to_weekday(7);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_dates_between() {
        let dates = get_dates_between("2025-11-24", "2025-11-26").unwrap();
        
        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0], "2025-11-24");
        assert_eq!(dates[1], "2025-11-25");
        assert_eq!(dates[2], "2025-11-26");
    }

    #[test]
    fn test_get_dates_between_same_date() {
        let dates = get_dates_between("2025-11-24", "2025-11-24").unwrap();
        assert_eq!(dates.len(), 1);
        assert_eq!(dates[0], "2025-11-24");
    }

    #[test]
    fn test_get_dates_between_invalid_format() {
        let result = get_dates_between("24-11-2025", "26-11-2025");
        assert!(result.is_err());
    }

    #[test]
    fn test_forecast_period_enum() {
        let current = ForecastPeriod::CurrentWeek;
        let next = ForecastPeriod::NextWeek { base_day: 0 };
        
        // Just verify they can be created
        match current {
            ForecastPeriod::CurrentWeek => assert!(true),
            _ => panic!("Wrong variant"),
        }
        
        match next {
            ForecastPeriod::NextWeek { base_day } => assert_eq!(base_day, 0),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_get_forecast_dates_current_week() {
        let dates = get_forecast_dates(ForecastPeriod::CurrentWeek).unwrap();
        assert_eq!(dates.len(), 7);
    }

    #[test]
    fn test_get_forecast_dates_next_week() {
        let dates = get_forecast_dates(ForecastPeriod::NextWeek { base_day: 0 }).unwrap();
        assert_eq!(dates.len(), 1);
    }

    #[test]
    fn test_realistic_scenario_monday() {
        // Test getting next Monday prediction
        let dates = get_next_week_date(0).unwrap();
        let target = chrono::NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d").unwrap();
        
        // Verify target is a Monday
        assert_eq!(target.weekday().number_from_monday(), 1); // 1 = Monday in chrono
    }

    #[test]
    fn test_realistic_scenario_friday() {
        // Test getting next Friday prediction
        let dates = get_next_week_date(4).unwrap();
        let target = chrono::NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d").unwrap();
        
        // Verify target is a Friday
        assert_eq!(target.weekday().number_from_monday(), 5); // 5 = Friday in chrono
    }

    #[test]
    fn test_weekday_consistency() {
        // For all 7 days of week
        for day in 0..7 {
            // Get the date
            let dates = get_next_week_date(day).unwrap();
            let target = chrono::NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d").unwrap();
            
            // Verify weekday matches
            let weekday_num = target.weekday().number_from_monday() - 1;
            assert_eq!(weekday_num, day, "Day {} weekday mismatch", day);
        }
    }
}
