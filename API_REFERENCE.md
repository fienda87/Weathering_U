# API Reference

Complete technical reference for the Weather Forecast API.

## Base URL

```
http://localhost:8000
```

## Authentication

No authentication is required. The API is publicly accessible for all endpoints.

## Rate Limiting

Currently no rate limiting is implemented. Consider adding rate limiting in production.

## Content Type

All requests and responses use `application/json` content type.

## Endpoints

### GET /api/cities

Retrieve a list of all available Indonesian cities.

#### Request

No parameters required.

```bash
curl -X GET http://localhost:8000/api/cities
```

#### Response

**Status Code:** `200 OK`

**Response Body:**

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

**Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `cities` | Array | Array of city objects |
| `cities[].id` | Number | Unique city identifier |
| `cities[].name` | String | City name |
| `cities[].province` | String | Province name |
| `cities[].latitude` | Number | Latitude coordinate (decimal degrees) |
| `cities[].longitude` | Number | Longitude coordinate (decimal degrees) |

#### Example

```javascript
// Using fetch API
fetch('http://localhost:8000/api/cities')
  .then(response => response.json())
  .then(data => {
    console.log(`Found ${data.cities.length} cities`);
    data.cities.forEach(city => {
      console.log(`${city.name}, ${city.province}`);
    });
  });
```

---

### GET /api/weather

Retrieve a 7-day weather forecast for a specific city.

#### Request

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `city` | String | Yes | Name of the city (case-insensitive, max 50 chars) |

**Example:**

```bash
curl -X GET "http://localhost:8000/api/weather?city=Jakarta"
```

#### Response - Success

**Status Code:** `200 OK`

**Response Body:**

```json
{
  "city": "Jakarta",
  "province": "DKI Jakarta",
  "country": "Indonesia",
  "latitude": -6.2088,
  "longitude": 106.8456,
  "last_updated": "2024-01-15T10:30:00.123456Z",
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
    },
    {
      "date": "2024-01-16",
      "temp_max": 31.2,
      "temp_min": 25.1,
      "temp_avg": 28.1,
      "condition": "Light rain",
      "humidity": 80,
      "wind_speed": 10.3,
      "icon": "rainy"
    }
    // ... 5 more days (7 total)
  ]
}
```

**Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `city` | String | City name |
| `province` | String | Province name |
| `country` | String | Country name (always "Indonesia") |
| `latitude` | Number | Latitude coordinate |
| `longitude` | Number | Longitude coordinate |
| `last_updated` | String | ISO 8601 timestamp of last data update |
| `forecast` | Array | Array of 7 daily forecast objects |
| `forecast[].date` | String | Date in YYYY-MM-DD format |
| `forecast[].temp_max` | Number | Maximum temperature (°C) |
| `forecast[].temp_min` | Number | Minimum temperature (°C) |
| `forecast[].temp_avg` | Number | Average temperature (°C) |
| `forecast[].condition` | String | Human-readable weather condition |
| `forecast[].humidity` | Number | Relative humidity (0-100%) |
| `forecast[].wind_speed` | Number | Wind speed (km/h) |
| `forecast[].icon` | String | Icon identifier for UI |

**Icon Values:**

| Icon | Description |
|------|-------------|
| `sunny` | Clear, sunny weather |
| `cloudy` | Cloudy or partly cloudy |
| `rainy` | Rain or drizzle |
| `snowy` | Snow (rare in Indonesia) |
| `fog` | Fog or mist |
| `stormy` | Thunderstorms |

#### Response - Errors

##### 400 Bad Request - Missing City Parameter

```json
{
  "error": "INVALID_INPUT",
  "message": "Missing required query parameter: city",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

##### 400 Bad Request - Empty City Parameter

```json
{
  "error": "INVALID_INPUT",
  "message": "City name is required",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

##### 400 Bad Request - City Name Too Long

```json
{
  "error": "INVALID_INPUT",
  "message": "City name must not exceed 50 characters",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

##### 404 Not Found - City Not Found

```json
{
  "error": "CITY_NOT_FOUND",
  "message": "City 'UnknownCity' not found in database",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

##### 503 Service Unavailable - Weather Providers Failed

```json
{
  "error": "SERVICE_UNAVAILABLE",
  "message": "All weather providers are currently unavailable. Please try again later.",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

##### 503 Service Unavailable - Timeout

```json
{
  "error": "TIMEOUT",
  "message": "Request timed out. Please try again later.",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

#### Example - JavaScript

```javascript
// Using fetch API with error handling
async function getWeatherForecast(cityName) {
  try {
    const response = await fetch(
      `http://localhost:8000/api/weather?city=${encodeURIComponent(cityName)}`
    );
    
    if (!response.ok) {
      const error = await response.json();
      console.error(`Error ${response.status}:`, error.message);
      throw new Error(error.message);
    }
    
    const data = await response.json();
    console.log(`Weather for ${data.city}, ${data.province}`);
    console.log(`Forecast for ${data.forecast.length} days`);
    
    data.forecast.forEach(day => {
      console.log(
        `${day.date}: ${day.temp_min}°C - ${day.temp_max}°C, ${day.condition}`
      );
    });
    
    return data;
  } catch (error) {
    console.error('Failed to fetch weather:', error);
    throw error;
  }
}

// Usage
getWeatherForecast('Jakarta');
```

#### Example - cURL

```bash
# Basic request
curl "http://localhost:8000/api/weather?city=Jakarta"

# With verbose output
curl -v "http://localhost:8000/api/weather?city=Jakarta"

# Pretty print JSON (requires jq)
curl -s "http://localhost:8000/api/weather?city=Jakarta" | jq '.'

# Extract only temperatures
curl -s "http://localhost:8000/api/weather?city=Jakarta" | jq '.forecast[] | {date, temp_max, temp_min}'

# Handle error
curl -w "\nStatus: %{http_code}\n" "http://localhost:8000/api/weather?city=InvalidCity"
```

#### Example - Python

```python
import requests

def get_weather_forecast(city: str) -> dict:
    """
    Get weather forecast for a city.
    
    Args:
        city: Name of the city
        
    Returns:
        Dictionary with weather forecast data
        
    Raises:
        requests.HTTPError: If request fails
    """
    url = "http://localhost:8000/api/weather"
    params = {"city": city}
    
    response = requests.get(url, params=params)
    response.raise_for_status()  # Raises HTTPError for bad status codes
    
    return response.json()

# Usage
try:
    forecast = get_weather_forecast("Jakarta")
    print(f"Weather for {forecast['city']}, {forecast['province']}")
    
    for day in forecast['forecast']:
        print(f"{day['date']}: {day['temp_min']}°C - {day['temp_max']}°C")
        print(f"  Condition: {day['condition']}, Humidity: {day['humidity']}%")
        
except requests.HTTPError as e:
    error_data = e.response.json()
    print(f"Error: {error_data['message']}")
```

## Error Response Format

All error responses follow a consistent structure:

```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable error description",
  "timestamp": "2024-01-15T10:30:00.123456Z"
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `error` | String | Machine-readable error code (uppercase with underscores) |
| `message` | String | Human-readable error description |
| `timestamp` | String | ISO 8601 timestamp when error occurred |

## HTTP Status Codes

| Code | Description | When Used |
|------|-------------|-----------|
| 200 | OK | Successful request |
| 400 | Bad Request | Invalid input or missing required parameters |
| 404 | Not Found | Requested resource (city) not found |
| 503 | Service Unavailable | Backend service or weather providers unavailable |

## CORS Headers

The API includes CORS headers to allow cross-origin requests from configured origins.

**Default Allowed Origins:**
- `http://localhost:5173` (Vite dev server)
- `http://localhost:3000` (Alternative dev server)

**Allowed Methods:**
- GET
- POST
- PUT
- DELETE
- OPTIONS

**Credentials:** Allowed

Configure additional origins using the `CORS_ORIGINS` environment variable (comma-separated).

## Provider Fallback Strategy

The weather service uses a multi-provider fallback strategy:

1. **Open-Meteo** (Primary)
   - Always attempted first
   - No API key required
   - Timeout: 5 seconds

2. **OpenWeatherMap** (Secondary)
   - Attempted if Open-Meteo fails
   - Requires `OPENWEATHER_API_KEY` environment variable
   - Skipped if API key not configured
   - Timeout: 5 seconds

3. **WeatherAPI** (Tertiary)
   - Attempted if both previous providers fail
   - Requires `WEATHERAPI_KEY` environment variable
   - Skipped if API key not configured
   - Timeout: 5 seconds

If all providers fail, a 503 error is returned.

## Logging

The backend logs all requests and responses:

```
INFO  [2024-01-15T10:30:00Z] GET /api/cities
INFO  [2024-01-15T10:30:00Z] Response status: 200
INFO  [2024-01-15T10:30:15Z] GET /api/weather?city=Jakarta
INFO  [2024-01-15T10:30:15Z] Getting weather forecast for city=Jakarta
INFO  [2024-01-15T10:30:15Z] Successfully retrieved forecast from Open-Meteo
INFO  [2024-01-15T10:30:15Z] Response status: 200
```

## Performance Considerations

- **City lookups**: O(n) linear search (fast for small dataset)
- **Weather requests**: 5-10 seconds max (including provider timeouts)
- **Caching**: Not currently implemented (consider adding for production)
- **Concurrent requests**: Handled by Tokio async runtime

## Security Considerations

- **No authentication**: API is publicly accessible
- **Input validation**: City names are validated and sanitized
- **SQL injection**: Not applicable (no database)
- **Rate limiting**: Not implemented (consider for production)
- **CORS**: Restricted to configured origins

## Production Recommendations

1. **Add caching**: Cache weather data for 10-15 minutes to reduce provider calls
2. **Implement rate limiting**: Prevent abuse with per-IP rate limits
3. **Add monitoring**: Track provider success rates and response times
4. **Enable compression**: Use gzip compression for responses
5. **Add health checks**: Implement `/health` and `/ready` endpoints
6. **Log aggregation**: Send logs to centralized logging service
7. **Add metrics**: Track request counts, error rates, and latencies
8. **HTTPS only**: Use TLS certificates and disable HTTP in production

## Changelog

### Version 1.0.0
- Initial release
- Two endpoints: `/api/cities` and `/api/weather`
- Multi-provider weather service with fallback
- Support for Open-Meteo, OpenWeatherMap, and WeatherAPI
- Comprehensive error handling
- CORS support
