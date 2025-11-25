use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: u32,
    pub name: &'static str,
    pub province: &'static str,
    pub latitude: f64,
    pub longitude: f64,
}
