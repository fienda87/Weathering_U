#[cfg(test)]
mod tests {
    use backend::utils::*;

    #[test]
    fn test_full_forecast_period_workflow() {
        // Current week
        let current = ForecastPeriod::CurrentWeek;
        let dates = get_forecast_dates(current).unwrap();
        assert_eq!(dates.len(), 7);
        println!("Current week dates: {:?}", dates);

        // Next week Monday
        let next = ForecastPeriod::NextWeek { base_day: 0 };
        let dates = get_forecast_dates(next).unwrap();
        assert_eq!(dates.len(), 1);
        println!("Next week Monday: {:?}", dates);
    }

    #[test]
    fn test_all_weekdays_next_week() {
        let day_names = [
            "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"
        ];

        for (day_num, day_name) in day_names.iter().enumerate() {
            let result = get_next_week_date(day_num as u32);
            assert!(result.is_ok(), "Failed for day: {}", day_name);
            
            let dates = result.unwrap();
            println!("Next week {}: {}", day_name, dates[0]);
        }
    }

    #[test]
    fn test_date_calculation_accuracy() {
        let today = chrono::Local::now().date_naive();
        
        // Get next Monday
        let next_monday_dates = get_next_week_date(0).unwrap();
        let next_monday = chrono::NaiveDate::parse_from_str(
            &next_monday_dates[0],
            "%Y-%m-%d"
        ).unwrap();
        
        let diff = (next_monday - today).num_days();
        
        // Next Monday should be 1-13 days away
        assert!(diff >= 1 && diff <= 13, "Next Monday distance invalid: {} days", diff);
    }
}
