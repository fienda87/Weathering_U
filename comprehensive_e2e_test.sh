#!/bin/bash

# Comprehensive E2E Test Script for DESWEBPRAKTIKUM10
# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
START_TIME=$(date +%s)

# API base URL
API_BASE="http://localhost:8000"

# Test result function
test_result() {
    local name="$1"
    local status="$2"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓${NC} $name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}✗${NC} $name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

echo "========================================="
echo "  COMPREHENSIVE E2E TESTING"
echo "  DESWEBPRAKTIKUM10"
echo "========================================="
echo ""

# A. INFRASTRUCTURE SETUP
echo -e "${BLUE}[A] INFRASTRUCTURE & SETUP${NC}"
echo "-----------------------------------"

# Check Node.js
NODE_VERSION=$(node -v 2>/dev/null | sed 's/v//')
if [[ ! -z "$NODE_VERSION" ]] && [[ $(echo "$NODE_VERSION" | cut -d. -f1) -ge 16 ]]; then
    test_result "Node.js v16+ installed (v$NODE_VERSION)" "PASS"
else
    test_result "Node.js v16+ installed" "FAIL"
fi

# Check Rust
RUST_VERSION=$(rustc -V 2>/dev/null | awk '{print $2}')
if [[ ! -z "$RUST_VERSION" ]]; then
    test_result "Rust installed ($RUST_VERSION)" "PASS"
else
    test_result "Rust installed" "FAIL"
fi

# Check .env files
if [ -f ".env" ] && [ -f "backend/.env" ]; then
    test_result ".env files exist" "PASS"
else
    test_result ".env files exist" "FAIL"
fi

echo ""

# B. BACKEND TESTING
echo -e "${BLUE}[B] BACKEND TESTING${NC}"
echo "-----------------------------------"

# Wait for backend to be ready
echo "Waiting for backend to be ready..."
MAX_RETRIES=30
RETRY_COUNT=0
BACKEND_READY=false

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s "$API_BASE/api/cities" > /dev/null 2>&1; then
        BACKEND_READY=true
        break
    fi
    sleep 2
    RETRY_COUNT=$((RETRY_COUNT + 1))
done

if [ "$BACKEND_READY" = true ]; then
    test_result "Backend running on port 8000" "PASS"
else
    test_result "Backend running on port 8000" "FAIL"
    echo "Backend not responding. Exiting..."
    exit 1
fi

# B.1 Cities Endpoint
echo ""
echo "Testing Cities Endpoint..."
CITIES_RESPONSE=$(curl -s "$API_BASE/api/cities")
if echo "$CITIES_RESPONSE" | jq -e '.cities' > /dev/null 2>&1; then
    test_result "GET /api/cities returns valid JSON" "PASS"
    
    # Check for specific cities
    if echo "$CITIES_RESPONSE" | jq -e '.cities[] | select(.name=="Jakarta")' > /dev/null 2>&1; then
        test_result "Jakarta city exists" "PASS"
    else
        test_result "Jakarta city exists" "FAIL"
    fi
    
    if echo "$CITIES_RESPONSE" | jq -e '.cities[] | select(.name=="Bandung")' > /dev/null 2>&1; then
        test_result "Bandung city exists" "PASS"
    else
        test_result "Bandung city exists" "FAIL"
    fi
else
    test_result "GET /api/cities returns valid JSON" "FAIL"
fi

# B.2 Weather Current Week Endpoint
echo ""
echo "Testing Weather Current Week Endpoint..."
WEATHER_RESPONSE=$(curl -s "$API_BASE/api/weather/ensemble?city=Jakarta")
if echo "$WEATHER_RESPONSE" | jq -e '.days' > /dev/null 2>&1; then
    test_result "GET /api/weather/ensemble?city=Jakarta returns data" "PASS"
    
    # Check if 7 days returned
    DAY_COUNT=$(echo "$WEATHER_RESPONSE" | jq '.days | length')
    if [ "$DAY_COUNT" -eq 7 ]; then
        test_result "Returns 7 days of forecast" "PASS"
    else
        test_result "Returns 7 days of forecast (got $DAY_COUNT)" "FAIL"
    fi
    
    # Check first day structure
    if echo "$WEATHER_RESPONSE" | jq -e '.days[0].date' > /dev/null 2>&1 && \
       echo "$WEATHER_RESPONSE" | jq -e '.days[0].final_forecast.temp_max' > /dev/null 2>&1; then
        test_result "Day structure includes required fields" "PASS"
    else
        test_result "Day structure includes required fields" "FAIL"
    fi
    
    # Check confidence levels
    if echo "$WEATHER_RESPONSE" | jq -e '.days[0].final_forecast.confidence' > /dev/null 2>&1; then
        CONFIDENCE=$(echo "$WEATHER_RESPONSE" | jq -r '.days[0].final_forecast.confidence')
        if [[ "$CONFIDENCE" =~ ^(high|medium|low)$ ]]; then
            test_result "Valid confidence level ($CONFIDENCE)" "PASS"
        else
            test_result "Valid confidence level" "FAIL"
        fi
    else
        test_result "Confidence field exists" "FAIL"
    fi
    
    # Check per-source data
    if echo "$WEATHER_RESPONSE" | jq -e '.days[0].per_source' > /dev/null 2>&1; then
        test_result "Per-source data included" "PASS"
    else
        test_result "Per-source data included" "FAIL"
    fi
else
    test_result "GET /api/weather/ensemble?city=Jakarta returns data" "FAIL"
fi

# B.3 Weather Next Week Endpoint
echo ""
echo "Testing Weather Next Week Endpoint..."
NEXTWEEK_RESPONSE=$(curl -s "$API_BASE/api/weather/ensemble?city=Jakarta&period=next_week&day=0")
if echo "$NEXTWEEK_RESPONSE" | jq -e '.days' > /dev/null 2>&1; then
    test_result "GET /api/weather/ensemble next_week returns data" "PASS"
else
    test_result "GET /api/weather/ensemble next_week returns data" "FAIL"
fi

# B.4 Error Handling
echo ""
echo "Testing Error Handling..."

# Invalid city
INVALID_CITY_RESPONSE=$(curl -s -w "%{http_code}" "$API_BASE/api/weather/ensemble?city=InvalidCity99999" -o /tmp/invalid_city.json)
if [ "$INVALID_CITY_RESPONSE" = "404" ]; then
    test_result "Invalid city returns 404" "PASS"
else
    test_result "Invalid city returns 404 (got $INVALID_CITY_RESPONSE)" "FAIL"
fi

# Empty city
EMPTY_CITY_RESPONSE=$(curl -s -w "%{http_code}" "$API_BASE/api/weather/ensemble?city=" -o /tmp/empty_city.json)
if [ "$EMPTY_CITY_RESPONSE" = "400" ]; then
    test_result "Empty city returns 400" "PASS"
else
    test_result "Empty city returns 400 (got $EMPTY_CITY_RESPONSE)" "FAIL"
fi

# Missing city
MISSING_CITY_RESPONSE=$(curl -s -w "%{http_code}" "$API_BASE/api/weather/ensemble" -o /tmp/missing_city.json)
if [ "$MISSING_CITY_RESPONSE" = "400" ]; then
    test_result "Missing city param returns 400" "PASS"
else
    test_result "Missing city param returns 400 (got $MISSING_CITY_RESPONSE)" "FAIL"
fi

# Invalid day (>6)
INVALID_DAY_RESPONSE=$(curl -s -w "%{http_code}" "$API_BASE/api/weather/ensemble?city=Jakarta&period=next_week&day=7" -o /tmp/invalid_day.json)
if [ "$INVALID_DAY_RESPONSE" = "400" ]; then
    test_result "Invalid day (>6) returns 400" "PASS"
else
    test_result "Invalid day (>6) returns 400 (got $INVALID_DAY_RESPONSE)" "FAIL"
fi

# C. PERFORMANCE TESTING
echo ""
echo -e "${BLUE}[C] PERFORMANCE TESTING${NC}"
echo "-----------------------------------"

# Test response time (cached)
START=$(date +%s%N)
curl -s "$API_BASE/api/weather/ensemble?city=Jakarta" > /dev/null
END=$(date +%s%N)
RESPONSE_TIME=$(( (END - START) / 1000000 ))

if [ $RESPONSE_TIME -lt 500 ]; then
    test_result "Cached request < 500ms (${RESPONSE_TIME}ms)" "PASS"
else
    test_result "Cached request < 500ms (${RESPONSE_TIME}ms)" "FAIL"
fi

# Test fresh request (different city)
START=$(date +%s%N)
curl -s "$API_BASE/api/weather/ensemble?city=Bandung" > /dev/null
END=$(date +%s%N)
RESPONSE_TIME=$(( (END - START) / 1000000 ))

if [ $RESPONSE_TIME -lt 5000 ]; then
    test_result "Fresh request < 5s (${RESPONSE_TIME}ms)" "PASS"
else
    test_result "Fresh request < 5s (${RESPONSE_TIME}ms)" "FAIL"
fi

# D. MULTIPLE CITIES TESTING
echo ""
echo -e "${BLUE}[D] MULTIPLE CITIES TESTING${NC}"
echo "-----------------------------------"

for CITY in "Jakarta" "Bandung" "Surabaya" "Medan"; do
    CITY_RESPONSE=$(curl -s "$API_BASE/api/weather/ensemble?city=$CITY")
    if echo "$CITY_RESPONSE" | jq -e '.city' > /dev/null 2>&1; then
        test_result "$CITY forecast loads correctly" "PASS"
    else
        test_result "$CITY forecast loads correctly" "FAIL"
    fi
done

# E. CORS HEADERS
echo ""
echo -e "${BLUE}[E] CORS HEADERS${NC}"
echo "-----------------------------------"

CORS_RESPONSE=$(curl -s -I "$API_BASE/api/cities")
if echo "$CORS_RESPONSE" | grep -i "access-control-allow-origin" > /dev/null; then
    test_result "CORS headers present" "PASS"
else
    test_result "CORS headers present" "FAIL"
fi

# F. DATA VALIDATION
echo ""
echo -e "${BLUE}[F] DATA VALIDATION${NC}"
echo "-----------------------------------"

VALIDATION_RESPONSE=$(curl -s "$API_BASE/api/weather/ensemble?city=Jakarta")

# Check date format (YYYY-MM-DD)
FIRST_DATE=$(echo "$VALIDATION_RESPONSE" | jq -r '.days[0].date')
if [[ $FIRST_DATE =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
    test_result "Date format is YYYY-MM-DD" "PASS"
else
    test_result "Date format is YYYY-MM-DD" "FAIL"
fi

# Check temperatures are numbers
TEMP_MAX=$(echo "$VALIDATION_RESPONSE" | jq '.days[0].final_forecast.temp_max')
if [[ $TEMP_MAX =~ ^[0-9]+\.?[0-9]*$ ]]; then
    test_result "Temperatures are valid numbers" "PASS"
else
    test_result "Temperatures are valid numbers" "FAIL"
fi

# Check no null values in required fields
NULL_CHECK=$(echo "$VALIDATION_RESPONSE" | jq '[.days[].date, .days[].final_forecast.temp_max, .days[].final_forecast.condition] | map(select(. == null)) | length')
if [ "$NULL_CHECK" -eq 0 ]; then
    test_result "No null values in required fields" "PASS"
else
    test_result "No null values in required fields" "FAIL"
fi

# Summary
echo ""
echo "========================================="
echo -e "${BLUE}TEST RESULTS SUMMARY${NC}"
echo "========================================="
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "Start time: $(date -d @$START_TIME '+%Y-%m-%d %H:%M:%S')"
echo "End time: $(date -d @$END_TIME '+%Y-%m-%d %H:%M:%S')"
echo "Duration: ${DURATION}s"
echo ""
echo "Total tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
echo -e "${RED}Failed: $FAILED_TESTS${NC}"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}✓ ALL TESTS PASSED - READY FOR PRODUCTION${NC}"
    exit 0
else
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${YELLOW}⚠ $FAILED_TESTS TEST(S) FAILED - Pass rate: ${PASS_RATE}%${NC}"
    exit 1
fi
