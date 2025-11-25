<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-400 via-blue-500 to-blue-600">
    <!-- Hero Section -->
    <section class="relative overflow-hidden">
      <!-- Background Elements -->
      <div class="absolute inset-0">
        <div class="absolute top-20 left-10 w-72 h-72 bg-white/10 rounded-full blur-3xl"></div>
        <div class="absolute bottom-20 right-10 w-96 h-96 bg-white/10 rounded-full blur-3xl"></div>
        <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-80 h-80 bg-yellow-300/10 rounded-full blur-3xl"></div>
      </div>

      <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
        <div class="text-center">
          <!-- Weather Icon -->
          <div class="flex justify-center mb-8">
            <div class="relative">
              <div class="w-32 h-32 bg-yellow-400 rounded-full shadow-2xl flex items-center justify-center">
                <svg class="w-20 h-20 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                </svg>
              </div>
              <!-- Animated clouds -->
              <div class="absolute -top-4 -right-4 w-16 h-16 bg-white/80 rounded-full shadow-lg animate-pulse"></div>
              <div class="absolute -bottom-4 -left-4 w-12 h-12 bg-white/60 rounded-full shadow-lg animate-pulse" style="animation-delay: 1s;"></div>
            </div>
          </div>

          <!-- Main Title -->
          <h1 class="text-5xl md:text-6xl lg:text-7xl font-bold text-white mb-6">
            Weather Forecast
            <span class="block text-3xl md:text-4xl lg:text-5xl text-blue-100 mt-2">
              for Indonesian Cities
            </span>
          </h1>

          <!-- Subtitle -->
          <p class="text-xl md:text-2xl text-blue-100 mb-12 max-w-3xl mx-auto">
            Get accurate weather predictions for cities across Indonesia. 
            Plan your day with confidence using our reliable forecast data.
          </p>

          <!-- Search Section -->
          <div class="max-w-2xl mx-auto">
            <div class="bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl p-8">
              <div class="mb-6">
                <h2 class="text-2xl font-semibold text-gray-900 mb-2">
                  Select Your City
                </h2>
                <p class="text-gray-600">
                  Choose a city to view the 7-day weather forecast
                </p>
              </div>

              <!-- City Selector -->
              <div class="mb-6">
                <CitySelector
                  v-model="selectedCity"
                  :error="cityError"
                  placeholder="Search for a city in Indonesia..."
                  @city-selected="handleCitySelected"
                />
              </div>

              <!-- Action Button -->
              <button
                @click="handleGetWeather"
                :disabled="!selectedCity || weatherLoading"
                class="w-full bg-blue-600 text-white px-8 py-4 rounded-lg font-semibold text-lg
                       hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-500/30
                       disabled:bg-gray-400 disabled:cursor-not-allowed
                       transition-all duration-200 transform hover:scale-[1.02] active:scale-[0.98]"
              >
                <div class="flex items-center justify-center space-x-3">
                  <svg v-if="weatherLoading" class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  <span v-if="weatherLoading">Getting Forecast...</span>
                  <span v-else>Lihat Prediksi</span>
                </div>
              </button>

              <!-- Validation Error -->
              <div v-if="validationError" class="mt-4 p-4 bg-red-50 border border-red-200 rounded-lg">
                <div class="flex items-center space-x-2">
                  <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                  </svg>
                  <span class="text-red-700 font-medium">{{ validationError }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section class="py-20 bg-white/10 backdrop-blur-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-white mb-4">
            Why Choose Our Weather Service?
          </h2>
          <div class="w-24 h-1 bg-yellow-400 mx-auto"></div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          <!-- Feature 1 -->
          <div class="text-center">
            <div class="w-20 h-20 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"></path>
              </svg>
            </div>
            <h3 class="text-xl font-semibold text-white mb-2">Multi-Provider</h3>
            <p class="text-blue-100">Data from multiple weather sources for accuracy</p>
          </div>

          <!-- Feature 2 -->
          <div class="text-center">
            <div class="w-20 h-20 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
            <h3 class="text-xl font-semibold text-white mb-2">7-Day Forecast</h3>
            <p class="text-blue-100">Plan your week with extended weather predictions</p>
          </div>

          <!-- Feature 3 -->
          <div class="text-center">
            <div class="w-20 h-20 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"></path>
              </svg>
            </div>
            <h3 class="text-xl font-semibold text-white mb-2">50+ Cities</h3>
            <p class="text-blue-100">Comprehensive coverage across Indonesia</p>
          </div>

          <!-- Feature 4 -->
          <div class="text-center">
            <div class="w-20 h-20 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
              </svg>
            </div>
            <h3 class="text-xl font-semibold text-white mb-2">Real-time Data</h3>
            <p class="text-blue-100">Up-to-date weather information when you need it</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import CitySelector from '../components/CitySelector.vue'

export default {
  name: 'Home',
  components: {
    CitySelector
  },
  setup() {
    const router = useRouter()
    const selectedCity = ref(null)
    const cityError = ref('')
    const validationError = ref('')
    const weatherLoading = ref(false)

    const handleCitySelected = (city) => {
      // Clear errors when a city is selected
      cityError.value = ''
      validationError.value = ''
      console.log('City selected:', city)
    }

    const handleGetWeather = async () => {
      // Validation
      if (!selectedCity.value) {
        validationError.value = 'Please select a city before viewing the weather forecast.'
        return
      }

      // Clear any existing errors
      validationError.value = ''
      cityError.value = ''
      weatherLoading.value = true

      try {
        // Navigate to weather page with city parameter
        await router.push({
          name: 'Weather',
          query: { city: selectedCity.value.name }
        })
      } catch (error) {
        console.error('Navigation error:', error)
        validationError.value = 'Failed to load weather forecast. Please try again.'
      } finally {
        weatherLoading.value = false
      }
    }

    return {
      selectedCity,
      cityError,
      validationError,
      weatherLoading,
      handleCitySelected,
      handleGetWeather
    }
  }
}
</script>