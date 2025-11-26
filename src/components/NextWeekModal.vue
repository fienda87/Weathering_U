<template>
  <div 
    v-if="isOpen" 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
    @click.self="closeModal"
  >
    <div class="bg-white rounded-lg shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden">
      <!-- Modal Header -->
      <div class="bg-gradient-to-r from-blue-500 to-blue-600 text-white px-6 py-4 flex items-center justify-between">
        <h2 class="text-xl font-bold">Next Week Forecast (D+7)</h2>
        <button
          @click="closeModal"
          class="text-white hover:text-gray-200 transition-colors duration-200 focus:outline-none"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-6 overflow-y-auto max-h-[calc(90vh-80px)]">
        <!-- Loading State -->
        <div v-if="loading" class="flex flex-col items-center justify-center py-12">
          <div class="animate-spin rounded-full h-16 w-16 border-b-4 border-blue-500 mb-4"></div>
          <p class="text-gray-600 text-lg">Fetching next week forecast...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="error" class="text-center py-12">
          <div class="text-red-500 mb-4">
            <svg class="w-16 h-16 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
          </div>
          <p class="text-red-600 text-lg font-semibold mb-4">{{ error }}</p>
          <button
            @click="retryFetch"
            class="px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Retry
          </button>
        </div>

        <!-- Forecast Comparison -->
        <div v-else-if="nextWeekData && thisWeekData" class="space-y-6">
          <!-- Comparison Grid -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- This Week Card -->
            <div class="bg-white border-l-4 border-green-500 rounded-lg shadow-md p-6">
              <div class="flex items-center mb-4">
                <div class="w-3 h-3 bg-green-500 rounded-full mr-2"></div>
                <h3 class="text-lg font-bold text-gray-800">This Week</h3>
              </div>
              <div class="space-y-3">
                <div>
                  <p class="text-sm text-gray-600 mb-1">Date</p>
                  <p class="font-medium text-gray-900">{{ formatDate(thisWeekData.date) }}</p>
                  <p class="text-xs text-gray-500">{{ thisWeekData.date }}</p>
                </div>
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <p class="text-sm text-gray-600 mb-1">Temp Max</p>
                    <p class="text-2xl font-bold text-red-600">{{ thisWeekData.final_forecast.temp_max.toFixed(1) }}°C</p>
                  </div>
                  <div>
                    <p class="text-sm text-gray-600 mb-1">Temp Min</p>
                    <p class="text-2xl font-bold text-blue-600">{{ thisWeekData.final_forecast.temp_min.toFixed(1) }}°C</p>
                  </div>
                </div>
                <div>
                  <p class="text-sm text-gray-600 mb-1">Condition</p>
                  <p class="font-medium text-gray-900">{{ thisWeekData.final_forecast.condition }}</p>
                </div>
                <div>
                  <p class="text-sm text-gray-600 mb-1">Confidence</p>
                  <span 
                    :class="[
                      'inline-block px-3 py-1 rounded-full text-xs font-semibold',
                      getConfidenceBadgeClass(thisWeekData.final_forecast.confidence)
                    ]"
                  >
                    {{ thisWeekData.final_forecast.confidence.toUpperCase() }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Next Week Card -->
            <div class="bg-white border-l-4 border-blue-500 rounded-lg shadow-md p-6">
              <div class="flex items-center mb-4">
                <div class="w-3 h-3 bg-blue-500 rounded-full mr-2"></div>
                <h3 class="text-lg font-bold text-gray-800">Next Week (D+7)</h3>
              </div>
              <div class="space-y-3">
                <div>
                  <p class="text-sm text-gray-600 mb-1">Date</p>
                  <p class="font-medium text-gray-900">{{ formatDate(nextWeekData.date) }}</p>
                  <p class="text-xs text-gray-500">{{ nextWeekData.date }}</p>
                </div>
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <p class="text-sm text-gray-600 mb-1">Temp Max</p>
                    <p class="text-2xl font-bold text-red-600">{{ nextWeekData.final_forecast.temp_max.toFixed(1) }}°C</p>
                  </div>
                  <div>
                    <p class="text-sm text-gray-600 mb-1">Temp Min</p>
                    <p class="text-2xl font-bold text-blue-600">{{ nextWeekData.final_forecast.temp_min.toFixed(1) }}°C</p>
                  </div>
                </div>
                <div>
                  <p class="text-sm text-gray-600 mb-1">Condition</p>
                  <p class="font-medium text-gray-900">{{ nextWeekData.final_forecast.condition }}</p>
                </div>
                <div>
                  <p class="text-sm text-gray-600 mb-1">Confidence</p>
                  <span 
                    :class="[
                      'inline-block px-3 py-1 rounded-full text-xs font-semibold',
                      getConfidenceBadgeClass(nextWeekData.final_forecast.confidence)
                    ]"
                  >
                    {{ nextWeekData.final_forecast.confidence.toUpperCase() }}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- Per-Source Provider Comparison -->
          <div class="bg-gray-50 rounded-lg p-6">
            <h3 class="text-lg font-bold text-gray-800 mb-4">Per-Source Provider Comparison</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <!-- This Week Providers -->
              <div>
                <h4 class="text-sm font-semibold text-gray-700 mb-3">This Week Sources</h4>
                <div class="space-y-3">
                  <!-- Open-Meteo -->
                  <div v-if="thisWeekData.per_source.open_meteo" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex items-center mb-2">
                      <div class="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
                      <h5 class="text-xs font-bold text-gray-700">Open-Meteo</h5>
                    </div>
                    <div class="space-y-1 text-xs">
                      <div class="flex justify-between">
                        <span class="text-gray-600">Max:</span>
                        <span class="font-semibold text-red-600">{{ thisWeekData.per_source.open_meteo.temp_max.toFixed(1) }}°C</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-gray-600">Min:</span>
                        <span class="font-semibold text-blue-600">{{ thisWeekData.per_source.open_meteo.temp_min.toFixed(1) }}°C</span>
                      </div>
                      <div class="text-gray-700">{{ thisWeekData.per_source.open_meteo.condition }}</div>
                    </div>
                  </div>
                  
                  <!-- OpenWeatherMap -->
                  <div v-if="thisWeekData.per_source.open_weather" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex items-center mb-2">
                      <div class="w-2 h-2 bg-orange-500 rounded-full mr-2"></div>
                      <h5 class="text-xs font-bold text-gray-700">OpenWeatherMap</h5>
                    </div>
                    <div class="space-y-1 text-xs">
                      <div class="flex justify-between">
                        <span class="text-gray-600">Max:</span>
                        <span class="font-semibold text-red-600">{{ thisWeekData.per_source.open_weather.temp_max.toFixed(1) }}°C</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-gray-600">Min:</span>
                        <span class="font-semibold text-blue-600">{{ thisWeekData.per_source.open_weather.temp_min.toFixed(1) }}°C</span>
                      </div>
                      <div class="text-gray-700">{{ thisWeekData.per_source.open_weather.condition }}</div>
                    </div>
                  </div>
                  
                  <!-- WeatherAPI -->
                  <div v-if="thisWeekData.per_source.weather_api" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex items-center mb-2">
                      <div class="w-2 h-2 bg-purple-500 rounded-full mr-2"></div>
                      <h5 class="text-xs font-bold text-gray-700">WeatherAPI</h5>
                    </div>
                    <div class="space-y-1 text-xs">
                      <div class="flex justify-between">
                        <span class="text-gray-600">Max:</span>
                        <span class="font-semibold text-red-600">{{ thisWeekData.per_source.weather_api.temp_max.toFixed(1) }}°C</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-gray-600">Min:</span>
                        <span class="font-semibold text-blue-600">{{ thisWeekData.per_source.weather_api.temp_min.toFixed(1) }}°C</span>
                      </div>
                      <div class="text-gray-700">{{ thisWeekData.per_source.weather_api.condition }}</div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Next Week Providers -->
              <div>
                <h4 class="text-sm font-semibold text-gray-700 mb-3">Next Week Sources</h4>
                <div class="space-y-3">
                  <!-- Open-Meteo -->
                  <div v-if="nextWeekData.per_source.open_meteo" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex items-center mb-2">
                      <div class="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
                      <h5 class="text-xs font-bold text-gray-700">Open-Meteo</h5>
                    </div>
                    <div class="space-y-1 text-xs">
                      <div class="flex justify-between">
                        <span class="text-gray-600">Max:</span>
                        <span class="font-semibold text-red-600">{{ nextWeekData.per_source.open_meteo.temp_max.toFixed(1) }}°C</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-gray-600">Min:</span>
                        <span class="font-semibold text-blue-600">{{ nextWeekData.per_source.open_meteo.temp_min.toFixed(1) }}°C</span>
                      </div>
                      <div class="text-gray-700">{{ nextWeekData.per_source.open_meteo.condition }}</div>
                    </div>
                  </div>
                  
                  <!-- OpenWeatherMap -->
                  <div v-if="nextWeekData.per_source.open_weather" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex items-center mb-2">
                      <div class="w-2 h-2 bg-orange-500 rounded-full mr-2"></div>
                      <h5 class="text-xs font-bold text-gray-700">OpenWeatherMap</h5>
                    </div>
                    <div class="space-y-1 text-xs">
                      <div class="flex justify-between">
                        <span class="text-gray-600">Max:</span>
                        <span class="font-semibold text-red-600">{{ nextWeekData.per_source.open_weather.temp_max.toFixed(1) }}°C</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-gray-600">Min:</span>
                        <span class="font-semibold text-blue-600">{{ nextWeekData.per_source.open_weather.temp_min.toFixed(1) }}°C</span>
                      </div>
                      <div class="text-gray-700">{{ nextWeekData.per_source.open_weather.condition }}</div>
                    </div>
                  </div>
                  
                  <!-- WeatherAPI -->
                  <div v-if="nextWeekData.per_source.weather_api" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex items-center mb-2">
                      <div class="w-2 h-2 bg-purple-500 rounded-full mr-2"></div>
                      <h5 class="text-xs font-bold text-gray-700">WeatherAPI</h5>
                    </div>
                    <div class="space-y-1 text-xs">
                      <div class="flex justify-between">
                        <span class="text-gray-600">Max:</span>
                        <span class="font-semibold text-red-600">{{ nextWeekData.per_source.weather_api.temp_max.toFixed(1) }}°C</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-gray-600">Min:</span>
                        <span class="font-semibold text-blue-600">{{ nextWeekData.per_source.weather_api.temp_min.toFixed(1) }}°C</span>
                      </div>
                      <div class="text-gray-700">{{ nextWeekData.per_source.weather_api.condition }}</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, watch } from 'vue'
import { ApiErrorResponse } from '../utils/api-errors'

export default {
  name: 'NextWeekModal',
  props: {
    isOpen: {
      type: Boolean,
      required: true
    },
    selectedDay: {
      type: Object,
      default: null
    },
    city: {
      type: String,
      required: true
    },
    thisWeekData: {
      type: Object,
      default: null
    }
  },
  emits: ['close'],
  setup(props, { emit }) {
    const loading = ref(false)
    const error = ref('')
    const nextWeekData = ref(null)

    const formatDate = (dateString) => {
      try {
        const date = new Date(dateString)
        const options = { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' }
        return date.toLocaleDateString('en-US', options)
      } catch (err) {
        console.error('[NextWeekModal] Error formatting date:', err)
        return dateString
      }
    }

    const getConfidenceBadgeClass = (confidence) => {
      const confidenceLower = confidence.toLowerCase()
      switch (confidenceLower) {
        case 'high':
          return 'bg-green-100 text-green-800'
        case 'medium':
          return 'bg-orange-100 text-orange-800'
        case 'low':
          return 'bg-red-100 text-red-800'
        default:
          return 'bg-gray-100 text-gray-800'
      }
    }

    const fetchNextWeekForecast = async () => {
      if (!props.selectedDay || !props.city) {
        console.warn('[NextWeekModal] Missing selectedDay or city')
        return
      }

      loading.value = true
      error.value = ''
      nextWeekData.value = null

      try {
        console.log(`[NextWeekModal] Fetching next week forecast for city: ${props.city}, dayOfWeek: ${props.selectedDay.dayOfWeek}`)
        
        const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000'
        const url = `${apiUrl}/api/weather/ensemble?city=${encodeURIComponent(props.city)}&period=next_week&day=${props.selectedDay.dayOfWeek}`
        
        console.log(`[NextWeekModal] Fetching from URL: ${url}`)
        
        const response = await fetch(url)

        if (!response.ok) {
          const data = await response.json().catch(() => ({}))
          
          if (response.status === 404) {
            error.value = `City "${props.city}" not found. Please select a valid city.`
            console.warn('[NextWeekModal] City not found:', props.city)
          } else if (response.status === 400) {
            error.value = `Invalid request: ${data.error || 'Please check your inputs'}`
            console.warn('[NextWeekModal] Invalid parameters:', data)
          } else if (response.status === 500) {
            error.value = 'Server error. Please try again later.'
            console.error('[NextWeekModal] Server error:', data)
          } else {
            error.value = data.error || 'Failed to fetch next week forecast'
            console.error('[NextWeekModal] API error:', response.status, data)
          }
          return
        }

        const data = await response.json()
        
        // Validate response has forecast data
        if (!data || !data.days || data.days.length === 0) {
          error.value = 'No forecast data available'
          console.warn('[NextWeekModal] Empty forecast data received')
          return
        }

        nextWeekData.value = data.days[0]
        console.log('[NextWeekModal] Successfully fetched D+7 forecast for', props.city, `with ${data.days.length} days`)

      } catch (err) {
        if (err instanceof ApiErrorResponse) {
          error.value = err.message
          console.error('[NextWeekModal] API Error:', err.status, err.message)
        } else if (err instanceof TypeError) {
          // Network error
          error.value = 'Network error. Please check your connection.'
          console.error('[NextWeekModal] Network error:', err)
        } else {
          error.value = 'An unexpected error occurred'
          console.error('[NextWeekModal] Unexpected error:', err)
        }
      } finally {
        loading.value = false
      }
    }

    const closeModal = () => {
      emit('close')
      nextWeekData.value = null
      error.value = ''
    }

    const retryFetch = () => {
      console.log('[NextWeekModal] Retrying fetch')
      fetchNextWeekForecast()
    }

    watch(
      () => [props.isOpen, props.selectedDay],
      ([isOpen, selectedDay]) => {
        if (isOpen && selectedDay) {
          console.log('[NextWeekModal] Modal opened, fetching forecast')
          fetchNextWeekForecast()
        }
      },
      { immediate: true }
    )

    return {
      loading,
      error,
      nextWeekData,
      formatDate,
      getConfidenceBadgeClass,
      closeModal,
      retryFetch
    }
  }
}
</script>

<style scoped>
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}
</style>
