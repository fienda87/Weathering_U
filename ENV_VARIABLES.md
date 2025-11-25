# Environment Variables

This project uses Vite environment variables to configure the API connection and other settings.

## Required Environment Variables

### VITE_API_BASE_URL
- **Description**: Base URL for the backend API server
- **Default**: `http://localhost:8000`
- **Usage**: Used by CitySelector and Weather components to fetch data from the backend
- **Example**: `VITE_API_BASE_URL=https://api.weatherapp.com`

## Optional Environment Variables

### VITE_PORT
- **Description**: Port for the Vite development server
- **Default**: `5173`
- **Usage**: Configures the development server port

## Setup Instructions

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file with your configuration:
   ```env
   # API Configuration
   VITE_API_BASE_URL=http://localhost:8000
   
   # Development Configuration
   VITE_PORT=5173
   ```

3. Restart the development server for changes to take effect.

## Component Usage

### CitySelector Component
- Fetches cities from `${VITE_API_BASE_URL}/api/cities`
- Handles loading states, errors, and debounced search
- Supports keyboard navigation and validation

### Weather Component
- Fetches weather data from `${VITE_API_BASE_URL}/api/weather?city=<city>`
- Displays current weather and 7-day forecast
- Includes error handling and retry functionality

## Production Deployment

For production, make sure to set the `VITE_API_BASE_URL` to your production API endpoint:

```env
VITE_API_BASE_URL=https://your-production-api.com
```

Note: Vite automatically prefixes environment variables with `VITE_` to make them available in the client-side code.