pub mod open_meteo;
pub mod openweather;
pub mod weatherapi;

pub use open_meteo::fetch_open_meteo;
pub use openweather::fetch_openweather;
pub use weatherapi::fetch_weatherapi;
