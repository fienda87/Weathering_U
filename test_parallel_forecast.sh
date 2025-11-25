#!/bin/bash

# Test script for parallel forecast processing
# This script tests the new parallel processing endpoints

echo "=== Testing Parallel Forecast Processing ==="
echo ""

BASE_URL="http://localhost:8000"
CITY="Jakarta"

echo "Testing rate-limited parallel endpoint..."
echo "URL: $BASE_URL/api/weather?city=$CITY"
time curl -s "$BASE_URL/api/weather?city=$CITY" | jq '.forecast | length' 2>/dev/null || echo "Request failed or JSON parsing failed"
echo ""

echo "Testing unlimited parallel endpoint..."
echo "URL: $BASE_URL/api/weather/parallel?city=$CITY"
time curl -s "$BASE_URL/api/weather/parallel?city=$CITY" | jq '.forecast | length' 2>/dev/null || echo "Request failed or JSON parsing failed"
echo ""

echo "Testing with invalid city..."
echo "URL: $BASE_URL/api/weather?city=InvalidCity"
curl -s "$BASE_URL/api/weather?city=InvalidCity" | jq '.error' 2>/dev/null || echo "Request failed or JSON parsing failed"
echo ""

echo "Testing missing city parameter..."
echo "URL: $BASE_URL/api/weather"
curl -s "$BASE_URL/api/weather" | jq '.error' 2>/dev/null || echo "Request failed or JSON parsing failed"
echo ""

echo "=== Performance Comparison ==="
echo "Running 5 requests for rate-limited endpoint..."
for i in {1..5}; do
    echo "Request $i:"
    time curl -s "$BASE_URL/api/weather?city=$CITY" > /dev/null
done

echo ""
echo "Running 5 requests for unlimited parallel endpoint..."
for i in {1..5}; do
    echo "Request $i:"
    time curl -s "$BASE_URL/api/weather/parallel?city=$CITY" > /dev/null
done

echo ""
echo "=== Test Complete ==="
echo "Check server logs for detailed parallel processing metrics"