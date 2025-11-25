# Quick Start Guide

Get the weather forecast application running in under 5 minutes!

## Prerequisites

- Rust toolchain installed ([rustup.rs](https://rustup.rs/))
- Node.js 18+ installed ([nodejs.org](https://nodejs.org/))

## 1. Clone and Setup

```bash
# Clone the repository (if not already done)
git clone <repository-url>
cd <project-directory>
```

## 2. Start the Backend

```bash
# Navigate to backend
cd backend

# Run the server (dependencies will auto-install)
cargo run

# You should see:
# "Starting IndoPrint API server on port 8000"
```

Leave this terminal running.

## 3. Start the Frontend

Open a **new terminal** in the project root:

```bash
# Install dependencies
npm install

# Create environment file (optional - has defaults)
cp .env.example .env

# Start development server
npm run dev

# Open http://localhost:5173 in your browser
```

## 4. Use the Application

1. Open `http://localhost:5173` in your browser
2. Select a city from the dropdown (e.g., "Jakarta")
3. Click "Lihat Prediksi" to view the 7-day forecast

## That's It! 🎉

The app is now running with:
- Backend API at `http://localhost:8000`
- Frontend at `http://localhost:5173`

## Next Steps

- **Add API Keys** (optional): Configure OpenWeatherMap and WeatherAPI keys in `backend/.env` for fallback providers
- **Read Full Documentation**: Check `README.md` for detailed setup, API docs, and troubleshooting
- **Production Build**: See `README.md` for production deployment instructions

## Troubleshooting Quick Fixes

**Backend won't start?**
```bash
# Update Rust
rustup update

# Try a clean build
cd backend
cargo clean
cargo build
```

**Frontend won't start?**
```bash
# Clean install
rm -rf node_modules package-lock.json
npm install
```

**Can't connect to backend?**
- Ensure backend is running and shows "Starting IndoPrint API server on port 8000"
- Check `VITE_API_BASE_URL` in `.env` is set to `http://localhost:8000`
- Test backend: `curl http://localhost:8000/api/cities`

For more help, see the full [README.md](README.md) troubleshooting section.
