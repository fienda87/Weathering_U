<template>
  <footer class="bg-blue-800 text-white">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div>
          <div class="flex items-center mb-4">
            <div class="bg-yellow-400 text-blue-900 px-3 py-1 rounded-lg font-bold text-lg">
              ☀️
            </div>
            <span class="ml-2 text-xl font-semibold">WeatherID</span>
          </div>
          <p class="text-blue-100 text-sm">
            WeatherID - Indonesian Weather Forecast Service
          </p>
          <p class="text-blue-200 text-xs mt-2">
            Accurate weather predictions for cities across Indonesia.
          </p>
        </div>
        
        <div>
          <h3 class="text-lg font-semibold mb-4">Weather Services</h3>
          <ul class="space-y-2 text-blue-100 text-sm">
            <li>Current Weather Conditions</li>
            <li>7-Day Weather Forecast</li>
            <li>Multi-Provider Data</li>
            <li>Real-time Updates</li>
            <li>50+ Indonesian Cities</li>
          </ul>
        </div>
        
        <div>
          <h3 class="text-lg font-semibold mb-4">Technology</h3>
          <div class="space-y-2 text-blue-100 text-sm">
            <p class="flex items-center">
              <span class="mr-2">☁️</span>
              Multi-Source Weather APIs
            </p>
            <p class="flex items-center">
              <span class="mr-2">⚡</span>
              Real-time Data Processing
            </p>
            <p class="flex items-center">
              <span class="mr-2">🛡️</span>
              Fallback Reliability
            </p>
          </div>
        </div>
      </div>
      
      <div class="border-t border-blue-700 mt-8 pt-8 text-center text-blue-200 text-sm">
        <p>&copy; 2024 WeatherID. All rights reserved.</p>
      </div>
    </div>
  </footer>
</template>

<script>
export default {
  name: 'Footer'
}
</script>