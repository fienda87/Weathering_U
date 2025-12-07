use chrono::{Local, Datelike, Duration};

/// CurrentWeek: hari ini + 6 hari | NextWeek: satu hari spesifik minggu depan
#[derive(Debug, Clone)]
pub enum ForecastPeriod {
    CurrentWeek,
    NextWeek { 
        base_day: u32,  // 0=Senin, 6=Minggu
    },
}

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

/// base_day: 0=Senin, 6=Minggu
pub fn get_next_week_date(base_day: u32) -> Result<Vec<String>, String> {
    if base_day > 6 {
        return Err(format!("Invalid day: {}. Must be 0-6 (Mon-Sun)", base_day));
    }

    let today = Local::now().date_naive();
    
    let today_weekday = today.weekday().number_from_monday() - 1;
    

    let days_until_target_this_week = if base_day >= today_weekday {
        base_day - today_weekday
    } else {
        // Target lebih awal di minggu ini, jadi ambil minggu depan
        7 - (today_weekday - base_day)
    };
    
    let target_date = today + Duration::days((days_until_target_this_week + 7) as i64);
    
    Ok(vec![target_date.format("%Y-%m-%d").to_string()])
}


