<template>
  <nav class="bg-blue-600 shadow-lg sticky top-0 z-50">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16">
        <div class="flex items-center">
          <div class="flex-shrink-0 flex items-center">
            <div class="bg-yellow-400 text-blue-900 px-3 py-1 rounded-lg font-bold text-lg">
              ☀️
            </div>
            <span class="ml-2 text-xl font-semibold text-white">WeatherID</span>
          </div>
        </div>
        
        <div class="flex items-center space-x-8">
          <router-link 
            to="/" 
            class="text-blue-100 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200"
            :class="{ 'text-white border-b-2 border-yellow-400': $route.name === 'Home' }"
          >
            Home
          </router-link>
          <router-link 
            to="/weather" 
            class="text-blue-100 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200"
            :class="{ 'text-white border-b-2 border-yellow-400': $route.name === 'Weather' }"
          >
            Weather
          </router-link>
        </div>
      </div>
    </div>
  </nav>
</template>

<script>
export default {
  name: 'Navbar'
}
</script>