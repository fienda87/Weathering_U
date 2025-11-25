use rocket::{get, State};
use rocket::serde::json::Json;
use crate::models::{ApiResponse, WeatherForecast};
use crate::services::WeatherService;

#[get("/weather/<city>?<lat>&<lon>")]
pub async fn get_weather(
    city: String,
    lat: f64,
    lon: f64,
    weather_service: &State<WeatherService>,
) -> Json<ApiResponse<WeatherForecast>> {
    match weather_service.get_forecast(&city, lat, lon).await {
        Ok(forecast) => Json(ApiResponse::success(forecast)),
        Err(error) => Json(ApiResponse::error(error)),
    }
}
