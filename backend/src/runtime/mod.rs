pub mod worker_pool;
pub mod diagnostics;

pub use worker_pool::WorkerPool;
pub use diagnostics::log_runtime_config;

use log::info;

/// Detect available CPU cores
pub fn detect_cpu_cores() -> usize {
    num_cpus::get()
}

/// Get configured worker thread count
pub fn get_worker_count() -> usize {
    // Default to 3 workers for parallel forecast processing
    // Can be overridden via environment variable
    std::env::var("WORKER_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3)
        .min(num_cpus::get() - 1) // Don't use more than available cores - 1 (reserve 1 for main thread)
        .max(1) // At least 1 worker
}

/// Initialize runtime configuration
pub fn init_runtime() {
    let cpu_cores = detect_cpu_cores();
    let worker_count = get_worker_count();
    
    info!(
        "Initializing Tokio runtime: {} CPU cores detected, {} worker threads configured",
        cpu_cores, worker_count
    );
}
