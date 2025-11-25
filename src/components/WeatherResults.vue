<template>
  <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-12">
    <!-- Loading State -->
    <div v-if="loading" class="space-y-8">
      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-8 animate-pulse">
        <div class="h-8 bg-gray-200 rounded w-48 mb-4"></div>
        <div class="h-4 bg-gray-200 rounded w-32 mb-6"></div>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="h-32 bg-gray-200 rounded-xl"></div>
          <div class="grid grid-cols-2 gap-4">
            <div class="h-16 bg-gray-200 rounded-lg" v-for="n in 4" :key="n"></div>
          </div>
        </div>
      </div>
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-7 gap-4">
        <div v-for="n in 7" :key="`skeleton-${n}`" class="bg-white rounded-xl shadow-md p-6 animate-pulse">
          <div class="h-4 bg-gray-200 rounded w-2/3 mb-4"></div>
          <div class="h-12 bg-gray-200 rounded w-20 mx-auto mb-4"></div>
          <div class="h-3 bg-gray-200 rounded w-full mb-2"></div>
          <div class="h-3 bg-gray-200 rounded w-3/4 mb-2"></div>
          <div class="h-3 bg-gray-200 rounded w-1/2"></div>
        </div>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-10 text-center">
      <div class="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-6">
        <svg class="w-9 h-9 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
      </div>
      <h2 class="text-2xl font-semibold text-gray-900 mb-3">Unable to Load Forecast</h2>
      <p class="text-gray-600 mb-8">{{ error }}</p>
      <button
        @click="$emit('retry')"
        class="bg-blue-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-700 transition-colors duration-200"
      >
        Try Again
      </button>
    </div>

    <!-- Empty State -->
    <div v-else-if="isEmpty" class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-10 text-center">
      <div class="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-6">
        <svg class="w-9 h-9 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2h2"></path>
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 12v9m0 0l-3-3m3 3l3-3"></path>
        </svg>
      </div>
      <h2 class="text-2xl font-semibold text-gray-900 mb-3">No Forecast Available</h2>
      <p class="text-gray-600 mb-8">We couldn't find forecast data for this city. Please try selecting another city.</p>
      <button
        @click="$emit('retry')"
        class="bg-blue-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-700 transition-colors duration-200"
      >
        Refresh
      </button>
    </div>

    <!-- Weather Results -->
    <div v-else class="space-y-8">
      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-8">
        <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-6">
          <div>
            <h2 class="text-3xl font-bold text-gray-900">{{ weatherData.city }}</h2>
            <p class="text-gray-600" v-if="weatherData.province">
              {{ weatherData.province }}, {{ weatherData.country }}
            </p>
            <p class="text-sm text-gray-500 mt-2">
              Last updated: {{ formattedUpdatedTime }}
            </p>
          </div>
          <div class="flex items-center gap-6">
            <div class="text-center">
              <div class="text-6xl mb-2">{{ getWeatherIcon(todayForecast.condition) }}</div>
              <p class="text-xl font-semibold text-gray-900">{{ todayForecast.temp_avg }}°C</p>
              <p class="text-sm text-gray-500 capitalize">{{ todayForecast.condition }}</p>
            </div>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div class="bg-blue-50 rounded-lg p-3">
                <p class="text-gray-500">High / Low</p>
                <p class="text-lg font-semibold text-gray-900">
                  {{ todayForecast.temp_max }}° / {{ todayForecast.temp_min }}°
                </p>
              </div>
              <div class="bg-blue-50 rounded-lg p-3">
                <p class="text-gray-500">Humidity</p>
                <p class="text-lg font-semibold text-gray-900">{{ todayForecast.humidity }}%</p>
              </div>
              <div class="bg-blue-50 rounded-lg p-3">
                <p class="text-gray-500">Wind Speed</p>
                <p class="text-lg font-semibold text-gray-900">{{ todayForecast.wind_speed }} km/h</p>
              </div>
              <div class="bg-blue-50 rounded-lg p-3">
                <p class="text-gray-500">Coordinates</p>
                <p class="text-lg font-semibold text-gray-900">
                  {{ formatCoordinate(weatherData.latitude) }}°, {{ formatCoordinate(weatherData.longitude) }}°
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-8">
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 mb-6">
          <h3 class="text-2xl font-bold text-gray-900">7-Day Forecast</h3>
          <p class="text-xs text-gray-500 sm:hidden">Scroll right to see all days →</p>
        </div>
        <div class="overflow-x-auto scrollbar-hide pb-2">
          <div class="flex gap-4 sm:grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-7">
            <WeatherCard
              v-for="(day, index) in weatherData.forecast"
              :key="`${day.date}-${index}`"
              :date="day.date"
              :temp-max="day.temp_max"
              :temp-min="day.temp_min"
              :temp-avg="day.temp_avg"
              :condition="day.condition"
              :humidity="day.humidity"
              :wind-speed="day.wind_speed"
              :icon="day.icon"
              :is-today="index === 0"
              class="flex-shrink-0 w-64 sm:w-auto"
            />
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script>
import { computed } from 'vue'
import WeatherCard from './WeatherCard.vue'

export default {
  name: 'WeatherResults',
  components: {
    WeatherCard
  },
  props: {
    loading: {
      type: Boolean,
      default: false
    },
    error: {
      type: String,
      default: ''
    },
    weatherData: {
      type: Object,
      default: null
    }
  },
  setup(props) {
    const isEmpty = computed(() => {
      return !props.weatherData || !props.weatherData.forecast || props.weatherData.forecast.length === 0
    })

    const todayForecast = computed(() => {
      if (!props.weatherData || props.weatherData.forecast.length === 0) {
        return {
          temp_avg: 'N/A',
          temp_max: 'N/A',
          temp_min: 'N/A',
          condition: 'Unknown',
          humidity: 'N/A',
          wind_speed: 'N/A'
        }
      }
      return props.weatherData.forecast[0]
    })

    const formattedUpdatedTime = computed(() => {
      if (!props.weatherData?.last_updated) return 'Unknown'
      const date = new Date(props.weatherData.last_updated)
      return date.toLocaleString('en-US', {
        weekday: 'short',
        hour: '2-digit',
        minute: '2-digit'
      })
    })

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
      const lowerCondition = condition?.toLowerCase() || ''
      return icons[lowerCondition] || '🌤️'
    }

    const formatCoordinate = (coord) => {
      if (typeof coord === 'number' && !Number.isNaN(coord)) {
        return coord.toFixed(2)
      }
      const parsed = Number(coord)
      if (!Number.isNaN(parsed)) {
        return parsed.toFixed(2)
      }
      return 'N/A'
    }

    return {
      isEmpty,
      todayForecast,
      formattedUpdatedTime,
      getWeatherIcon,
      formatCoordinate
    }
  }
}
</script>

<style scoped>
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
