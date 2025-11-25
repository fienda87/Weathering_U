<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-400 via-blue-500 to-blue-600">
    <!-- Header -->
    <section class="relative">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div class="flex items-center justify-between">
          <button
            @click="goBack"
            class="flex items-center space-x-2 text-white hover:text-blue-200 transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
            </svg>
            <span>Back to Home</span>
          </button>
          <h1 class="text-3xl font-bold text-white">Weather Forecast</h1>
          <div class="w-32"></div>
        </div>
      </div>
    </section>

    <!-- Weather Results Component -->
    <WeatherResults
      :loading="loading"
      :error="error"
      :weather-data="weatherData"
      @retry="retryFetch"
    />
  </div>
</template>

<script>
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import WeatherResults from '../components/WeatherResults.vue'

export default {
  name: 'Weather',
  components: {
    WeatherResults
  },
  setup() {
    const route = useRoute()
    const router = useRouter()
    
    const loading = ref(true)
    const error = ref('')
    const weatherData = ref(null)

    const normalizeCityParam = (param) => {
      if (Array.isArray(param)) {
        return param[0]
      }
      return typeof param === 'string' ? param : ''
    }

    const fetchWeatherData = async (city) => {
      if (!city) {
        error.value = 'No city specified. Please go back and select a city.'
        loading.value = false
        return
      }

      loading.value = true
      error.value = ''
      weatherData.value = null
      
      try {
        console.log(`[Weather] Fetching weather data for city: ${city}`)
        
        const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000'
        const response = await fetch(`${apiUrl}/api/weather?city=${encodeURIComponent(city)}`)
        
        if (!response.ok) {
          const errorData = await response.json().catch(() => ({}))
          const errorMessage = errorData.error?.message || `HTTP ${response.status}: ${response.statusText}`
          
          console.error(`[Weather] API error:`, {
            status: response.status,
            statusText: response.statusText,
            errorData
          })
          
          throw new Error(errorMessage)
        }
        
        const data = await response.json()
        
        if (!data || !data.forecast || data.forecast.length === 0) {
          console.warn('[Weather] Received empty forecast data:', data)
          throw new Error('No forecast data available for this city.')
        }
        
        console.log(`[Weather] Successfully fetched ${data.forecast.length} days of forecast for ${data.city}`)
        weatherData.value = data
      } catch (err) {
        console.error('[Weather] Error fetching weather data:', err)
        error.value = err.message || 'Failed to fetch weather data. Please try again.'
      } finally {
        loading.value = false
      }
    }

    const goBack = () => {
      router.push('/')
    }

    const retryFetch = () => {
      const city = normalizeCityParam(route.query.city)
      if (city) {
        console.log('[Weather] Retrying fetch for city:', city)
        fetchWeatherData(city)
      } else {
        error.value = 'No city specified. Please go back and select a city.'
      }
    }

    onMounted(() => {
      const city = normalizeCityParam(route.query.city)
      if (city) {
        fetchWeatherData(city)
      } else {
        console.error('[Weather] No city specified in query parameters')
        error.value = 'No city specified. Please go back and select a city.'
        loading.value = false
      }
    })

    watch(
      () => route.query.city,
      (newCity, oldCity) => {
        const nextCity = normalizeCityParam(newCity)
        const prevCity = normalizeCityParam(oldCity)

        if (!nextCity) {
          weatherData.value = null
          error.value = 'No city specified. Please go back and select a city.'
          return
        }

        if (nextCity !== prevCity) {
          console.log('[Weather] City query changed, fetching new forecast:', nextCity)
          fetchWeatherData(nextCity)
        }
      }
    )

    return {
      loading,
      error,
      weatherData,
      goBack,
      retryFetch
    }
  }
}
</script>
