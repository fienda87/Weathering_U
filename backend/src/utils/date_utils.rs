use chrono::{Local, NaiveDate, Datelike, Duration};

/// Represent forecast period type
#[derive(Debug, Clone)]
pub enum ForecastPeriod {
    CurrentWeek,  // Default: today to +6 days
    NextWeek { 
        base_day: u32,  // 0=Mon, 1=Tue, ..., 6=Sun
    },
}

/// Calculate forecast dates based on period
pub fn get_forecast_dates(period: ForecastPeriod) -> Result<Vec<String>, String> {
    match period {
        ForecastPeriod::CurrentWeek => {
            get_current_week_dates()
        }
        ForecastPeriod::NextWeek { base_day } => {
            get_next_week_date(base_day)
        }
    }
}

/// Get 7 dates for current week (today to +6 days)
pub fn get_current_week_dates() -> Result<Vec<String>, String> {
    let today = Local::now().date_naive();
    
    let dates: Vec<String> = (0..7)
        .map(|i| {
            let date = today + Duration::days(i);
            date.format("%Y-%m-%d").to_string()
        })
        .collect();
    
    Ok(dates)
}

/// Get single date for next week same weekday
/// base_day: 0=Monday, 1=Tuesday, ..., 6=Sunday
pub fn get_next_week_date(base_day: u32) -> Result<Vec<String>, String> {
    if base_day > 6 {
        return Err(format!("Invalid day: {}. Must be 0-6 (Mon-Sun)", base_day));
    }

    let today = Local::now().date_naive();
    
    // Get today's weekday as number (0=Monday from chrono's weekday)
    let today_weekday = today.weekday().number_from_monday() - 1; // Convert to 0-6
    
    // Calculate how many days until target weekday this week
    let days_until_target_this_week = if base_day >= today_weekday {
        base_day - today_weekday
    } else {
        // Target is earlier in week, so it's next occurrence
        7 - (today_weekday - base_day)
    };
    
    // Next week = this week's target + 7 days
    let target_date = today + Duration::days((days_until_target_this_week + 7) as i64);
    
    Ok(vec![target_date.format("%Y-%m-%d").to_string()])
}

/// Get day of week name (0=Monday, 6=Sunday)
pub fn get_weekday_name(day_number: u32) -> Result<String, String> {
    match day_number {
        0 => Ok("Monday".to_string()),
        1 => Ok("Tuesday".to_string()),
        2 => Ok("Wednesday".to_string()),
        3 => Ok("Thursday".to_string()),
        4 => Ok("Friday".to_string()),
        5 => Ok("Saturday".to_string()),
        6 => Ok("Sunday".to_string()),
        _ => Err(format!("Invalid day number: {}", day_number)),
    }
}

/// Get day of week number from date string "YYYY-MM-DD"
pub fn get_day_of_week_from_date(date_str: &str) -> Result<u32, String> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|date| date.weekday().number_from_monday() - 1)
        .map_err(|e| format!("Failed to parse date: {}", e))
}

/// Check if today is the selected weekday
pub fn is_today_weekday(base_day: u32) -> Result<bool, String> {
    if base_day > 6 {
        return Err("Invalid day number".to_string());
    }
    
    let today = Local::now().date_naive();
    let today_weekday = today.weekday().number_from_monday() - 1;
    
    Ok(today_weekday == base_day)
}

/// Calculate offset days from today to target weekday
pub fn days_to_weekday(base_day: u32) -> Result<i64, String> {
    if base_day > 6 {
        return Err("Invalid day number".to_string());
    }
    
    let today = Local::now().date_naive();
    let today_weekday = today.weekday().number_from_monday() - 1;
    
    let offset = if base_day >= today_weekday {
        base_day - today_weekday
    } else {
        7 - (today_weekday - base_day)
    };
    
    Ok(offset as i64)
}

/// Get dates between two dates (inclusive)
pub fn get_dates_between(
    start_date: &str,
    end_date: &str,
) -> Result<Vec<String>, String> {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse start date: {}", e))?;
    
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse end date: {}", e))?;
    
    let mut dates = Vec::new();
    let mut current = start;
    
    while current <= end {
        dates.push(current.format("%Y-%m-%d").to_string());
        current = current + Duration::days(1);
    }
    
    Ok(dates)
}
