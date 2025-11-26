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
          <h1 class="text-3xl font-bold text-white">Ensemble Weather Forecast</h1>
          <div class="w-32"></div>
        </div>
      </div>
    </section>

    <!-- Main Content -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-12">
      <!-- Loading State -->
      <div v-if="loading" class="flex flex-col items-center justify-center py-12">
        <div class="animate-spin rounded-full h-16 w-16 border-b-4 border-white mb-4"></div>
        <p class="text-white text-lg">Loading ensemble forecast...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="bg-white rounded-lg shadow-lg p-8 text-center">
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

      <!-- Forecast Table -->
      <div v-else-if="ensembleForecast" class="space-y-6">
        <!-- City Info Header -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold text-gray-800 mb-2">{{ ensembleForecast.city }}</h2>
          <div class="text-gray-600 space-y-1">
            <p>{{ ensembleForecast.province }}, {{ ensembleForecast.country }}</p>
            <p class="text-sm">Coordinates: {{ ensembleForecast.latitude.toFixed(4) }}, {{ ensembleForecast.longitude.toFixed(4) }}</p>
            <p class="text-sm text-gray-500">Last updated: {{ formatTimestamp(ensembleForecast.source_timestamp) }}</p>
          </div>
        </div>

        <!-- Ensemble Forecast Table -->
        <EnsembleForecastTable
          :forecast-data="ensembleForecast"
          @next-week-click="handleNextWeekClick"
        />
      </div>
    </section>

    <!-- Next Week Modal -->
    <NextWeekModal
      :is-open="showNextWeekModal"
      :selected-day="selectedDay"
      :city="cityName"
      :this-week-data="selectedDayData"
      @close="closeModal"
    />
  </div>
</template>

<script>
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import EnsembleForecastTable from '../components/EnsembleForecastTable.vue'
import NextWeekModal from '../components/NextWeekModal.vue'

export default {
  name: 'Weather',
  components: {
    EnsembleForecastTable,
    NextWeekModal
  },
  setup() {
    const route = useRoute()
    const router = useRouter()
    
    const loading = ref(true)
    const error = ref('')
    const ensembleForecast = ref(null)
    const showNextWeekModal = ref(false)
    const selectedDay = ref(null)
    const selectedDayIndex = ref(null)

    const cityName = computed(() => {
      return normalizeCityParam(route.query.city)
    })

    const selectedDayData = computed(() => {
      if (selectedDayIndex.value !== null && ensembleForecast.value?.days) {
        return ensembleForecast.value.days[selectedDayIndex.value]
      }
      return null
    })

    const normalizeCityParam = (param) => {
      if (Array.isArray(param)) {
        return param[0]
      }
      return typeof param === 'string' ? param : ''
    }

    const formatTimestamp = (timestamp) => {
      try {
        const date = new Date(timestamp)
        return date.toLocaleString('en-US', {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit'
        })
      } catch (err) {
        console.error('[Weather] Error formatting timestamp:', err)
        return timestamp
      }
    }

    const fetchEnsembleForecast = async (city) => {
      if (!city) {
        error.value = 'No city specified. Please go back and select a city.'
        loading.value = false
        return
      }

      loading.value = true
      error.value = ''
      ensembleForecast.value = null
      
      try {
        console.log(`[Weather] Fetching ensemble forecast for city: ${city}`)
        
        const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000'
        const url = `${apiUrl}/api/weather/ensemble?city=${encodeURIComponent(city)}`
        
        console.log(`[Weather] Fetching from URL: ${url}`)
        
        const response = await fetch(url)
        
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
        
        if (!data || !data.days || data.days.length === 0) {
          console.warn('[Weather] Received empty ensemble forecast data:', data)
          throw new Error('No ensemble forecast data available for this city.')
        }
        
        console.log(`[Weather] Successfully fetched ensemble forecast with ${data.days.length} days for ${data.city}`)
        ensembleForecast.value = data
      } catch (err) {
        console.error('[Weather] Error fetching ensemble forecast:', err)
        error.value = err.message || 'Failed to fetch ensemble forecast. Please try again.'
      } finally {
        loading.value = false
      }
    }

    const handleNextWeekClick = (payload) => {
      console.log('[Weather] Next week click event received:', payload)
      selectedDay.value = payload
      selectedDayIndex.value = payload.dayIndex
      showNextWeekModal.value = true
    }

    const closeModal = () => {
      console.log('[Weather] Closing next week modal')
      showNextWeekModal.value = false
      selectedDay.value = null
      selectedDayIndex.value = null
    }

    const goBack = () => {
      router.push('/')
    }

    const retryFetch = () => {
      const city = cityName.value
      if (city) {
        console.log('[Weather] Retrying fetch for city:', city)
        fetchEnsembleForecast(city)
      } else {
        error.value = 'No city specified. Please go back and select a city.'
      }
    }

    onMounted(() => {
      const city = cityName.value
      if (city) {
        fetchEnsembleForecast(city)
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
          ensembleForecast.value = null
          error.value = 'No city specified. Please go back and select a city.'
          return
        }

        if (nextCity !== prevCity) {
          console.log('[Weather] City query changed, fetching new ensemble forecast:', nextCity)
          fetchEnsembleForecast(nextCity)
        }
      }
    )

    return {
      loading,
      error,
      ensembleForecast,
      showNextWeekModal,
      selectedDay,
      selectedDayData,
      cityName,
      formatTimestamp,
      handleNextWeekClick,
      closeModal,
      goBack,
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
