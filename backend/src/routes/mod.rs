use rocket::{get, routes};
use rocket::serde::json::Json;
use serde_json::Value;
use log::info;

pub mod weather;
pub mod health;

use weather::{get_cities, get_weather};

#[get("/")]
pub fn index() -> Json<Value> {
    info!("GET / - Index");
    Json(serde_json::json!({
        "message": "IndoPrint API Server",
        "status": "running",
        "version": "0.1.0"
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, health::health, get_cities, get_weather]
}
