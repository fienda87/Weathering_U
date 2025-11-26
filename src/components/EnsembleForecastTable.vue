<template>
  <div class="ensemble-forecast-table">
    <div class="overflow-x-auto">
      <table class="w-full bg-white shadow-lg rounded-lg overflow-hidden">
        <thead class="bg-gradient-to-r from-blue-500 to-blue-600 text-white">
          <tr>
            <th class="px-4 py-3 text-left text-sm font-semibold">Date</th>
            <th class="px-4 py-3 text-left text-sm font-semibold">Temp Max</th>
            <th class="px-4 py-3 text-left text-sm font-semibold">Temp Min</th>
            <th class="px-4 py-3 text-left text-sm font-semibold">Condition</th>
            <th class="px-4 py-3 text-left text-sm font-semibold">Confidence</th>
            <th class="px-4 py-3 text-center text-sm font-semibold">Action</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="(day, index) in forecastData.days" :key="index">
            <!-- Main row -->
            <tr 
              :class="[
                'border-b border-gray-200 transition-all duration-200',
                selectedDayIndex === index ? 'bg-blue-50 border-l-4 border-l-blue-500' : 'hover:bg-gray-50',
                'cursor-pointer'
              ]"
              @click="toggleExpand(index)"
            >
              <td class="px-4 py-3 text-sm">
                <div class="font-medium text-gray-900">{{ formatDate(day.date) }}</div>
                <div class="text-xs text-gray-500">{{ day.date }}</div>
              </td>
              <td class="px-4 py-3 text-sm">
                <span class="font-bold text-red-600">{{ day.final_forecast.temp_max.toFixed(1) }}°C</span>
              </td>
              <td class="px-4 py-3 text-sm">
                <span class="font-bold text-blue-600">{{ day.final_forecast.temp_min.toFixed(1) }}°C</span>
              </td>
              <td class="px-4 py-3 text-sm text-gray-700">
                {{ day.final_forecast.condition }}
              </td>
              <td class="px-4 py-3 text-sm">
                <span 
                  :class="[
                    'inline-block px-3 py-1 rounded-full text-xs font-semibold',
                    getConfidenceBadgeClass(day.final_forecast.confidence)
                  ]"
                >
                  {{ day.final_forecast.confidence.toUpperCase() }}
                </span>
              </td>
              <td class="px-4 py-3 text-center">
                <button
                  @click.stop="handleNextWeekClick(index, day.date)"
                  class="px-4 py-2 bg-blue-500 text-white text-sm font-medium rounded-md hover:bg-blue-600 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                >
                  Next Week
                </button>
              </td>
            </tr>
            
            <!-- Expandable per-source row -->
            <tr v-if="expandedDays.has(index)" class="bg-gray-50">
              <td colspan="6" class="px-4 py-4">
                <div class="space-y-3">
                  <h4 class="text-sm font-semibold text-gray-700 mb-3">Per-Source Data:</h4>
                  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <!-- Open-Meteo -->
                    <div v-if="day.per_source.open_meteo" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                      <div class="flex items-center mb-2">
                        <div class="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
                        <h5 class="text-xs font-bold text-gray-700">Open-Meteo</h5>
                      </div>
                      <div class="space-y-1 text-xs">
                        <div class="flex justify-between">
                          <span class="text-gray-600">Max:</span>
                          <span class="font-semibold text-red-600">{{ day.per_source.open_meteo.temp_max.toFixed(1) }}°C</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-600">Min:</span>
                          <span class="font-semibold text-blue-600">{{ day.per_source.open_meteo.temp_min.toFixed(1) }}°C</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-600">Condition:</span>
                          <span class="font-medium text-gray-700">{{ day.per_source.open_meteo.condition }}</span>
                        </div>
                      </div>
                    </div>
                    
                    <!-- OpenWeatherMap -->
                    <div v-if="day.per_source.open_weather" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                      <div class="flex items-center mb-2">
                        <div class="w-2 h-2 bg-orange-500 rounded-full mr-2"></div>
                        <h5 class="text-xs font-bold text-gray-700">OpenWeatherMap</h5>
                      </div>
                      <div class="space-y-1 text-xs">
                        <div class="flex justify-between">
                          <span class="text-gray-600">Max:</span>
                          <span class="font-semibold text-red-600">{{ day.per_source.open_weather.temp_max.toFixed(1) }}°C</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-600">Min:</span>
                          <span class="font-semibold text-blue-600">{{ day.per_source.open_weather.temp_min.toFixed(1) }}°C</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-600">Condition:</span>
                          <span class="font-medium text-gray-700">{{ day.per_source.open_weather.condition }}</span>
                        </div>
                      </div>
                    </div>
                    
                    <!-- WeatherAPI -->
                    <div v-if="day.per_source.weather_api" class="bg-white p-3 rounded-lg shadow-sm border border-gray-200">
                      <div class="flex items-center mb-2">
                        <div class="w-2 h-2 bg-purple-500 rounded-full mr-2"></div>
                        <h5 class="text-xs font-bold text-gray-700">WeatherAPI</h5>
                      </div>
                      <div class="space-y-1 text-xs">
                        <div class="flex justify-between">
                          <span class="text-gray-600">Max:</span>
                          <span class="font-semibold text-red-600">{{ day.per_source.weather_api.temp_max.toFixed(1) }}°C</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-600">Min:</span>
                          <span class="font-semibold text-blue-600">{{ day.per_source.weather_api.temp_min.toFixed(1) }}°C</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-600">Condition:</span>
                          <span class="font-medium text-gray-700">{{ day.per_source.weather_api.condition }}</span>
                        </div>
                      </div>
                    </div>
                    
                    <!-- No data message -->
                    <div v-if="!day.per_source.open_meteo && !day.per_source.open_weather && !day.per_source.weather_api" class="col-span-3 text-center text-gray-500 text-sm py-4">
                      No per-source data available
                    </div>
                  </div>
                </div>
              </td>
            </tr>
          </template>
        </tbody>
      </table>
      
      <!-- Empty state -->
      <div v-if="!forecastData.days || forecastData.days.length === 0" class="text-center py-12 text-gray-500">
        <p class="text-lg">No forecast data available</p>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'

export default {
  name: 'EnsembleForecastTable',
  props: {
    forecastData: {
      type: Object,
      required: true
    }
  },
  emits: ['next-week-click'],
  setup(props, { emit }) {
    const selectedDayIndex = ref(null)
    const expandedDays = ref(new Set())

    const formatDate = (dateString) => {
      try {
        const date = new Date(dateString)
        const today = new Date()
        today.setHours(0, 0, 0, 0)
        const targetDate = new Date(date)
        targetDate.setHours(0, 0, 0, 0)
        
        if (targetDate.getTime() === today.getTime()) {
          return 'Today'
        }
        
        const options = { weekday: 'short', month: 'short', day: 'numeric' }
        return date.toLocaleDateString('en-US', options)
      } catch (error) {
        console.error('[EnsembleForecastTable] Error formatting date:', error)
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

    const handleNextWeekClick = (dayIndex, date) => {
      console.log(`[EnsembleForecastTable] Next Week clicked for day ${dayIndex}, date: ${date}`)
      selectedDayIndex.value = dayIndex
      
      const dateObj = new Date(date)
      const dayOfWeek = (dateObj.getDay() + 6) % 7
      
      emit('next-week-click', {
        dayIndex,
        dayOfWeek,
        date
      })
    }

    const toggleExpand = (index) => {
      if (expandedDays.value.has(index)) {
        expandedDays.value.delete(index)
      } else {
        expandedDays.value.add(index)
      }
      expandedDays.value = new Set(expandedDays.value)
    }

    return {
      selectedDayIndex,
      expandedDays,
      formatDate,
      getConfidenceBadgeClass,
      handleNextWeekClick,
      toggleExpand
    }
  }
}
</script>

<style scoped>
.ensemble-forecast-table table {
  border-collapse: collapse;
}

.ensemble-forecast-table thead {
  position: sticky;
  top: 0;
  z-index: 10;
}

.ensemble-forecast-table tbody tr {
  transition: background-color 0.2s ease;
}
</style>
