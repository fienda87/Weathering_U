#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_cors::{CorsOptions, AllowedOrigins};
use log::info;

mod routes;
mod services;
mod models;
mod utils;
mod cities;

use utils::{Config, init_logger};
use routes::routes;
use services::WeatherService;

#[launch]
async fn rocket() -> _ {
    // Initialize configuration
    let config = Config::from_env();
    
    // Initialize logger
    init_logger();
    
    info!("Starting IndoPrint API server on port {}", config.server_port);
    
    // Create weather service
    let weather_service = WeatherService::new(
        config.openweather_key.clone(),
        config.weatherapi_key.clone(),
    );
    
    // Configure CORS
    let allowed_origins = AllowedOrigins::some_exact(&config.cors_origins);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Put,
            rocket::http::Method::Delete,
            rocket::http::Method::Options,
        ].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS");

    // Configure Rocket
    let figment = rocket::Config::figment()
        .merge(("port", config.server_port))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .manage(weather_service)
        .attach(cors)
        .attach(AdHoc::on_request("Request Logger", |req, _| {
            Box::pin(async move {
                info!("{} {}", req.method(), req.uri());
            })
        }))
        .attach(AdHoc::on_response("Response Logger", |_req, res| {
            Box::pin(async move {
                info!("Response status: {}", res.status());
            })
        }))
        .mount("/", routes())
}