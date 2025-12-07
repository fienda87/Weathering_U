use rocket::fairing::AdHoc;
use rocket_cors::{CorsOptions, AllowedOrigins};
use log::info;
use std::sync::Arc;
use tokio::sync::{Notify, Semaphore};

mod routes;
mod services;
mod models;
mod utils;
mod cities;
mod errors;
mod runtime;

use utils::{Config, init_logger};
use routes::routes;
use services::{WeatherService, ForecastCache};
use models::EnsembleForecast;
use runtime::{init_runtime, log_runtime_config, WorkerPool, get_worker_count};

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    dotenvy::dotenv().ok();
    init_logger();
    init_runtime();
    log_runtime_config();
    let config = Config::from_env();

    info!("Starting IndoPrint API server on port {}", config.server_port);

    let worker_count = get_worker_count();
    let worker_pool = Arc::new(WorkerPool::new(worker_count));
    let semaphore = Arc::new(Semaphore::new(3));
    info!("Created rate limiting semaphore with 3 permits");

    let weather_service = WeatherService::new(
        config.openweather_key.clone(),
        config.weatherapi_key.clone(),
    );

    // Cache ensemble forecast: TTL 1 jam, max 100 entries
    let ensemble_cache = Arc::new(ForecastCache::<EnsembleForecast>::new(3600, 100));
    info!("Created ensemble forecast cache with 1 hour TTL");

    let shutdown = Arc::new(Notify::new());
    let shutdown_clone = shutdown.clone();

    // Handle graceful shutdown (CTRL-C / SIGTERM)
    tokio::spawn(async move {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL-C signal handler");
        };

        #[cfg(unix)]
        let terminate = async {
            use tokio::signal::unix::{signal, SignalKind};
            signal(SignalKind::terminate())
                .expect("Failed to install SIGTERM signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            () = ctrl_c => {
                info!("Received CTRL-C signal, initiating graceful shutdown...");
            }
            () = terminate => {
                info!("Received SIGTERM signal, initiating graceful shutdown...");
            }
        }
        shutdown_clone.notify_one();
    });

    let allowed_origins = AllowedOrigins::some_exact(&config.cors_origins);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Put,
            rocket::http::Method::Delete,
            rocket::http::Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS");

    let figment = rocket::Config::figment()
        .merge(("port", config.server_port))
        .merge(("address", "0.0.0.0"));

    let rocket_instance = rocket::custom(figment)
        .manage(weather_service)
        .manage(worker_pool)
        .manage(semaphore)
        .manage(ensemble_cache)
        .manage(config.clone())
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
        .mount("/", routes());

    let launch_task = tokio::spawn(async move {
        let _ = rocket_instance.launch().await;
    });

    shutdown.notified().await;

    info!("Shutdown signal received, waiting for pending tasks to complete...");

    launch_task.abort();

    info!("Server shutdown complete");
}