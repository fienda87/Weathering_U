<template>
  <div 
    class="bg-white rounded-xl shadow-md p-6 hover:shadow-lg transition-all duration-200 border border-gray-100"
    :class="{ 'ring-2 ring-blue-500 bg-blue-50/50': isToday }"
  >
    <div class="flex flex-col items-center space-y-3">
      <!-- Date -->
      <div class="text-sm font-semibold text-gray-900">
        {{ formattedDate }}
      </div>
      
      <!-- Weather Icon -->
      <div class="text-5xl">
        {{ weatherIcon }}
      </div>
      
      <!-- Condition -->
      <div class="text-sm text-gray-600 capitalize text-center">
        {{ condition }}
      </div>
      
      <!-- Temperature -->
      <div class="flex flex-col items-center space-y-1 w-full">
        <div class="flex items-center justify-between w-full px-2">
          <span class="text-xs text-gray-500">High</span>
          <span class="text-lg font-bold text-red-600">{{ tempMax }}°</span>
        </div>
        <div class="flex items-center justify-between w-full px-2">
          <span class="text-xs text-gray-500">Low</span>
          <span class="text-lg font-bold text-blue-600">{{ tempMin }}°</span>
        </div>
      </div>
      
      <!-- Additional Info -->
      <div class="w-full pt-3 border-t border-gray-200 space-y-2">
        <div class="flex items-center justify-between text-xs">
          <span class="text-gray-500">Avg</span>
          <span class="font-medium text-gray-700">{{ tempAvg }}°C</span>
        </div>
        <div class="flex items-center justify-between text-xs">
          <span class="text-gray-500">Humidity</span>
          <span class="font-medium text-gray-700">{{ humidity }}%</span>
        </div>
        <div v-if="hasWindSpeed" class="flex items-center justify-between text-xs">
          <span class="text-gray-500">Wind</span>
          <span class="font-medium text-gray-700">{{ windSpeed }} km/h</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { computed } from 'vue'

export default {
  name: 'WeatherCard',
  props: {
    date: {
      type: String,
      required: true
    },
    tempMax: {
      type: Number,
      required: true
    },
    tempMin: {
      type: Number,
      required: true
    },
    tempAvg: {
      type: Number,
      required: true
    },
    condition: {
      type: String,
      required: true
    },
    humidity: {
      type: Number,
      required: true
    },
    windSpeed: {
      type: Number,
      default: null
    },
    icon: {
      type: String,
      default: 'sunny'
    },
    isToday: {
      type: Boolean,
      default: false
    }
  },
  setup(props) {
    const formattedDate = computed(() => {
      if (props.isToday) return 'Today'
      const date = new Date(props.date)
      return date.toLocaleDateString('en-US', { 
        weekday: 'short', 
        month: 'short', 
        day: 'numeric' 
      })
    })

    const weatherIcon = computed(() => {
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
      
      const lowerCondition = props.condition.toLowerCase()
      return icons[lowerCondition] || icons[props.icon] || '🌤️'
    })

    const hasWindSpeed = computed(() => {
      return props.windSpeed !== null && props.windSpeed !== undefined && !Number.isNaN(props.windSpeed)
    })

    return {
      formattedDate,
      weatherIcon,
      hasWindSpeed
    }
  }
}
</script>
