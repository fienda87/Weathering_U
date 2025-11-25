use rocket::get;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use log::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

#[get("/health")]
pub fn health() -> Json<HealthResponse> {
    info!("GET /health - Health check");
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
