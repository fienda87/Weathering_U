# Documentation Index

Complete documentation for the Weather Forecast Application.

## Getting Started

### New to the Project?
Start here for a quick overview and setup:

1. **[QUICKSTART.md](QUICKSTART.md)** - Get up and running in 5 minutes
   - Prerequisites installation
   - Basic setup commands
   - Quick troubleshooting

### Comprehensive Setup Guide
For detailed setup instructions:

2. **[README.md](README.md)** - Complete project documentation
   - Full feature list and architecture overview
   - Detailed prerequisites and installation
   - Backend and frontend setup guides
   - Production deployment instructions
   - Comprehensive troubleshooting section

## Development

### Environment Configuration

3. **[ENV_VARIABLES.md](ENV_VARIABLES.md)** - Environment variables reference
   - Frontend environment variables (VITE_*)
   - Backend environment variables
   - Configuration examples

### Configuration Files

- **`.env.example`** - Frontend environment template
- **`backend/.env.example`** - Backend environment template

Copy these files to create your own `.env` files:
```bash
cp .env.example .env
cp backend/.env.example backend/.env
```

## API Documentation

### Technical Reference

4. **[API_REFERENCE.md](API_REFERENCE.md)** - Complete API specification
   - Endpoint details and parameters
   - Request/response formats
   - Error codes and handling
   - Authentication and CORS
   - Provider fallback strategy
   - Production recommendations

### Code Examples

5. **[EXAMPLES.md](EXAMPLES.md)** - Real-world usage examples
   - Frontend integration (Vue 3, React)
   - Backend scripts (Node.js, Python)
   - cURL testing commands
   - Common patterns (caching, retry logic, batch fetching)
   - Performance monitoring
   - Integration testing

## Implementation Details

6. **[IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)** - Technical implementation notes
   - Feature implementation history
   - Component architecture
   - Service layer details
   - Development decisions

## Quick Reference

### Available Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/cities` | GET | List all Indonesian cities |
| `/api/weather?city=<name>` | GET | Get 7-day weather forecast |

### Project Structure

```
.
├── backend/                 # Rust API server
│   ├── src/
│   │   ├── main.rs         # Server entry point
│   │   ├── routes/         # API endpoints
│   │   ├── services/       # Weather service
│   │   └── models/         # Data models
│   ├── Cargo.toml
│   └── .env.example
├── src/                     # Vue 3 frontend
│   ├── components/         # Reusable components
│   ├── views/             # Page components
│   ├── router/            # Routing
│   └── main.js
├── README.md               # Main documentation
├── QUICKSTART.md          # Quick setup guide
├── API_REFERENCE.md       # API specification
├── EXAMPLES.md            # Usage examples
└── .env.example           # Frontend config template
```

### Common Commands

**Backend:**
```bash
cd backend
cargo run              # Run development server
cargo build --release  # Build for production
cargo test            # Run tests
```

**Frontend:**
```bash
npm install           # Install dependencies
npm run dev          # Run development server
npm run build        # Build for production
npm run preview      # Preview production build
```

### Default Ports

- **Backend API:** `http://localhost:8000`
- **Frontend Dev:** `http://localhost:5173`

### Weather Providers

1. **Open-Meteo** (Primary, free) - Always attempted first
2. **OpenWeatherMap** (Fallback 1) - Requires API key
3. **WeatherAPI** (Fallback 2) - Requires API key

## Troubleshooting

### Quick Fixes

**Backend won't start?**
- Check if Rust is installed: `rustc --version`
- Try clean build: `cd backend && cargo clean && cargo build`

**Frontend won't start?**
- Check Node.js version: `node --version` (need 18+)
- Reinstall dependencies: `rm -rf node_modules && npm install`

**Can't connect to backend?**
- Verify backend is running on port 8000
- Check `VITE_API_BASE_URL` in `.env`
- Test with: `curl http://localhost:8000/api/cities`

**CORS errors?**
- Ensure `CORS_ORIGINS` includes your frontend URL
- Default: `http://localhost:5173,http://localhost:3000`

For detailed troubleshooting, see [README.md#troubleshooting](README.md#troubleshooting).

## Support and Resources

### External Documentation

- [Rocket Framework](https://rocket.rs/) - Rust web framework
- [Vue 3 Documentation](https://vuejs.org/) - Frontend framework
- [Vite](https://vitejs.dev/) - Build tool
- [Tailwind CSS](https://tailwindcss.com/) - Styling

### Weather API Providers

- [Open-Meteo](https://open-meteo.com/) - Free weather API
- [OpenWeatherMap](https://openweathermap.org/api) - Weather API with free tier
- [WeatherAPI](https://www.weatherapi.com/) - Weather API with free tier

## Contributing

When contributing to this project:

1. Follow existing code style and patterns
2. Update relevant documentation
3. Test both backend and frontend changes
4. Ensure all tests pass before submitting

## License

This project is proprietary software for PT. INTI TALENTA ANDALAN.

---

**Last Updated:** November 2024  
**Version:** 1.0.0
