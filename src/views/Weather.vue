<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-400 via-blue-500 to-blue-600">
    <!-- Header -->
    <section class="relative">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div class="flex items-center justify-between">
          <div>
            <button
              @click="goBack"
              class="flex items-center space-x-2 text-white hover:text-blue-200 transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
              </svg>
              <span>Back to Home</span>
            </button>
          </div>
          <h1 class="text-3xl font-bold text-white">Weather Forecast</h1>
          <div class="w-20"></div> <!-- Spacer for centering -->
        </div>
      </div>
    </section>

    <!-- Loading State -->
    <section v-if="loading" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-12">
        <div class="text-center">
          <div class="w-16 h-16 bg-blue-600 rounded-full flex items-center justify-center mx-auto mb-4 animate-pulse">
            <svg class="w-8 h-8 text-white animate-spin" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
          </div>
          <h2 class="text-2xl font-semibold text-gray-900 mb-2">Loading Weather Data</h2>
          <p class="text-gray-600">Fetching forecast information for {{ cityName }}...</p>
        </div>
      </div>
    </section>

    <!-- Error State -->
    <section v-else-if="error" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-12">
        <div class="text-center">
          <div class="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg class="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
          </div>
          <h2 class="text-2xl font-semibold text-gray-900 mb-2">Unable to Load Weather</h2>
          <p class="text-gray-600 mb-6">{{ error }}</p>
          <button
            @click="retryFetch"
            class="bg-blue-600 text-white px-6 py-3 rounded-lg font-semibold hover:bg-blue-700 transition-colors duration-200"
          >
            Try Again
          </button>
        </div>
      </div>
    </section>

    <!-- Weather Display -->
    <section v-else-if="weatherData" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-12">
      <!-- Current Weather Card -->
      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-8 mb-8">
        <div class="text-center mb-8">
          <h2 class="text-4xl font-bold text-gray-900 mb-2">
            {{ weatherData.city }}
          </h2>
          <p v-if="weatherData.province" class="text-xl text-gray-600">
            {{ weatherData.province }}
          </p>
        </div>

        <!-- Current Weather -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-8">
          <div class="text-center">
            <div class="text-6xl mb-4">{{ getWeatherIcon(weatherData.forecast[0]?.condition || 'sunny') }}</div>
            <div class="text-3xl font-bold text-gray-900 mb-2">
              {{ weatherData.forecast[0]?.temp_avg || 'N/A' }}°C
            </div>
            <div class="text-xl text-gray-600 capitalize">
              {{ weatherData.forecast[0]?.condition || 'Unknown' }}
            </div>
          </div>
          <div class="grid grid-cols-2 gap-4 content-center">
            <div class="bg-blue-50 rounded-lg p-4">
              <div class="text-sm text-gray-600 mb-1">High / Low</div>
              <div class="text-lg font-semibold text-gray-900">
                {{ weatherData.forecast[0]?.temp_max || 'N/A' }}° / {{ weatherData.forecast[0]?.temp_min || 'N/A' }}°
              </div>
            </div>
            <div class="bg-blue-50 rounded-lg p-4">
              <div class="text-sm text-gray-600 mb-1">Humidity</div>
              <div class="text-lg font-semibold text-gray-900">
                {{ weatherData.forecast[0]?.humidity || 'N/A' }}%
              </div>
            </div>
            <div class="bg-blue-50 rounded-lg p-4">
              <div class="text-sm text-gray-600 mb-1">Wind Speed</div>
              <div class="text-lg font-semibold text-gray-900">
                {{ weatherData.forecast[0]?.wind_speed || 'N/A' }} km/h
              </div>
            </div>
            <div class="bg-blue-50 rounded-lg p-4">
              <div class="text-sm text-gray-600 mb-1">Updated</div>
              <div class="text-lg font-semibold text-gray-900">
                {{ formatTime(weatherData.last_updated) }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 7-Day Forecast -->
      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-8">
        <h3 class="text-2xl font-bold text-gray-900 mb-6">7-Day Forecast</h3>
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-7 gap-4">
          <div
            v-for="(day, index) in weatherData.forecast"
            :key="index"
            class="bg-gray-50 rounded-lg p-4 text-center hover:bg-gray-100 transition-colors duration-200"
            :class="{ 'ring-2 ring-blue-500': index === 0 }"
          >
            <div class="font-semibold text-gray-900 mb-2">
              {{ index === 0 ? 'Today' : formatDate(day.date) }}
            </div>
            <div class="text-3xl mb-2">
              {{ getWeatherIcon(day.condition) }}
            </div>
            <div class="text-sm text-gray-600 capitalize mb-2">
              {{ day.condition }}
            </div>
            <div class="text-sm font-medium text-gray-900">
              {{ day.temp_max }}° / {{ day.temp_min }}°
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'

export default {
  name: 'Weather',
  setup() {
    const route = useRoute()
    const router = useRouter()
    
    const loading = ref(true)
    const error = ref('')
    const weatherData = ref(null)
    const cityName = ref('')

    const fetchWeatherData = async (city) => {
      loading.value = true
      error.value = ''
      
      try {
        const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000'
        const response = await fetch(`${apiUrl}/api/weather?city=${encodeURIComponent(city)}`)
        
        if (!response.ok) {
          const errorData = await response.json().catch(() => ({}))
          throw new Error(errorData.error?.message || `HTTP ${response.status}: ${response.statusText}`)
        }
        
        const data = await response.json()
        weatherData.value = data
        cityName.value = city
      } catch (err) {
        console.error('Error fetching weather data:', err)
        error.value = err.message || 'Failed to fetch weather data. Please try again.'
      } finally {
        loading.value = false
      }
    }

    const goBack = () => {
      router.push('/')
    }

    const retryFetch = () => {
      const city = route.query.city
      if (city) {
        fetchWeatherData(city)
      }
    }

    const formatDate = (dateString) => {
      const date = new Date(dateString)
      return date.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })
    }

    const formatTime = (dateString) => {
      const date = new Date(dateString)
      return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
    }

    const getWeatherIcon = (condition) => {
      const icons = {
        'sunny': '☀️',
        'clear': '☀️',
        'cloudy': '☁️',
        'partly cloudy': '⛅',
        'overcast': '☁️',
        'rainy': '🌧️',
        'rain': '🌧️',
        'light rain': '🌦️',
        'heavy rain': '⛈️',
        'thunderstorm': '⛈️',
        'stormy': '⛈️',
        'snowy': '❄️',
        'snow': '❄️',
        'fog': '🌫️',
        'foggy': '🌫️',
        'mist': '🌫️',
        'windy': '💨'
      }
      
      const lowerCondition = condition.toLowerCase()
      return icons[lowerCondition] || '🌤️'
    }

    onMounted(() => {
      const city = route.query.city
      if (city) {
        fetchWeatherData(city)
      } else {
        error.value = 'No city specified. Please go back and select a city.'
        loading.value = false
      }
    })

    return {
      loading,
      error,
      weatherData,
      cityName,
      goBack,
      retryFetch,
      formatDate,
      formatTime,
      getWeatherIcon
    }
  }
}
</script>