use log::info;
use crate::runtime::{detect_cpu_cores, get_worker_count};

/// Log runtime configuration details on startup
pub fn log_runtime_config() {
    let cpu_cores = detect_cpu_cores();
    let worker_threads = get_worker_count();

    info!("╔════════════════════════════════════════╗");
    info!("║     Tokio Multi-Thread Runtime         ║");
    info!("╠════════════════════════════════════════╣");
    info!("║ CPU Cores: {:<31} ║", cpu_cores);
    info!("║ Worker Threads: {:<26} ║", worker_threads);
    info!("║ Max Concurrent Tasks: {:<21} ║", worker_threads);
    info!("║ Core 0: Rocket API Server               ║");
    info!("║ Cores 1-{}: Task Workers (Parallel)     ║", worker_threads);
    info!("╚════════════════════════════════════════╝");
}

/// Log current worker pool status
#[allow(dead_code)]
pub fn log_worker_status(available_permits: usize, total_workers: usize) {
    let active = total_workers - available_permits;
    info!(
        "Worker Pool Status: {}/{} active (available permits: {})",
        active, total_workers, available_permits
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_runtime_config() {
        // This test just ensures the function can be called without panicking
        log_runtime_config();
    }

    #[test]
    fn test_log_worker_status() {
        // This test just ensures the function can be called without panicking
        log_worker_status(2, 3);
    }
}
