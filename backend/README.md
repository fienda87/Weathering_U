# IndoPrint Backend API

A Rust-based REST API server built with Rocket framework for the IndoPrint website.

## Features

- RESTful API built with Rocket
- CORS support for Vue.js frontend
- Request/response logging
- Environment-based configuration
- Modular structure with routes, services, models, and utilities

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Installation

1. Clone the repository and navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

3. Adjust the configuration in `.env` as needed:
   - `SERVER_PORT`: Port for the API server (default: 8000)
   - `API_BASE_URL`: Base URL for external API calls
   - `API_KEY`: API key for external services (optional)
   - `CORS_ORIGINS`: Comma-separated list of allowed origins
   - `RUST_LOG`: Logging level (info, debug, warn, error)

### Running the Server

Start the development server:
```bash
cargo run
```

The server will start on `http://localhost:8000` by default.

## API Endpoints

### GET /
Returns basic server information and status.

### GET /health
Health check endpoint with server timestamp.

## Project Structure

```
src/
├── main.rs          # Application entry point and Rocket configuration
├── lib.rs           # Module declarations
├── routes/          # API route handlers
│   └── mod.rs
├── services/        # Business logic and external API clients
│   └── mod.rs
├── models/          # Data structures and serialization
│   └── mod.rs
└── utils/           # Configuration and utilities
    └── mod.rs
```

## Dependencies

- `rocket` - Web framework
- `rocket_cors` - CORS handling
- `reqwest` - HTTP client for external APIs
- `serde/serde_json` - Serialization/deserialization
- `tokio` - Async runtime
- `log/env_logger` - Logging
- `chrono` - Date/time handling

## Development

### Adding New Routes

1. Define route handlers in `src/routes/mod.rs`
2. Register routes in the `routes()` function
3. Add any new models to `src/models/mod.rs`

### Adding External Services

1. Implement service logic in `src/services/mod.rs`
2. Add configuration options to `src/utils/mod.rs`
3. Update `.env.example` with new environment variables

## License

This project is part of the IndoPrint website.