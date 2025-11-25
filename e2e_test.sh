#!/bin/bash

# Comprehensive End-to-End Test Script
# Tests all requirements from the Final QA ticket

set -e

API_BASE="http://localhost:8000"
RESULTS_FILE="/tmp/e2e_test_results.txt"
ERRORS_FILE="/tmp/e2e_test_errors.txt"

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🚀 Starting Comprehensive E2E Testing" > $RESULTS_FILE
echo "=======================================" >> $RESULTS_FILE
echo "" >> $RESULTS_FILE

# Clear errors file
> $ERRORS_FILE

# Helper functions
log_pass() {
    echo -e "${GREEN}✓${NC} $1"
    echo "✓ $1" >> $RESULTS_FILE
}

log_fail() {
    echo -e "${RED}✗${NC} $1"
    echo "✗ $1" >> $RESULTS_FILE
    echo "✗ $1" >> $ERRORS_FILE
}

log_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
    echo "ℹ $1" >> $RESULTS_FILE
}

# 1. Backend Service Verification
echo ""
log_info "=== 1. Backend Service Verification ==="
echo ""

# Test /health endpoint
log_info "Testing GET /health"
START_TIME=$(date +%s%3N)
HEALTH_RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/health" 2>&1)
END_TIME=$(date +%s%3N)
RESPONSE_TIME=$((END_TIME - START_TIME))
HTTP_CODE=$(echo "$HEALTH_RESPONSE" | tail -1)
BODY=$(echo "$HEALTH_RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ]; then
    log_pass "/health returns 200 OK"
    if [ $RESPONSE_TIME -lt 100 ]; then
        log_pass "/health response time: ${RESPONSE_TIME}ms (< 100ms)"
    else
        log_fail "/health response time: ${RESPONSE_TIME}ms (>= 100ms)"
    fi
else
    log_fail "/health returns $HTTP_CODE (expected 200)"
fi

# Test /api/cities endpoint
log_info "Testing GET /api/cities"
START_TIME=$(date +%s%3N)
CITIES_RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/cities" 2>&1)
END_TIME=$(date +%s%3N)
RESPONSE_TIME=$((END_TIME - START_TIME))
HTTP_CODE=$(echo "$CITIES_RESPONSE" | tail -1)
BODY=$(echo "$CITIES_RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ]; then
    log_pass "/api/cities returns 200 OK"
    CITY_COUNT=$(echo "$BODY" | jq '.cities | length' 2>/dev/null || echo "0")
    if [ "$CITY_COUNT" = "50" ]; then
        log_pass "/api/cities returns exactly 50 cities"
    else
        log_fail "/api/cities returns $CITY_COUNT cities (expected 50)"
    fi
    if [ $RESPONSE_TIME -lt 500 ]; then
        log_pass "/api/cities response time: ${RESPONSE_TIME}ms (< 500ms)"
    else
        log_fail "/api/cities response time: ${RESPONSE_TIME}ms (>= 500ms)"
    fi
else
    log_fail "/api/cities returns $HTTP_CODE (expected 200)"
fi

# Test /api/weather endpoint
log_info "Testing GET /api/weather?city=Jakarta"
START_TIME=$(date +%s%3N)
WEATHER_RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/weather?city=Jakarta" 2>&1)
END_TIME=$(date +%s%3N)
RESPONSE_TIME=$((END_TIME - START_TIME))
HTTP_CODE=$(echo "$WEATHER_RESPONSE" | tail -1)
BODY=$(echo "$WEATHER_RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ]; then
    log_pass "/api/weather returns 200 OK for Jakarta"
    FORECAST_COUNT=$(echo "$BODY" | jq '.forecast | length' 2>/dev/null || echo "0")
    if [ "$FORECAST_COUNT" = "7" ]; then
        log_pass "/api/weather returns 7-day forecast"
    else
        log_fail "/api/weather returns $FORECAST_COUNT days (expected 7)"
    fi
    if [ $RESPONSE_TIME -lt 5000 ]; then
        log_pass "/api/weather response time: ${RESPONSE_TIME}ms (< 5000ms)"
    else
        log_fail "/api/weather response time: ${RESPONSE_TIME}ms (>= 5000ms)"
    fi
else
    log_fail "/api/weather returns $HTTP_CODE (expected 200)"
fi

# Test CORS preflight
log_info "Testing OPTIONS /api/weather (CORS preflight)"
CORS_RESPONSE=$(curl -s -w "\n%{http_code}" -X OPTIONS \
    -H "Origin: http://localhost:5173" \
    -H "Access-Control-Request-Method: GET" \
    "$API_BASE/api/weather" 2>&1)
HTTP_CODE=$(echo "$CORS_RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "204" ]; then
    log_pass "CORS preflight returns $HTTP_CODE"
else
    log_fail "CORS preflight returns $HTTP_CODE (expected 200 or 204)"
fi

# Check CORS headers
CORS_HEADERS=$(curl -s -I "$API_BASE/api/cities" 2>&1 | grep -i "access-control")
if [ -n "$CORS_HEADERS" ]; then
    log_pass "CORS headers present on responses"
else
    log_fail "CORS headers missing from responses"
fi

# 2. Test All 50 Cities
echo ""
log_info "=== 2. Testing All 50 Cities ==="
echo ""

# Get all cities
CITIES=$(curl -s "$API_BASE/api/cities" 2>&1)
CITY_NAMES=$(echo "$CITIES" | jq -r '.cities[].name' 2>/dev/null)

TESTED_CITIES=0
SUCCESSFUL_CITIES=0
FAILED_CITIES=0

while IFS= read -r CITY; do
    if [ -n "$CITY" ]; then
        TESTED_CITIES=$((TESTED_CITIES + 1))
        WEATHER_RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/weather?city=$CITY" 2>&1)
        HTTP_CODE=$(echo "$WEATHER_RESPONSE" | tail -1)
        
        if [ "$HTTP_CODE" = "200" ]; then
            BODY=$(echo "$WEATHER_RESPONSE" | head -n -1)
            FORECAST_COUNT=$(echo "$BODY" | jq '.forecast | length' 2>/dev/null || echo "0")
            if [ "$FORECAST_COUNT" = "7" ]; then
                SUCCESSFUL_CITIES=$((SUCCESSFUL_CITIES + 1))
            else
                log_fail "$CITY: Returns $FORECAST_COUNT days (expected 7)"
                FAILED_CITIES=$((FAILED_CITIES + 1))
            fi
        else
            log_fail "$CITY: Returns HTTP $HTTP_CODE (expected 200)"
            FAILED_CITIES=$((FAILED_CITIES + 1))
        fi
    fi
done <<< "$CITY_NAMES"

log_info "Tested $TESTED_CITIES cities: $SUCCESSFUL_CITIES successful, $FAILED_CITIES failed"

if [ $TESTED_CITIES -eq 50 ]; then
    log_pass "All 50 cities tested"
else
    log_fail "Only $TESTED_CITIES cities tested (expected 50)"
fi

if [ $SUCCESSFUL_CITIES -eq 50 ]; then
    log_pass "All 50 cities return valid 7-day forecast"
else
    log_fail "Only $SUCCESSFUL_CITIES cities return valid forecast (expected 50)"
fi

# 3. Error Handling Tests
echo ""
log_info "=== 3. Error Handling Tests ==="
echo ""

# Test missing city parameter
log_info "Testing /api/weather without city parameter"
RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/weather" 2>&1)
HTTP_CODE=$(echo "$RESPONSE" | tail -1)
if [ "$HTTP_CODE" = "400" ] || [ "$HTTP_CODE" = "422" ]; then
    log_pass "Missing city parameter returns $HTTP_CODE (Bad Request)"
else
    log_fail "Missing city parameter returns $HTTP_CODE (expected 400 or 422)"
fi

# Test non-existent city
log_info "Testing /api/weather with non-existent city"
RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/weather?city=NonExistentCity999" 2>&1)
HTTP_CODE=$(echo "$RESPONSE" | tail -1)
if [ "$HTTP_CODE" = "404" ]; then
    log_pass "Non-existent city returns 404 (Not Found)"
else
    log_fail "Non-existent city returns $HTTP_CODE (expected 404)"
fi

# Test empty city parameter
log_info "Testing /api/weather with empty city parameter"
RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/weather?city=" 2>&1)
HTTP_CODE=$(echo "$RESPONSE" | tail -1)
if [ "$HTTP_CODE" = "400" ] || [ "$HTTP_CODE" = "422" ] || [ "$HTTP_CODE" = "404" ]; then
    log_pass "Empty city parameter returns $HTTP_CODE (Bad Request)"
else
    log_fail "Empty city parameter returns $HTTP_CODE (expected 400, 422, or 404)"
fi

# Test city with special characters
log_info "Testing /api/weather with special characters (Yogyakarta)"
RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/api/weather?city=Yogyakarta" 2>&1)
HTTP_CODE=$(echo "$RESPONSE" | tail -1)
if [ "$HTTP_CODE" = "200" ]; then
    log_pass "City with special characters returns 200 OK"
else
    log_fail "City with special characters returns $HTTP_CODE (expected 200)"
fi

# 4. Data Validation
echo ""
log_info "=== 4. Data Validation ==="
echo ""

# Test Jakarta weather for data quality
WEATHER_DATA=$(curl -s "$API_BASE/api/weather?city=Jakarta" 2>&1)

# Check forecast structure
HAS_FORECAST=$(echo "$WEATHER_DATA" | jq 'has("forecast")' 2>/dev/null)
if [ "$HAS_FORECAST" = "true" ]; then
    log_pass "Response has 'forecast' field"
else
    log_fail "Response missing 'forecast' field"
fi

# Validate each day's data
VALIDATION_ERRORS=0
for i in {0..6}; do
    DATE=$(echo "$WEATHER_DATA" | jq -r ".forecast[$i].date" 2>/dev/null)
    TEMP_MAX=$(echo "$WEATHER_DATA" | jq -r ".forecast[$i].temp_max" 2>/dev/null)
    TEMP_MIN=$(echo "$WEATHER_DATA" | jq -r ".forecast[$i].temp_min" 2>/dev/null)
    HUMIDITY=$(echo "$WEATHER_DATA" | jq -r ".forecast[$i].humidity" 2>/dev/null)
    CONDITION=$(echo "$WEATHER_DATA" | jq -r ".forecast[$i].condition" 2>/dev/null)
    ICON=$(echo "$WEATHER_DATA" | jq -r ".forecast[$i].icon" 2>/dev/null)
    
    # Check date format
    if [[ ! "$DATE" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
        log_fail "Day $i: Invalid date format '$DATE'"
        VALIDATION_ERRORS=$((VALIDATION_ERRORS + 1))
    fi
    
    # Check temperature range
    if [ "$TEMP_MAX" != "null" ] && [ "$TEMP_MIN" != "null" ]; then
        if (( $(echo "$TEMP_MAX < $TEMP_MIN" | bc -l) )); then
            log_fail "Day $i: temp_max ($TEMP_MAX) < temp_min ($TEMP_MIN)"
            VALIDATION_ERRORS=$((VALIDATION_ERRORS + 1))
        fi
        
        # Check realistic temperature for Indonesia
        if (( $(echo "$TEMP_MAX > 40 || $TEMP_MAX < -10" | bc -l) )); then
            log_fail "Day $i: temp_max ($TEMP_MAX) out of realistic range"
            VALIDATION_ERRORS=$((VALIDATION_ERRORS + 1))
        fi
    fi
    
    # Check humidity range
    if [ "$HUMIDITY" != "null" ]; then
        if (( $(echo "$HUMIDITY < 0 || $HUMIDITY > 100" | bc -l) )); then
            log_fail "Day $i: humidity ($HUMIDITY) out of range 0-100"
            VALIDATION_ERRORS=$((VALIDATION_ERRORS + 1))
        fi
    fi
    
    # Check condition is not empty
    if [ -z "$CONDITION" ] || [ "$CONDITION" = "null" ]; then
        log_fail "Day $i: Missing or null condition"
        VALIDATION_ERRORS=$((VALIDATION_ERRORS + 1))
    fi
    
    # Check icon is valid
    if [ -z "$ICON" ] || [ "$ICON" = "null" ]; then
        log_fail "Day $i: Missing or null icon"
        VALIDATION_ERRORS=$((VALIDATION_ERRORS + 1))
    fi
done

if [ $VALIDATION_ERRORS -eq 0 ]; then
    log_pass "All data validation checks passed"
else
    log_fail "$VALIDATION_ERRORS data validation errors found"
fi

# Summary
echo ""
echo "=======================================" >> $RESULTS_FILE
echo "Test Summary" >> $RESULTS_FILE
echo "=======================================" >> $RESULTS_FILE
echo ""

TOTAL_TESTS=$(grep -c "✓\|✗" $RESULTS_FILE)
PASSED_TESTS=$(grep -c "✓" $RESULTS_FILE)
FAILED_TESTS=$(grep -c "✗" $RESULTS_FILE)

echo "Total Tests: $TOTAL_TESTS" >> $RESULTS_FILE
echo "Passed: $PASSED_TESTS" >> $RESULTS_FILE
echo "Failed: $FAILED_TESTS" >> $RESULTS_FILE
echo ""

log_info "======================================="
log_info "Total Tests: $TOTAL_TESTS"
log_info "Passed: $PASSED_TESTS"
log_info "Failed: $FAILED_TESTS"
log_info "======================================="

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Check $ERRORS_FILE for details.${NC}"
    exit 1
fi
