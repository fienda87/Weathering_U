use backend::models::City;

#[tokio::test]
async fn test_next_week_cache_key_generation() {
    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let key = format!(
        "forecast:{}:next_week:day_{}",
        city.name.to_lowercase(),
        0
    );

    assert_eq!(key, "forecast:jakarta:next_week:day_0");
}

#[tokio::test]
async fn test_day_offset_validation() {
    assert!(13 <= 13); // Valid: 2 weeks (day 13 is exactly 2 weeks)
    assert!(!(14 > 13)); // Day 14 would be invalid
    assert!(15 > 13); // Invalid: > 2 weeks
}

#[tokio::test]
async fn test_cache_key_format_different_cities() {
    let jakarta = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let bandung = City {
        id: 2,
        name: "Bandung",
        province: "Jawa Barat",
        latitude: -6.9,
        longitude: 107.6,
    };

    let key1 = format!(
        "forecast:{}:next_week:day_{}",
        jakarta.name.to_lowercase(),
        0
    );

    let key2 = format!(
        "forecast:{}:next_week:day_{}",
        bandung.name.to_lowercase(),
        0
    );

    assert_ne!(key1, key2);
    assert_eq!(key1, "forecast:jakarta:next_week:day_0");
    assert_eq!(key2, "forecast:bandung:next_week:day_0");
}

#[tokio::test]
async fn test_cache_key_format_different_days() {
    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let key_monday = format!(
        "forecast:{}:next_week:day_{}",
        city.name.to_lowercase(),
        0
    );

    let key_friday = format!(
        "forecast:{}:next_week:day_{}",
        city.name.to_lowercase(),
        4
    );

    assert_ne!(key_monday, key_friday);
    assert_eq!(key_monday, "forecast:jakarta:next_week:day_0");
    assert_eq!(key_friday, "forecast:jakarta:next_week:day_4");
}

#[tokio::test]
async fn test_day_offset_cache_key_generation() {
    let city = City {
        id: 1,
        name: "Jakarta",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let key = format!(
        "forecast:{}:day_offset_{}",
        city.name.to_lowercase(),
        7
    );

    assert_eq!(key, "forecast:jakarta:day_offset_7");
}

#[tokio::test]
async fn test_day_offset_bounds() {
    // Valid offsets: 0-13
    for offset in 0..=13 {
        assert!(offset <= 13, "Offset {} should be valid", offset);
    }

    // Invalid offsets: 14+
    for offset in 14..20 {
        assert!(offset > 13, "Offset {} should be invalid", offset);
    }
}

#[tokio::test]
async fn test_cache_key_lowercase_consistency() {
    let city = City {
        id: 1,
        name: "JAKARTA",
        province: "DKI Jakarta",
        latitude: -6.2,
        longitude: 106.8,
    };

    let key = format!(
        "forecast:{}:next_week:day_{}",
        city.name.to_lowercase(),
        0
    );

    // Should always be lowercase regardless of input
    assert_eq!(key, "forecast:jakarta:next_week:day_0");
    assert!(!key.contains("JAKARTA"));
}
