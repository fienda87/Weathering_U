<template>
  <div class="relative">
    <div class="relative">
      <input
        v-model="searchQuery"
        @input="handleInput"
        @keydown.enter="handleSearch"
        type="text"
        :placeholder="placeholder"
        class="w-full px-4 py-3 pl-12 text-gray-900 bg-white border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
        :class="{ 'border-red-500 ring-red-500': error }"
        :disabled="disabled"
      />
      <div class="absolute inset-y-0 left-0 flex items-center pl-3">
        <svg v-if="loading" class="w-5 h-5 text-gray-400 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <svg v-else class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
        </svg>
      </div>
      <div v-if="searchQuery" class="absolute inset-y-0 right-0 flex items-center pr-3">
        <button
          @click="clearSearch"
          class="text-gray-400 hover:text-gray-600 transition-colors duration-200"
          type="button"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="mt-1 text-sm text-red-600">
      {{ error }}
    </div>
  </div>
</template>

<script>
import { ref, watch } from 'vue'

export default {
  name: 'SearchBar',
  props: {
    modelValue: {
      type: String,
      default: ''
    },
    placeholder: {
      type: String,
      default: 'Search...'
    },
    disabled: {
      type: Boolean,
      default: false
    },
    loading: {
      type: Boolean,
      default: false
    },
    error: {
      type: String,
      default: ''
    },
    debounceDelay: {
      type: Number,
      default: 300
    }
  },
  emits: ['update:modelValue', 'search', 'input'],
  setup(props, { emit }) {
    const searchQuery = ref(props.modelValue)
    let debounceTimer = null

    const handleInput = () => {
      emit('update:modelValue', searchQuery.value)
      
      // Debounce the input event
      clearTimeout(debounceTimer)
      debounceTimer = setTimeout(() => {
        emit('input', searchQuery.value)
      }, props.debounceDelay)
    }

    const handleSearch = () => {
      emit('search', searchQuery.value)
    }

    const clearSearch = () => {
      searchQuery.value = ''
      emit('update:modelValue', '')
      emit('search', '')
    }

    // Watch for external model value changes
    watch(() => props.modelValue, (newValue) => {
      searchQuery.value = newValue
    })

    return {
      searchQuery,
      handleInput,
      handleSearch,
      clearSearch
    }
  }
}
</script>