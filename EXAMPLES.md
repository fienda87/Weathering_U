# Usage Examples

Real-world examples of using the Weather Forecast API.

## Table of Contents

- [Frontend Integration](#frontend-integration)
- [Backend Scripts](#backend-scripts)
- [Testing with cURL](#testing-with-curl)
- [Common Patterns](#common-patterns)

## Frontend Integration

### Vue 3 Component

```vue
<template>
  <div class="weather-widget">
    <select v-model="selectedCity" @change="fetchWeather">
      <option value="">Select a city...</option>
      <option v-for="city in cities" :key="city.id" :value="city.name">
        {{ city.name }}, {{ city.province }}
      </option>
    </select>

    <div v-if="loading" class="loading">
      Loading weather data...
    </div>

    <div v-else-if="error" class="error">
      {{ error }}
      <button @click="fetchWeather">Retry</button>
    </div>

    <div v-else-if="weather" class="forecast">
      <h2>{{ weather.city }}, {{ weather.province }}</h2>
      <div class="days">
        <div v-for="day in weather.forecast" :key="day.date" class="day-card">
          <div class="date">{{ formatDate(day.date) }}</div>
          <div class="icon">{{ getWeatherEmoji(day.icon) }}</div>
          <div class="temps">
            <span class="high">{{ day.temp_max }}°C</span>
            <span class="low">{{ day.temp_min }}°C</span>
          </div>
          <div class="condition">{{ day.condition }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000';

const cities = ref([]);
const selectedCity = ref('');
const weather = ref(null);
const loading = ref(false);
const error = ref(null);

const fetchCities = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/api/cities`);
    const data = await response.json();
    cities.value = data.cities;
  } catch (err) {
    console.error('Failed to fetch cities:', err);
  }
};

const fetchWeather = async () => {
  if (!selectedCity.value) return;
  
  loading.value = true;
  error.value = null;
  weather.value = null;
  
  try {
    const response = await fetch(
      `${API_BASE_URL}/api/weather?city=${encodeURIComponent(selectedCity.value)}`
    );
    
    if (!response.ok) {
      const errorData = await response.json();
      throw new Error(errorData.message);
    }
    
    weather.value = await response.json();
  } catch (err) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
};

const formatDate = (dateStr) => {
  const date = new Date(dateStr);
  return date.toLocaleDateString('id-ID', { 
    weekday: 'short', 
    month: 'short', 
    day: 'numeric' 
  });
};

const getWeatherEmoji = (icon) => {
  const icons = {
    sunny: '☀️',
    cloudy: '☁️',
    rainy: '🌧️',
    snowy: '❄️',
    fog: '🌫️',
    stormy: '⛈️'
  };
  return icons[icon] || '🌤️';
};

onMounted(() => {
  fetchCities();
});
</script>
```

### React Component

```jsx
import { useState, useEffect } from 'react';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8000';

function WeatherWidget() {
  const [cities, setCities] = useState([]);
  const [selectedCity, setSelectedCity] = useState('');
  const [weather, setWeather] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchCities();
  }, []);

  const fetchCities = async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/api/cities`);
      const data = await response.json();
      setCities(data.cities);
    } catch (err) {
      console.error('Failed to fetch cities:', err);
    }
  };

  const fetchWeather = async () => {
    if (!selectedCity) return;
    
    setLoading(true);
    setError(null);
    setWeather(null);
    
    try {
      const response = await fetch(
        `${API_BASE_URL}/api/weather?city=${encodeURIComponent(selectedCity)}`
      );
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message);
      }
      
      const data = await response.json();
      setWeather(data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="weather-widget">
      <select 
        value={selectedCity} 
        onChange={(e) => {
          setSelectedCity(e.target.value);
          fetchWeather();
        }}
      >
        <option value="">Select a city...</option>
        {cities.map(city => (
          <option key={city.id} value={city.name}>
            {city.name}, {city.province}
          </option>
        ))}
      </select>

      {loading && <div className="loading">Loading weather data...</div>}
      
      {error && (
        <div className="error">
          {error}
          <button onClick={fetchWeather}>Retry</button>
        </div>
      )}
      
      {weather && (
        <div className="forecast">
          <h2>{weather.city}, {weather.province}</h2>
          <div className="days">
            {weather.forecast.map(day => (
              <div key={day.date} className="day-card">
                <div className="date">{new Date(day.date).toLocaleDateString()}</div>
                <div className="temps">
                  <span className="high">{day.temp_max}°C</span>
                  <span className="low">{day.temp_min}°C</span>
                </div>
                <div className="condition">{day.condition}</div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

export default WeatherWidget;
```

## Backend Scripts

### Node.js Script

```javascript
// weather-cli.js
const fetch = require('node-fetch');

const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:8000';

async function getCities() {
  const response = await fetch(`${API_BASE_URL}/api/cities`);
  const data = await response.json();
  return data.cities;
}

async function getWeather(cityName) {
  const response = await fetch(
    `${API_BASE_URL}/api/weather?city=${encodeURIComponent(cityName)}`
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(`${error.error}: ${error.message}`);
  }
  
  return await response.json();
}

async function main() {
  const cityName = process.argv[2];
  
  if (!cityName) {
    console.log('Usage: node weather-cli.js <city-name>');
    console.log('\nAvailable cities:');
    const cities = await getCities();
    cities.slice(0, 10).forEach(city => {
      console.log(`  - ${city.name}, ${city.province}`);
    });
    console.log(`  ... and ${cities.length - 10} more`);
    return;
  }
  
  try {
    const weather = await getWeather(cityName);
    
    console.log(`\n🌤️  Weather Forecast for ${weather.city}, ${weather.province}`);
    console.log(`📍 Location: ${weather.latitude}°, ${weather.longitude}°`);
    console.log(`🕒 Last Updated: ${new Date(weather.last_updated).toLocaleString()}\n`);
    
    weather.forecast.forEach((day, index) => {
      const date = new Date(day.date);
      const dayLabel = index === 0 ? 'Today' : date.toLocaleDateString('en-US', { weekday: 'short' });
      
      console.log(`${dayLabel.padEnd(10)} ${day.temp_min}°C - ${day.temp_max}°C`);
      console.log(`           ${day.condition}`);
      console.log(`           💧 ${day.humidity}% humidity, 💨 ${day.wind_speed} km/h\n`);
    });
  } catch (error) {
    console.error(`❌ Error: ${error.message}`);
    process.exit(1);
  }
}

main();
```

Usage:
```bash
node weather-cli.js Jakarta
```

### Python Script

```python
#!/usr/bin/env python3
import requests
import sys
from datetime import datetime

API_BASE_URL = "http://localhost:8000"

def get_cities():
    """Fetch all available cities."""
    response = requests.get(f"{API_BASE_URL}/api/cities")
    response.raise_for_status()
    return response.json()['cities']

def get_weather(city_name):
    """Fetch weather forecast for a city."""
    response = requests.get(
        f"{API_BASE_URL}/api/weather",
        params={"city": city_name}
    )
    response.raise_for_status()
    return response.json()

def format_weather(weather):
    """Format weather data for display."""
    output = []
    output.append(f"\n🌤️  Weather Forecast for {weather['city']}, {weather['province']}")
    output.append(f"📍 Location: {weather['latitude']}°, {weather['longitude']}°")
    
    last_updated = datetime.fromisoformat(weather['last_updated'].replace('Z', '+00:00'))
    output.append(f"🕒 Last Updated: {last_updated.strftime('%Y-%m-%d %H:%M:%S')}\n")
    
    for i, day in enumerate(weather['forecast']):
        day_date = datetime.strptime(day['date'], '%Y-%m-%d')
        day_label = 'Today' if i == 0 else day_date.strftime('%a')
        
        output.append(f"{day_label:<10} {day['temp_min']}°C - {day['temp_max']}°C")
        output.append(f"           {day['condition']}")
        output.append(f"           💧 {day['humidity']}% humidity, 💨 {day['wind_speed']} km/h\n")
    
    return '\n'.join(output)

def main():
    if len(sys.argv) < 2:
        print("Usage: python weather-cli.py <city-name>")
        print("\nAvailable cities:")
        cities = get_cities()
        for city in cities[:10]:
            print(f"  - {city['name']}, {city['province']}")
        print(f"  ... and {len(cities) - 10} more")
        return
    
    city_name = ' '.join(sys.argv[1:])
    
    try:
        weather = get_weather(city_name)
        print(format_weather(weather))
    except requests.HTTPError as e:
        try:
            error = e.response.json()
            print(f"❌ Error: {error['message']}")
        except:
            print(f"❌ Error: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()
```

Usage:
```bash
chmod +x weather-cli.py
./weather-cli.py Jakarta
```

## Testing with cURL

### Basic Tests

```bash
# Test if server is running
curl http://localhost:8000/api/cities

# Get weather for Jakarta
curl "http://localhost:8000/api/weather?city=Jakarta"

# Pretty print with jq
curl -s "http://localhost:8000/api/weather?city=Jakarta" | jq '.'

# Extract only dates and temperatures
curl -s "http://localhost:8000/api/weather?city=Jakarta" | \
  jq '.forecast[] | "\(.date): \(.temp_min)°C - \(.temp_max)°C"'
```

### Error Cases

```bash
# Missing city parameter
curl "http://localhost:8000/api/weather"

# City not found
curl "http://localhost:8000/api/weather?city=InvalidCity"

# Empty city name
curl "http://localhost:8000/api/weather?city="
```

### Testing Different Cities

```bash
#!/bin/bash
# test-cities.sh - Test weather API for multiple cities

cities=("Jakarta" "Bandung" "Surabaya" "Medan" "Yogyakarta")

for city in "${cities[@]}"; do
  echo "Testing $city..."
  response=$(curl -s "http://localhost:8000/api/weather?city=$city")
  
  if echo "$response" | jq -e '.forecast' > /dev/null 2>&1; then
    forecast_count=$(echo "$response" | jq '.forecast | length')
    echo "✅ $city: $forecast_count days of forecast"
  else
    error=$(echo "$response" | jq -r '.message')
    echo "❌ $city: $error"
  fi
  echo ""
done
```

## Common Patterns

### Caching Weather Data

```javascript
// Simple in-memory cache with TTL
class WeatherCache {
  constructor(ttlMinutes = 10) {
    this.cache = new Map();
    this.ttl = ttlMinutes * 60 * 1000;
  }

  get(city) {
    const item = this.cache.get(city);
    if (!item) return null;
    
    if (Date.now() - item.timestamp > this.ttl) {
      this.cache.delete(city);
      return null;
    }
    
    return item.data;
  }

  set(city, data) {
    this.cache.set(city, {
      data,
      timestamp: Date.now()
    });
  }
}

// Usage
const cache = new WeatherCache(10); // 10 minutes TTL

async function getWeatherWithCache(city) {
  // Check cache first
  let weather = cache.get(city);
  if (weather) {
    console.log('Using cached data');
    return weather;
  }
  
  // Fetch from API
  const response = await fetch(
    `http://localhost:8000/api/weather?city=${encodeURIComponent(city)}`
  );
  weather = await response.json();
  
  // Store in cache
  cache.set(city, weather);
  
  return weather;
}
```

### Retry Logic

```javascript
async function fetchWithRetry(url, maxRetries = 3, delay = 1000) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return await response.json();
      }
      
      // Don't retry on 4xx errors
      if (response.status >= 400 && response.status < 500) {
        const error = await response.json();
        throw new Error(error.message);
      }
      
      // Retry on 5xx errors
      if (i < maxRetries - 1) {
        console.log(`Attempt ${i + 1} failed, retrying in ${delay}ms...`);
        await new Promise(resolve => setTimeout(resolve, delay));
        delay *= 2; // Exponential backoff
      }
    } catch (error) {
      if (i === maxRetries - 1) throw error;
      
      console.log(`Attempt ${i + 1} failed, retrying in ${delay}ms...`);
      await new Promise(resolve => setTimeout(resolve, delay));
      delay *= 2;
    }
  }
}
```

### Batch Fetching

```javascript
async function fetchMultipleCities(cityNames) {
  const promises = cityNames.map(city => 
    fetch(`http://localhost:8000/api/weather?city=${encodeURIComponent(city)}`)
      .then(r => r.json())
      .catch(err => ({ city, error: err.message }))
  );
  
  return await Promise.all(promises);
}

// Usage
const forecasts = await fetchMultipleCities(['Jakarta', 'Bandung', 'Surabaya']);
forecasts.forEach(forecast => {
  if (forecast.error) {
    console.error(`Failed to fetch ${forecast.city}: ${forecast.error}`);
  } else {
    console.log(`${forecast.city}: ${forecast.forecast.length} days`);
  }
});
```

### Weather Comparison

```javascript
async function compareWeather(city1, city2) {
  const [weather1, weather2] = await Promise.all([
    fetch(`http://localhost:8000/api/weather?city=${city1}`).then(r => r.json()),
    fetch(`http://localhost:8000/api/weather?city=${city2}`).then(r => r.json())
  ]);
  
  console.log(`\nWeather Comparison: ${city1} vs ${city2}\n`);
  
  weather1.forecast.forEach((day1, i) => {
    const day2 = weather2.forecast[i];
    const date = new Date(day1.date).toLocaleDateString();
    
    console.log(`${date}:`);
    console.log(`  ${city1}: ${day1.temp_min}°C - ${day1.temp_max}°C (${day1.condition})`);
    console.log(`  ${city2}: ${day2.temp_min}°C - ${day2.temp_max}°C (${day2.condition})`);
    
    const tempDiff = Math.abs(day1.temp_avg - day2.temp_avg);
    console.log(`  Temperature difference: ${tempDiff.toFixed(1)}°C\n`);
  });
}

// Usage
compareWeather('Jakarta', 'Bandung');
```

## Performance Monitoring

```javascript
async function fetchWithTiming(city) {
  const startTime = performance.now();
  
  try {
    const response = await fetch(
      `http://localhost:8000/api/weather?city=${encodeURIComponent(city)}`
    );
    const data = await response.json();
    
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    console.log(`Fetched weather for ${city} in ${duration.toFixed(2)}ms`);
    
    return data;
  } catch (error) {
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    console.error(`Failed to fetch ${city} after ${duration.toFixed(2)}ms: ${error.message}`);
    throw error;
  }
}
```

## Integration Testing

```bash
#!/bin/bash
# integration-test.sh - Test complete API workflow

API_URL="http://localhost:8000"

echo "=== Weather API Integration Test ==="
echo ""

# Test 1: Fetch cities
echo "Test 1: Fetching cities..."
cities_response=$(curl -s "$API_URL/api/cities")
cities_count=$(echo "$cities_response" | jq '.cities | length')

if [ "$cities_count" -gt 0 ]; then
  echo "✅ Fetched $cities_count cities"
else
  echo "❌ Failed to fetch cities"
  exit 1
fi

# Test 2: Get first city name
first_city=$(echo "$cities_response" | jq -r '.cities[0].name')
echo ""
echo "Test 2: Fetching weather for $first_city..."

weather_response=$(curl -s "$API_URL/api/weather?city=$first_city")
forecast_count=$(echo "$weather_response" | jq '.forecast | length')

if [ "$forecast_count" -eq 7 ]; then
  echo "✅ Received 7-day forecast for $first_city"
else
  echo "❌ Expected 7 days, got $forecast_count"
  exit 1
fi

# Test 3: Error handling
echo ""
echo "Test 3: Testing error handling..."
error_response=$(curl -s "$API_URL/api/weather?city=InvalidCityXYZ")
error_code=$(echo "$error_response" | jq -r '.error')

if [ "$error_code" = "CITY_NOT_FOUND" ]; then
  echo "✅ Error handling works correctly"
else
  echo "❌ Unexpected error response"
  exit 1
fi

echo ""
echo "=== All tests passed! ==="
```

Make executable and run:
```bash
chmod +x integration-test.sh
./integration-test.sh
```
