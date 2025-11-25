use backend::cities::CITIES;
use backend::utils::city_search::find_city_by_name;

#[test]
fn test_find_city_jakarta() {
    let result = find_city_by_name("Jakarta");
    assert!(result.is_some());
    let city = result.unwrap();
    assert_eq!(city.name, "Jakarta");
    assert_eq!(city.province, "DKI Jakarta");
}

#[test]
fn test_find_city_bandung() {
    let result = find_city_by_name("Bandung");
    assert!(result.is_some());
    let city = result.unwrap();
    assert_eq!(city.name, "Bandung");
    assert_eq!(city.province, "Jawa Barat");
}

#[test]
fn test_find_city_surabaya() {
    let result = find_city_by_name("Surabaya");
    assert!(result.is_some());
    let city = result.unwrap();
    assert_eq!(city.name, "Surabaya");
    assert_eq!(city.province, "Jawa Timur");
}

#[test]
fn test_case_insensitive_lowercase() {
    let result = find_city_by_name("jakarta");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Jakarta");
}

#[test]
fn test_case_insensitive_uppercase() {
    let result = find_city_by_name("JAKARTA");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Jakarta");
}

#[test]
fn test_case_insensitive_mixed() {
    let result = find_city_by_name("JaKaRtA");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Jakarta");
}

#[test]
fn test_partial_matching() {
    let result = find_city_by_name("jak");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Jakarta");
}

#[test]
fn test_partial_matching_band() {
    let result = find_city_by_name("band");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Bandung");
}

#[test]
fn test_non_existent_city() {
    let result = find_city_by_name("NonExistentCity");
    assert!(result.is_none());
}

#[test]
fn test_non_existent_city_xyz() {
    let result = find_city_by_name("XYZ");
    assert!(result.is_none());
}

#[test]
fn test_empty_string() {
    let result = find_city_by_name("");
    assert!(result.is_none());
}

#[test]
fn test_all_50_cities_count() {
    assert_eq!(CITIES.len(), 50, "Expected exactly 50 cities in the database");
}

#[test]
fn test_all_cities_searchable_by_exact_name() {
    for city in CITIES.iter() {
        let result = find_city_by_name(city.name);
        assert!(result.is_some(), "City {} should be searchable", city.name);
        assert_eq!(result.unwrap().name, city.name);
    }
}

#[test]
fn test_all_cities_searchable_by_lowercase() {
    for city in CITIES.iter() {
        let lowercase_name = city.name.to_lowercase();
        let result = find_city_by_name(&lowercase_name);
        assert!(result.is_some(), "City {} should be searchable in lowercase", city.name);
    }
}

#[test]
fn test_all_cities_have_valid_id() {
    for city in CITIES.iter() {
        assert!(city.id > 0, "City {} should have valid ID", city.name);
        assert!(city.id <= 50, "City {} ID should be <= 50", city.name);
    }
}

#[test]
fn test_all_cities_have_unique_id() {
    use std::collections::HashSet;
    let mut ids = HashSet::new();
    
    for city in CITIES.iter() {
        assert!(ids.insert(city.id), "City ID {} is duplicated", city.id);
    }
    
    assert_eq!(ids.len(), 50);
}

#[test]
fn test_all_cities_have_name() {
    for city in CITIES.iter() {
        assert!(!city.name.is_empty(), "City with ID {} has empty name", city.id);
    }
}

#[test]
fn test_all_cities_have_province() {
    for city in CITIES.iter() {
        assert!(!city.province.is_empty(), "City {} has empty province", city.name);
    }
}

#[test]
fn test_all_cities_have_valid_latitude() {
    for city in CITIES.iter() {
        assert!(city.latitude >= -90.0 && city.latitude <= 90.0, 
                "City {} has invalid latitude: {}", city.name, city.latitude);
    }
}

#[test]
fn test_all_cities_have_valid_longitude() {
    for city in CITIES.iter() {
        assert!(city.longitude >= -180.0 && city.longitude <= 180.0, 
                "City {} has invalid longitude: {}", city.name, city.longitude);
    }
}

#[test]
fn test_indonesian_cities_latitude_range() {
    for city in CITIES.iter() {
        assert!(city.latitude >= -11.0 && city.latitude <= 6.0, 
                "City {} latitude {} is outside Indonesia range", city.name, city.latitude);
    }
}

#[test]
fn test_indonesian_cities_longitude_range() {
    for city in CITIES.iter() {
        assert!(city.longitude >= 95.0 && city.longitude <= 141.0, 
                "City {} longitude {} is outside Indonesia range", city.name, city.longitude);
    }
}

#[test]
fn test_find_city_medan() {
    let result = find_city_by_name("Medan");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Medan");
}

#[test]
fn test_find_city_makassar() {
    let result = find_city_by_name("Makassar");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Makassar");
}

#[test]
fn test_find_city_yogyakarta() {
    let result = find_city_by_name("Yogyakarta");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Yogyakarta");
}

#[test]
fn test_find_city_solo() {
    let result = find_city_by_name("Solo");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Solo");
}

#[test]
fn test_find_city_malang() {
    let result = find_city_by_name("Malang");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Malang");
}

#[test]
fn test_find_city_semarang() {
    let result = find_city_by_name("Semarang");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Semarang");
}

#[test]
fn test_find_city_palembang() {
    let result = find_city_by_name("Palembang");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Palembang");
}

#[test]
fn test_city_coordinates_jakarta() {
    let result = find_city_by_name("Jakarta");
    assert!(result.is_some());
    let city = result.unwrap();
    assert_eq!(city.latitude, -6.2088);
    assert_eq!(city.longitude, 106.8456);
}

#[test]
fn test_city_coordinates_bandung() {
    let result = find_city_by_name("Bandung");
    assert!(result.is_some());
    let city = result.unwrap();
    assert_eq!(city.latitude, -6.9175);
    assert_eq!(city.longitude, 107.6191);
}

#[test]
fn test_city_coordinates_surabaya() {
    let result = find_city_by_name("Surabaya");
    assert!(result.is_some());
    let city = result.unwrap();
    assert_eq!(city.latitude, -7.2575);
    assert_eq!(city.longitude, 112.7521);
}

#[test]
fn test_all_cities_unique_names() {
    use std::collections::HashSet;
    let mut names = HashSet::new();
    
    for city in CITIES.iter() {
        assert!(names.insert(city.name), "City name {} is duplicated", city.name);
    }
    
    assert_eq!(names.len(), 50);
}

#[test]
fn test_cities_from_different_provinces() {
    use std::collections::HashSet;
    let provinces: HashSet<&str> = CITIES.iter().map(|c| c.province).collect();
    
    assert!(provinces.len() > 1, "Cities should be from multiple provinces");
}

#[test]
fn test_partial_match_case_insensitive() {
    let result = find_city_by_name("JAK");
    assert!(result.is_some());
}

#[test]
fn test_search_stability() {
    let result1 = find_city_by_name("Jakarta");
    let result2 = find_city_by_name("Jakarta");
    
    assert!(result1.is_some());
    assert!(result2.is_some());
    assert_eq!(result1.unwrap().id, result2.unwrap().id);
}
