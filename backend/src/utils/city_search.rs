use crate::models::City;
use crate::cities::CITIES;

pub fn find_city_by_name(name: &str) -> Option<&'static City> {
    let search_term = name.to_lowercase();
    
    CITIES.iter().find(|city| {
        let city_name = city.name.to_lowercase();
        
        city_name == search_term 
            || city_name.contains(&search_term)
            || search_term.chars().all(|c| city_name.contains(c))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_exact_match() {
        let result = find_city_by_name("Jakarta");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Jakarta");
    }

    #[test]
    fn test_find_case_insensitive() {
        let result = find_city_by_name("jakarta");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Jakarta");
    }

    #[test]
    fn test_find_partial_match() {
        let result = find_city_by_name("jka");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Jakarta");
    }

    #[test]
    fn test_find_not_found() {
        let result = find_city_by_name("NonExistentCity");
        assert!(result.is_none());
    }
}
