use rocket::{get, routes};
use rocket::serde::json::Json;
use serde_json::Value;

#[get("/")]
pub fn index() -> Json<Value> {
    Json(serde_json::json!({
        "message": "IndoPrint API Server",
        "status": "running",
        "version": "0.1.0"
    }))
}

#[get("/health")]
pub fn health() -> Json<Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, health]
}