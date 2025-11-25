// Simple test script to verify API integration
// This can be run in the browser console to test the components

const testCitiesAPI = async () => {
  try {
    const response = await fetch('http://localhost:8000/api/cities')
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`)
    }
    const data = await response.json()
    console.log('✅ Cities API test passed:', data)
    return data
  } catch (error) {
    console.error('❌ Cities API test failed:', error)
    return null
  }
}

const testWeatherAPI = async (city = 'Jakarta') => {
  try {
    const response = await fetch(`http://localhost:8000/api/weather?city=${encodeURIComponent(city)}`)
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`)
    }
    const data = await response.json()
    console.log('✅ Weather API test passed:', data)
    return data
  } catch (error) {
    console.error('❌ Weather API test failed:', error)
    return null
  }
}

// Export for use in browser console
window.testAPI = {
  testCitiesAPI,
  testWeatherAPI
}

console.log('API test functions loaded. Use window.testAPI.testCitiesAPI() and window.testAPI.testWeatherAPI() to test.')