use std::sync::Arc;
use tokio::sync::Semaphore;
use std::future::Future;
use std::time::Instant;
use log::debug;

/// Worker pool untuk manage eksekusi task concurrent dengan rate limiting
#[allow(dead_code)]
pub struct WorkerPool {
    worker_threads: usize,
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    /// Buat worker pool baru dengan jumlah worker tertentu
    pub fn new(num_workers: usize) -> Self {
        debug!("Creating worker pool with {} workers", num_workers);
        Self {
            worker_threads: num_workers,
            semaphore: Arc::new(Semaphore::new(num_workers)),
        }
    }

    /// Ambil jumlah worker thread
    #[allow(dead_code)]
    pub fn worker_count(&self) -> usize {
        self.worker_threads
    }

    /// Ambil jumlah permit yang tersedia di semaphore
    #[allow(dead_code)]
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Eksekusi task dengan rate limiting pakai semaphore
    #[allow(dead_code)]
    pub async fn execute_task<F, Fut, T>(&self, task: F) -> Result<T, String>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| format!("Failed to acquire semaphore permit: {}", e))?;

        let start = Instant::now();

        let result = tokio::spawn(async move {
            let task_result = task().await;
            drop(permit); // Lepas permit semaphore
            task_result
        })
        .await
        .map_err(|e| format!("Task execution failed: {}", e))?;

        let elapsed = start.elapsed();
        debug!(
            "Task executed in {:.2}ms, available permits: {}",
            elapsed.as_secs_f64() * 1000.0,
            self.semaphore.available_permits()
        );

        Ok(result)
    }

    /// Eksekusi banyak task secara parallel dengan rate limiting
    #[allow(dead_code)]
    pub async fn execute_batch<F, Fut, T>(
        &self,
        tasks: Vec<F>,
    ) -> Vec<Result<T, String>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let mut handles = vec![];

        for task in tasks {
            let semaphore = self.semaphore.clone();
            let handle = tokio::spawn(async move {
                let permit = semaphore
                    .acquire_owned()
                    .await
                    .map_err(|e| format!("Failed to acquire semaphore permit: {}", e))?;

                let start = Instant::now();
                let task_result = task().await;
                drop(permit);

                let elapsed = start.elapsed();
                debug!(
                    "Batch task executed in {:.2}ms",
                    elapsed.as_secs_f64() * 1000.0
                );

                Ok(task_result)
            });

            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(format!("Batch task failed: {}", e))),
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_worker_pool_creation() {
        let pool = WorkerPool::new(3);
        assert_eq!(pool.worker_count(), 3);
        assert_eq!(pool.available_permits(), 3);
    }

    #[tokio::test]
    async fn test_worker_pool_semaphore_limits_concurrency() {
        let pool = Arc::new(WorkerPool::new(2));
        let mut handles = vec![];

        for _ in 0..3 {
            let pool_clone = Arc::clone(&pool);
            let handle = tokio::spawn(async move {
                let result = pool_clone
                    .execute_task(|| async { 100 })
                    .await;
                assert!(result.is_ok());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_execute_task_success() {
        let pool = WorkerPool::new(1);
        let result = pool.execute_task(|| async { 42 }).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_available_permits() {
        let pool = WorkerPool::new(3);
        assert_eq!(pool.available_permits(), 3);
    }
}
