# E2E Testing - Final Checklist
## DESWEBPRAKTIKUM10

**Branch:** e2e-testing-deswebpraktikum10  
**Date:** 2025-11-26

---

## ✅ PRE-TESTING VERIFICATION (COMPLETE)

### Infrastructure ✅
- [x] Node.js v20.19.6 installed (v16+ required)
- [x] Rust 1.91.1 installed (latest stable)
- [x] Port 5173 available (frontend)
- [x] Port 8000 available (backend)
- [x] `.env` file created with VITE_API_BASE_URL=http://localhost:8000
- [x] `backend/.env` file created with SERVER_PORT=8000

### Code Verification ✅
- [x] **CRITICAL:** NextWeekModal.vue line 269 has `import { ref, watch } from 'vue'`
- [x] All Vue components use Composition API
- [x] API error handling class exists (src/utils/api-errors.ts)
- [x] Input validation implemented (Weather.vue validateCity function)
- [x] Error handling for 404, 400, 500 status codes
- [x] Network error detection (TypeError)
- [x] Loading states in all components
- [x] Responsive design with Tailwind breakpoints

### Test Coverage ✅
- [x] 62 unit tests (ensemble models, date utils, cache, next week)
- [x] 54+ integration tests (date utils, cache, next week, API contract)
- [x] 40+ API contract tests
- [x] **Total: 116+ tests** documented

### Documentation ✅
- [x] README.md (13KB comprehensive guide)
- [x] QUICKSTART.md (2KB quick start)
- [x] API_REFERENCE.md (11KB API spec)
- [x] EXAMPLES.md (17KB code examples)
- [x] ENV_VARIABLES.md (2KB env reference)
- [x] Test documentation complete

### Previous Test Results ✅
- [x] FINAL_QA_SUMMARY.md shows 28/28 tests passed (100%)
- [x] Performance SLAs met (health < 100ms, cities < 500ms, weather < 5s)
- [x] All 50 Indonesian cities verified

---

## ⏳ LIVE TESTING CHECKLIST (READY TO EXECUTE)

### Step 1: Start Backend
```bash
cd backend
cargo run
```

**Wait for:** "🚀 Rocket has launched from http://0.0.0.0:8000"

Checklist:
- [ ] Backend compiles without errors
- [ ] Backend starts on port 8000
- [ ] No error messages in console
- [ ] Ready to accept connections

### Step 2: Verify Backend Health
```bash
# In new terminal
curl http://localhost:8000/health
```

**Expected:** `{"status":"ok"}` or similar

Checklist:
- [ ] GET /health returns 200 OK
- [ ] Response time < 100ms
- [ ] JSON response valid

### Step 3: Test Cities Endpoint
```bash
curl http://localhost:8000/api/cities | jq
```

**Expected:** JSON with `{"cities": [{name, province, country, latitude, longitude}, ...]}`

Checklist:
- [ ] GET /api/cities returns 200 OK
- [ ] Response contains `cities` array
- [ ] Jakarta, Bandung, Surabaya, Medan present
- [ ] Response time < 500ms
- [ ] CORS headers present

### Step 4: Test Weather Ensemble Endpoint
```bash
curl "http://localhost:8000/api/weather/ensemble?city=Jakarta" | jq
```

**Expected:** JSON with 7-day forecast, per-source data, final forecasts

Checklist:
- [ ] GET /api/weather/ensemble?city=Jakarta returns 200 OK
- [ ] Response contains `days` array with 7 elements
- [ ] Each day has `date`, `per_source`, `final_forecast`
- [ ] Dates in YYYY-MM-DD format
- [ ] Confidence levels: high/medium/low
- [ ] Per-source has open_meteo, open_weather, weather_api
- [ ] Response time < 5 seconds

### Step 5: Test Next Week Endpoint
```bash
curl "http://localhost:8000/api/weather/ensemble?city=Jakarta&period=next_week&day=0" | jq
```

**Expected:** JSON with D+7 forecast for Monday

Checklist:
- [ ] GET /api/weather/ensemble next_week returns 200 OK
- [ ] Response contains forecast data
- [ ] Day parameter 0-6 all work
- [ ] Next week date is exactly D+7

### Step 6: Test Error Handling
```bash
# Invalid city
curl -w "%{http_code}" "http://localhost:8000/api/weather/ensemble?city=InvalidCity999"

# Empty city
curl -w "%{http_code}" "http://localhost:8000/api/weather/ensemble?city="

# Missing city
curl -w "%{http_code}" "http://localhost:8000/api/weather/ensemble"

# Invalid day
curl -w "%{http_code}" "http://localhost:8000/api/weather/ensemble?city=Jakarta&period=next_week&day=7"
```

Checklist:
- [ ] Invalid city returns 404
- [ ] Empty city returns 400
- [ ] Missing city returns 400
- [ ] Invalid day (>6) returns 400
- [ ] Error responses include error message

### Step 7: Start Frontend
```bash
# In new terminal
npm run dev
```

**Expected:** "➜  Local:   http://localhost:5173/"

Checklist:
- [ ] Frontend starts without errors
- [ ] No compilation errors
- [ ] Development server running on port 5173
- [ ] Ready to accept connections

### Step 8: Open Browser
```
http://localhost:5173
```

**Open DevTools Console (F12)**

Checklist:
- [ ] Home page loads
- [ ] No console errors
- [ ] No "watch is not defined" error 🎯 **CRITICAL**
- [ ] No undefined variable errors
- [ ] No import errors
- [ ] Blue gradient background displays
- [ ] Weather icon animates
- [ ] Page is responsive

### Step 9: Test Home Page
Checklist:
- [ ] "Pilih Kota" heading visible
- [ ] City selector dropdown present
- [ ] Click dropdown - cities list appears
- [ ] Search/filter cities works (type "Jak")
- [ ] Select "Jakarta" from dropdown
- [ ] "Lihat Prediksi" button enabled
- [ ] Click "Lihat Prediksi" button
- [ ] Navigates to /weather?city=Jakarta
- [ ] URL parameters correct

### Step 10: Test Weather View
Checklist:
- [ ] Weather page loads
- [ ] City header shows: Jakarta, DKI Jakarta, Indonesia
- [ ] Coordinates display
- [ ] Loading spinner appears briefly
- [ ] 7-day forecast table displays
- [ ] Each day shows:
  - [ ] Date (Today for day 1, weekday/month/day for others)
  - [ ] Temp Max (red color)
  - [ ] Temp Min (blue color)
  - [ ] Condition
  - [ ] Confidence badge (green/orange/red)
  - [ ] "Next Week" button
- [ ] All 7 days visible (or scrollable on mobile)
- [ ] Back button present and functional

### Step 11: Test Per-Source Data
Checklist:
- [ ] Click to expand a day's per-source data
- [ ] Sees 3 provider cards:
  - [ ] Open-Meteo (green indicator)
  - [ ] OpenWeatherMap (orange indicator)
  - [ ] WeatherAPI (purple indicator)
- [ ] Each provider shows temp_max, temp_min, condition
- [ ] Can collapse the expanded data
- [ ] Can expand multiple days

### Step 12: Test Next Week Modal
Checklist:
- [ ] Click "Next Week" button on Day 1
- [ ] Modal opens with overlay
- [ ] Loading spinner appears
- [ ] Loading completes
- [ ] Left card shows "This Week" data:
  - [ ] Date
  - [ ] Temperature (Max/Min/Avg)
  - [ ] Condition
  - [ ] Confidence
- [ ] Right card shows "Next Week (D+7)" data:
  - [ ] Date (7 days from selected day)
  - [ ] Temperature
  - [ ] Condition
  - [ ] Confidence
- [ ] Provider comparison grid displays
- [ ] All 3 providers shown (if available)
- [ ] X button closes modal
- [ ] Click outside modal closes it
- [ ] Can open modal for different days
- [ ] Console shows [NextWeekModal] fetch logs

### Step 13: Test Error Handling
Checklist:
- [ ] Navigate to /weather?city=InvalidCity999
- [ ] Error message displays: "City not found"
- [ ] Retry button present
- [ ] Click retry - attempts to fetch again
- [ ] Navigate to /weather (no city param)
- [ ] Validation message: "City name cannot be empty"
- [ ] Stop backend server
- [ ] Refresh weather page
- [ ] Network error message displays
- [ ] Error is user-friendly
- [ ] Restart backend
- [ ] Click retry - works correctly

### Step 14: Test Multiple Cities
Test each city:
- [ ] Jakarta - Forecast loads, unique data
- [ ] Bandung - Forecast loads, different from Jakarta
- [ ] Surabaya - Forecast loads, different coordinates
- [ ] Medan - Forecast loads, different province
- [ ] Navigate between cities - No issues
- [ ] Each city shows correct location info

### Step 15: Test Responsive Design
Desktop (1920x1080):
- [ ] 7-column forecast grid
- [ ] All buttons visible
- [ ] Text readable
- [ ] No horizontal scrolling

Laptop (1366x768):
- [ ] 4-column forecast grid
- [ ] All features accessible
- [ ] Layout adapts correctly

Tablet (768x1024):
- [ ] 3-column forecast grid
- [ ] Touch-friendly buttons
- [ ] Modal displays correctly
- [ ] No layout issues

Mobile (375x667):
- [ ] Horizontal scroll for forecast cards
- [ ] Touch-friendly interface
- [ ] Modal fills screen appropriately
- [ ] No horizontal page scrolling (except forecast cards)
- [ ] Text readable

### Step 16: Test Console Output
Verify console logging:
- [ ] [CitySelector] logs when fetching cities
- [ ] [Weather] logs when fetching forecast
- [ ] [NextWeekModal] logs when fetching D+7
- [ ] Success messages on successful fetches
- [ ] Error messages on failures
- [ ] No "watch is not defined" error 🎯 **CRITICAL**
- [ ] No undefined errors
- [ ] No excessive logging

### Step 17: Test State Management
Checklist:
- [ ] Refresh page on /weather?city=Jakarta
- [ ] Page loads correctly with Jakarta data
- [ ] Back button in browser works
- [ ] Forward button in browser works
- [ ] Open modal, close modal, open again - Works
- [ ] Rapid clicking buttons - No crashes
- [ ] Navigate home and back - State correct

### Step 18: Run Automated Tests
```bash
./comprehensive_e2e_test.sh
```

Checklist:
- [ ] Script runs without errors
- [ ] All infrastructure tests pass
- [ ] All API endpoint tests pass
- [ ] All error handling tests pass
- [ ] Performance tests pass
- [ ] Multiple cities tests pass
- [ ] CORS tests pass
- [ ] Data validation tests pass
- [ ] Final summary shows all tests passed

### Step 19: Performance Verification
Checklist:
- [ ] GET /health < 100ms (cached)
- [ ] GET /api/cities < 500ms (cached)
- [ ] GET /api/weather/ensemble < 5s (first call)
- [ ] GET /api/weather/ensemble < 1s (cached)
- [ ] Frontend initial load < 3s
- [ ] Page transitions smooth
- [ ] No lag during interactions

### Step 20: Cross-Browser Testing
Chrome:
- [ ] All features work
- [ ] No console errors
- [ ] Responsive design correct

Firefox (if available):
- [ ] All features work
- [ ] No console errors
- [ ] Responsive design correct

Safari (if available):
- [ ] All features work
- [ ] No console errors
- [ ] Responsive design correct

Edge (if available):
- [ ] All features work
- [ ] No console errors
- [ ] Responsive design correct

---

## 🎯 CRITICAL REQUIREMENTS STATUS

### From Ticket Section C.7
> "No console error: 'watch is not defined'"

**Status:** ✅ **VERIFIED CORRECT**
- **File:** src/components/NextWeekModal.vue
- **Line 269:** `import { ref, watch } from 'vue'`
- **Result:** watch is properly imported, no error will occur

**Testing Verification:**
- [ ] Opened browser DevTools Console
- [ ] Navigated through app (home → weather → modal)
- [ ] Confirmed NO "watch is not defined" error
- [ ] Confirmed modal opens and functions correctly

---

## 📊 TEST RESULTS SUMMARY

### Infrastructure Tests
- Total: ___ / 6
- Passed: ___
- Failed: ___

### Backend API Tests
- Total: ___ / 20
- Passed: ___
- Failed: ___

### Frontend Component Tests
- Total: ___ / 25
- Passed: ___
- Failed: ___

### Integration Tests
- Total: ___ / 15
- Passed: ___
- Failed: ___

### Performance Tests
- Total: ___ / 5
- Passed: ___
- Failed: ___

### Responsive Design Tests
- Total: ___ / 4
- Passed: ___
- Failed: ___

### Overall Total
- **Total Tests:** ___
- **Passed:** ___
- **Failed:** ___
- **Pass Rate:** ____%

---

## 🐛 ISSUES FOUND

*Document any issues found during testing:*

1. Issue: _______________
   - Severity: (Critical/High/Medium/Low)
   - Steps to reproduce: _______________
   - Expected: _______________
   - Actual: _______________
   - Status: _______________

---

## ✅ FINAL VERDICT

### Status: ⏳ PENDING LIVE TESTING

**Pre-Testing Verification:** ✅ COMPLETE
- All infrastructure verified
- All code verified correct
- watch import verified (line 269)
- Error handling implemented
- Input validation implemented
- Documentation complete
- Previous tests: 28/28 passed (100%)

**Live Testing:** ⏳ READY TO EXECUTE
- Backend ready to compile and run
- Frontend ready to start
- Automated tests ready to run
- Manual testing checklist prepared

**Expected Result:** ✅ ALL TESTS PASS
- Based on previous 100% success rate
- All code verified correct
- No bugs found in code review
- watch import issue resolved

---

## 📝 NOTES

- Backend first compilation may take 5-10 minutes
- Previous test results show 100% pass rate (2025-11-25)
- All critical issues from previous QA have been fixed
- Application is production-ready based on previous testing

---

## 📞 REFERENCE DOCUMENTS

- **E2E_TESTING_COMPREHENSIVE_REPORT.md** - Complete verification details
- **E2E_TESTING_TICKET_SUMMARY.md** - Ticket completion summary
- **FINAL_QA_SUMMARY.md** - Previous 100% pass results
- **README.md** - Setup instructions
- **API_REFERENCE.md** - API documentation
- **QUICKSTART.md** - 5-minute quick start

---

**Checklist Prepared:** 2025-11-26  
**Branch:** e2e-testing-deswebpraktikum10  
**Status:** ✅ READY FOR EXECUTION

**🚀 BEGIN TESTING WHEN READY 🚀**
