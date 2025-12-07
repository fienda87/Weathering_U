pub mod worker_pool;
pub mod diagnostics;

pub use worker_pool::WorkerPool;
pub use diagnostics::log_runtime_config;

use log::info;

/// Deteksi jumlah CPU core yang tersedia
pub fn detect_cpu_cores() -> usize {
    num_cpus::get()
}

    /// Ambil jumlah worker thread yang dikonfigurasi
pub fn get_worker_count() -> usize {
    // Default 3 worker untuk parallel forecast processing
    // Bisa diubah lewat environment variable
    std::env::var("WORKER_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3)
        .min(num_cpus::get() - 1) // Jangan pakai lebih dari cores - 1 (sisain 1 untuk main thread)
        .max(1) // Minimal 1 worker
}

/// Inisialisasi konfigurasi runtime
pub fn init_runtime() {
    let cpu_cores = detect_cpu_cores();
    let worker_count = get_worker_count();
    
    info!(
        "Initializing Tokio runtime: {} CPU cores detected, {} worker threads configured",
        cpu_cores, worker_count
    );
}
