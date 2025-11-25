<template>
  <div class="relative">
    <!-- Search Input -->
    <div class="relative">
      <input
        ref="searchInput"
        v-model="searchQuery"
        @input="handleSearchInput"
        @keydown.down="handleKeyDown"
        @keydown.up="handleKeyUp"
        @keydown.enter="handleEnter"
        @keydown.escape="handleEscape"
        @focus="showDropdown = true"
        type="text"
        :placeholder="placeholder"
        class="w-full px-4 py-3 pr-10 text-gray-900 bg-white border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
        :class="{ 'border-red-500 ring-red-500': error }"
        :disabled="disabled"
      />
      <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
        <svg v-if="loading" class="w-5 h-5 text-gray-400 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <svg v-else class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
        </svg>
      </div>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="mt-1 text-sm text-red-600">
      {{ error }}
    </div>

    <!-- Dropdown -->
    <div
      v-if="showDropdown && (filteredCities.length > 0 || searchQuery.length > 0)"
      class="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-lg shadow-lg max-h-60 overflow-y-auto"
    >
      <!-- Loading State -->
      <div v-if="loading" class="px-4 py-3 text-center text-gray-500">
        <div class="flex items-center justify-center space-x-2">
          <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>Loading cities...</span>
        </div>
      </div>

      <!-- No Results -->
      <div v-else-if="filteredCities.length === 0 && searchQuery.length > 0" class="px-4 py-3 text-center text-gray-500">
        No cities found for "{{ searchQuery }}"
      </div>

      <!-- City List -->
      <div v-else>
        <div
          v-for="(city, index) in filteredCities"
          :key="city.id"
          @click="selectCity(city)"
          @mouseenter="highlightedIndex = index"
          class="px-4 py-3 cursor-pointer transition-colors duration-150"
          :class="{
            'bg-blue-50 text-blue-700': highlightedIndex === index,
            'hover:bg-gray-50': highlightedIndex !== index,
            'border-t border-gray-200': index > 0
          }"
        >
          <div class="flex justify-between items-center">
            <div>
              <div class="font-medium text-gray-900">{{ city.name }}</div>
              <div class="text-sm text-gray-500">{{ city.province }}</div>
            </div>
            <div class="text-xs text-gray-400">
              {{ city.latitude.toFixed(2) }}, {{ city.longitude.toFixed(2) }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'

export default {
  name: 'CitySelector',
  props: {
    modelValue: {
      type: Object,
      default: null
    },
    placeholder: {
      type: String,
      default: 'Search for a city...'
    },
    disabled: {
      type: Boolean,
      default: false
    },
    error: {
      type: String,
      default: ''
    }
  },
  emits: ['update:modelValue', 'city-selected'],
  setup(props, { emit }) {
    const searchQuery = ref('')
    const cities = ref([])
    const loading = ref(false)
    const showDropdown = ref(false)
    const highlightedIndex = ref(-1)
    const searchInput = ref(null)
    
    // Debounce timer
    let debounceTimer = null

    // Fetch cities from API
    const fetchCities = async () => {
      if (cities.value.length > 0) return // Already loaded
      
      loading.value = true
      try {
        const response = await fetch(`${import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000'}/api/cities`)
        if (!response.ok) {
          throw new Error(`Failed to fetch cities: ${response.status}`)
        }
        const data = await response.json()
        cities.value = data.cities
      } catch (error) {
        console.error('Error fetching cities:', error)
        // For development, you might want to use mock data
        cities.value = []
      } finally {
        loading.value = false
      }
    }

    // Filter cities based on search query
    const filteredCities = computed(() => {
      if (!searchQuery.value.trim()) {
        return cities.value.slice(0, 10) // Show first 10 cities when no search
      }
      
      const query = searchQuery.value.toLowerCase()
      return cities.value.filter(city => 
        city.name.toLowerCase().includes(query) ||
        city.province.toLowerCase().includes(query)
      ).slice(0, 20) // Limit to 20 results
    })

    // Handle search input with debouncing
    const handleSearchInput = () => {
      clearTimeout(debounceTimer)
      highlightedIndex.value = -1
      
      // Debounce search to avoid excessive filtering
      debounceTimer = setTimeout(() => {
        if (searchQuery.value && cities.value.length === 0) {
          fetchCities()
        }
      }, 300)
    }

    // Select a city
    const selectCity = (city) => {
      searchQuery.value = `${city.name}, ${city.province}`
      showDropdown.value = false
      highlightedIndex.value = -1
      emit('update:modelValue', city)
      emit('city-selected', city)
    }

    // Keyboard navigation
    const handleKeyDown = () => {
      if (!showDropdown.value || filteredCities.value.length === 0) return
      
      if (highlightedIndex.value < filteredCities.value.length - 1) {
        highlightedIndex.value++
      }
    }

    const handleKeyUp = () => {
      if (!showDropdown.value || filteredCities.value.length === 0) return
      
      if (highlightedIndex.value > 0) {
        highlightedIndex.value--
      }
    }

    const handleEnter = () => {
      if (showDropdown.value && highlightedIndex.value >= 0 && filteredCities.value[highlightedIndex.value]) {
        selectCity(filteredCities.value[highlightedIndex.value])
      }
    }

    const handleEscape = () => {
      showDropdown.value = false
      highlightedIndex.value = -1
      searchInput.value?.blur()
    }

    // Close dropdown when clicking outside
    const handleClickOutside = (event) => {
      if (!event.target.closest('.relative')) {
        showDropdown.value = false
        highlightedIndex.value = -1
      }
    }

    // Watch for external model value changes
    watch(() => props.modelValue, (newValue) => {
      if (newValue) {
        searchQuery.value = `${newValue.name}, ${newValue.province}`
      } else {
        searchQuery.value = ''
      }
    }, { immediate: true })

    // Initialize
    onMounted(() => {
      fetchCities()
      document.addEventListener('click', handleClickOutside)
    })

    onUnmounted(() => {
      document.removeEventListener('click', handleClickOutside)
      if (debounceTimer) {
        clearTimeout(debounceTimer)
      }
    })

    return {
      searchQuery,
      cities,
      loading,
      showDropdown,
      highlightedIndex,
      filteredCities,
      searchInput,
      handleSearchInput,
      selectCity,
      handleKeyDown,
      handleKeyUp,
      handleEnter,
      handleEscape
    }
  }
}
</script>