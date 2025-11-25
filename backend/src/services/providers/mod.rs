pub mod open_meteo;
pub mod openweather;
pub mod weatherapi;

#[allow(unused_imports)]
pub use open_meteo::{fetch_open_meteo, OpenMeteoResponse, OpenMeteoDaily};
pub use openweather::fetch_openweather;
pub use weatherapi::fetch_weatherapi;
