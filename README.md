# Weather Forecast Application

A full-stack weather forecast application with a Vue 3 frontend and Rust backend API. The application provides 7-day weather forecasts for Indonesian cities with a multi-provider fallback strategy.

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Project Structure](#project-structure)
- [Backend Setup](#backend-setup)
- [Frontend Setup](#frontend-setup)
- [API Documentation](#api-documentation)
- [Weather Service Architecture](#weather-service-architecture)
- [Production Deployment](#production-deployment)
- [Troubleshooting](#troubleshooting)

## Features

- 🌤️ 7-day weather forecasts for Indonesian cities
- 🔄 Multi-provider fallback system (Open-Meteo, OpenWeatherMap, WeatherAPI)
- 🎨 Modern, responsive UI built with Vue 3 and Tailwind CSS
- ⚡ Fast REST API built with Rust and Rocket framework
- 🔍 Searchable city selector with debounced input
- 📱 Mobile-friendly design with horizontal scroll on small screens
- 🛡️ Comprehensive error handling and validation
- 🌐 CORS-enabled for cross-origin requests

## Architecture

**Frontend:**
- Vue 3 (Composition API)
- Vue Router for navigation
- Vite for build tooling
- Tailwind CSS for styling

**Backend:**
- Rust with Rocket 0.5 framework
- Multi-provider weather service with fallback
- RESTful API design
- CORS middleware for cross-origin requests

## Prerequisites

### Required Software

1. **Rust Toolchain** (1.70.0 or later)
   ```bash
   # Install Rust using rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Node.js** (18.0.0 or later) and npm
   ```bash
   # Install Node.js from https://nodejs.org/ or use nvm
   node --version
   npm --version
   ```

### Optional API Keys

While the app works out-of-the-box with the free Open-Meteo provider, you can configure additional providers for better reliability:

- **OpenWeatherMap API Key**: [Sign up here](https://openweathermap.org/api)
- **WeatherAPI Key**: [Sign up here](https://www.weatherapi.com/)

## Project Structure

```
.
├── backend/                 # Rust backend API
│   ├── src/
│   │   ├── main.rs         # Server entry point
│   │   ├── routes/         # API route handlers
│   │   ├── services/       # Weather service & providers
│   │   ├── models/         # Data models
│   │   ├── cities.rs       # Indonesian cities database
│   │   ├── errors.rs       # Error handling
│   │   └── utils/          # Configuration & utilities
│   └── Cargo.toml          # Rust dependencies
├── src/                     # Vue 3 frontend
│   ├── components/         # Reusable Vue components
│   ├── views/             # Page components
│   ├── router/            # Vue Router configuration
│   ├── App.vue            # Root component
│   └── main.js            # Frontend entry point
├── dist/                   # Production build output
├── index.html             # HTML entry point
├── package.json           # Node dependencies
├── vite.config.js         # Vite configuration
└── tailwind.config.js     # Tailwind CSS configuration
```

## Backend Setup

### 1. Navigate to Backend Directory

```bash
cd backend
```

### 2. Configure Environment Variables

Create a `.env` file in the `backend/` directory (optional for basic usage):

```bash
# Server Configuration
SERVER_PORT=8000

# CORS Origins (comma-separated)
CORS_ORIGINS=http://localhost:5173,http://localhost:3000

# Weather API Keys (Optional - fallback providers)
OPENWEATHER_API_KEY=your-openweather-api-key
WEATHERAPI_KEY=your-weatherapi-key
```

**Note:** The app will work without API keys using the free Open-Meteo provider.

### 3. Install Dependencies

Cargo will automatically download and compile dependencies:

```bash
cargo build
```

### 4. Run the Backend Server

For development:
```bash
cargo run
```

The server will start on `http://0.0.0.0:8000` by default.

For production-optimized build:
```bash
cargo build --release
./target/release/backend
```

### 5. Verify Backend is Running

Test the cities endpoint:
```bash
curl http://localhost:8000/api/cities
```

You should see a JSON response with a list of Indonesian cities.

## Frontend Setup

### 1. Return to Project Root

```bash
cd ..  # If you're in the backend directory
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Configure Environment Variables

Create a `.env` file in the project root:

```env
# Backend API Base URL
VITE_API_BASE_URL=http://localhost:8000

# Development Server Port (optional)
VITE_PORT=5173
```

You can also copy the example file if it exists:
```bash
cp .env.example .env  # If available
```

### 4. Run the Frontend Development Server

```bash
npm run dev
```

The application will be available at `http://localhost:5173`.

### 5. Access the Application

Open your browser and navigate to:
- **Development:** `http://localhost:5173`

The frontend will communicate with the backend API at `http://localhost:8000`.

## API Documentation

### Base URL

Development: `http://localhost:8000`

### Endpoints

#### GET /api/cities

Returns a list of all available Indonesian cities.

**Request:**
```bash
curl http://localhost:8000/api/cities
```

**Response:** `200 OK`
```json
{
  "cities": [
    {
      "id": 1,
      "name": "Jakarta",
      "province": "DKI Jakarta",
      "latitude": -6.2088,
      "longitude": 106.8456
    },
    {
      "id": 2,
      "name": "Bandung",
      "province": "Jawa Barat",
      "latitude": -6.9175,
      "longitude": 107.6191
    }
    // ... more cities
  ]
}
```

#### GET /api/weather

Fetches a 7-day weather forecast for a specific city.

**Query Parameters:**
- `city` (required): Name of the city (case-insensitive)

**Request:**
```bash
curl "http://localhost:8000/api/weather?city=Jakarta"
```

**Response:** `200 OK`
```json
{
  "city": "Jakarta",
  "province": "DKI Jakarta",
  "country": "Indonesia",
  "latitude": -6.2088,
  "longitude": 106.8456,
  "last_updated": "2024-01-15T10:30:00Z",
  "forecast": [
    {
      "date": "2024-01-15",
      "temp_max": 32.5,
      "temp_min": 24.8,
      "temp_avg": 28.6,
      "condition": "Partly cloudy",
      "humidity": 75,
      "wind_speed": 12.5,
      "icon": "cloudy"
    }
    // ... 6 more days
  ]
}
```

**Weather Icons:**
The `icon` field can be one of: `sunny`, `cloudy`, `rainy`, `snowy`, `fog`, `stormy`

### Error Responses

All error responses follow this structure:

```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable error message",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### Error Codes

| Status Code | Error Code | Description |
|------------|------------|-------------|
| 400 | `INVALID_INPUT` | Missing or invalid city parameter |
| 404 | `CITY_NOT_FOUND` | Requested city not in database |
| 503 | `SERVICE_UNAVAILABLE` | All weather providers failed |
| 503 | `TIMEOUT` | Request timed out |

**Example Error:**
```bash
curl "http://localhost:8000/api/weather?city=NonExistentCity"
```

Response: `404 Not Found`
```json
{
  "error": "CITY_NOT_FOUND",
  "message": "City 'NonExistentCity' not found in database",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Input Validation

**City Parameter:**
- Must not be empty
- Maximum length: 50 characters
- Automatically trimmed of whitespace
- Case-insensitive matching

## Weather Service Architecture

The backend implements a robust multi-provider fallback strategy to ensure maximum uptime and reliability.

### Provider Fallback Order

1. **Open-Meteo** (Primary, Free)
   - No API key required
   - 7-day forecast
   - WMO weather codes
   - Timeout: 5 seconds

2. **OpenWeatherMap** (Fallback 1)
   - Requires API key (`OPENWEATHER_API_KEY`)
   - 5-day forecast (padded to 7 days)
   - Comprehensive weather data
   - Timeout: 5 seconds

3. **WeatherAPI** (Fallback 2)
   - Requires API key (`WEATHERAPI_KEY`)
   - 7-day forecast
   - Icon URLs included
   - Timeout: 5 seconds

### Fallback Logic

1. Request comes in for a city
2. City coordinates are looked up from the local database
3. **Open-Meteo** is attempted first (always available)
4. If Open-Meteo fails, **OpenWeatherMap** is tried (if API key configured)
5. If OpenWeatherMap fails, **WeatherAPI** is tried (if API key configured)
6. If all providers fail, a `503 Service Unavailable` error is returned

### Provider Skipping

Providers are automatically skipped if:
- No API key is configured
- API key is set to placeholder value `"your-key-here"`

This allows the application to work with just the free Open-Meteo provider while gracefully handling optional paid providers.

## Production Deployment

### Backend Production Build

1. Build the optimized binary:
   ```bash
   cd backend
   cargo build --release
   ```

2. The binary will be at `backend/target/release/backend`

3. Set production environment variables:
   ```bash
   export SERVER_PORT=8000
   export CORS_ORIGINS=https://yourdomain.com
   export OPENWEATHER_API_KEY=your-production-key
   export WEATHERAPI_KEY=your-production-key
   ```

4. Run the binary:
   ```bash
   ./target/release/backend
   ```

### Frontend Production Build

1. Update `.env` for production:
   ```env
   VITE_API_BASE_URL=https://api.yourdomain.com
   ```

2. Build the frontend:
   ```bash
   npm run build
   ```

3. Static files will be generated in the `dist/` directory

4. Serve the `dist/` directory with any static file server:
   ```bash
   # Using Python
   python3 -m http.server 8080 --directory dist
   
   # Using Node.js serve package
   npx serve dist
   
   # Using Nginx (recommended for production)
   # Configure nginx to serve the dist/ directory
   ```

### Production Checklist

- [ ] Set production API keys in backend environment
- [ ] Update `VITE_API_BASE_URL` to production backend URL
- [ ] Configure CORS origins to match your frontend domain
- [ ] Enable HTTPS for both frontend and backend
- [ ] Set up proper logging and monitoring
- [ ] Configure firewall rules for port 8000
- [ ] Consider using a process manager (systemd, PM2) for the backend
- [ ] Set up reverse proxy (Nginx) for SSL termination

## Troubleshooting

### CORS Issues

**Problem:** Frontend can't connect to backend due to CORS errors.

**Solution:**
1. Ensure backend is running on the correct port
2. Check that `CORS_ORIGINS` includes your frontend URL
3. Verify `VITE_API_BASE_URL` matches the backend address

Example backend `.env`:
```env
CORS_ORIGINS=http://localhost:5173,http://localhost:3000
```

### Backend Connection Refused

**Problem:** Frontend shows "Failed to fetch" or connection errors.

**Solutions:**
1. Verify backend is running: `curl http://localhost:8000/api/cities`
2. Check backend logs for errors
3. Ensure `SERVER_PORT` matches the port in `VITE_API_BASE_URL`
4. Check firewall rules aren't blocking port 8000

### Weather Data Not Loading

**Problem:** Cities load but weather forecast shows errors.

**Solutions:**
1. Check backend logs for provider failures
2. Verify internet connectivity from backend server
3. Test providers individually:
   ```bash
   curl "http://localhost:8000/api/weather?city=Jakarta"
   ```
4. Configure additional API keys for fallback providers

### Missing API Keys

**Problem:** All weather providers fail consistently.

**Solution:**
Open-Meteo should work without keys. If it fails:
1. Check internet connectivity
2. Verify Open-Meteo service is not down: `https://open-meteo.com/`
3. Add fallback API keys for OpenWeatherMap and WeatherAPI
4. Check backend logs for detailed error messages

### Port Already in Use

**Problem:** `Address already in use` error when starting backend.

**Solutions:**
1. Change the port in backend `.env`:
   ```env
   SERVER_PORT=8001
   ```
2. Update frontend `.env` to match:
   ```env
   VITE_API_BASE_URL=http://localhost:8001
   ```
3. Or kill the process using port 8000:
   ```bash
   # Find process on port 8000
   lsof -i :8000
   # Kill it
   kill -9 <PID>
   ```

### Build Errors

**Backend build fails:**
1. Update Rust toolchain: `rustup update`
2. Clean build artifacts: `cargo clean`
3. Rebuild: `cargo build`

**Frontend build fails:**
1. Delete `node_modules`: `rm -rf node_modules`
2. Delete lock file: `rm package-lock.json`
3. Reinstall: `npm install`
4. Rebuild: `npm run build`

### Development Mode Performance

**Problem:** Slow compilation times during development.

**Solutions:**
1. Use `cargo check` instead of `cargo build` for faster feedback
2. Enable incremental compilation (enabled by default)
3. Consider using `cargo watch` for auto-recompilation:
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

## Additional Resources

- [Rocket Web Framework Documentation](https://rocket.rs/)
- [Vue 3 Documentation](https://vuejs.org/)
- [Vite Documentation](https://vitejs.dev/)
- [Tailwind CSS Documentation](https://tailwindcss.com/)
- [Open-Meteo API](https://open-meteo.com/)
- [OpenWeatherMap API](https://openweathermap.org/api)
- [WeatherAPI Documentation](https://www.weatherapi.com/docs/)

## License

This project is proprietary software for PT. INTI TALENTA ANDALAN.

## Support

For issues or questions, please contact the development team.
