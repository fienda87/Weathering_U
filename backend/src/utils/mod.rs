use std::env;

pub struct Config {
    pub server_port: u16,
    pub api_base_url: String,
    pub api_key: Option<String>,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000);

        let api_base_url = env::var("API_BASE_URL")
            .unwrap_or_else(|_| "https://api.example.com".to_string());

        let api_key = env::var("API_KEY").ok();

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Self {
            server_port,
            api_base_url,
            api_key,
            cors_origins,
        }
    }
}

pub fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
}