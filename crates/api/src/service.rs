use sqlx::PgPool;
use std::future::Future;

/// Trait for persistent structure data services.
/// This trait provides methods to check database connectivity.
pub trait PersistentStructureDataService {
    /// Check if the database connection is active.
    fn is_connected(&self) -> impl Future<Output = bool> + Send;
}

/// NeonService wraps a sqlx PgPool for PostgreSQL database operations.
pub struct NeonService {
    pool: PgPool,
}

impl NeonService {
    /// Create a new NeonService with the given PgPool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PersistentStructureDataService for NeonService {
    async fn is_connected(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }
}

/// NeonTestService is a test implementation of PersistentStructureDataService.
/// It returns a configurable connection status for testing purposes.
pub struct NeonTestService {
    connected: bool,
}

impl NeonTestService {
    /// Create a new NeonTestService with the given connection status.
    pub fn new(connected: bool) -> Self {
        Self { connected }
    }
}

impl PersistentStructureDataService for NeonTestService {
    async fn is_connected(&self) -> bool {
        self.connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_neon_test_service_connected() {
        let service = NeonTestService::new(true);
        assert!(service.is_connected().await);
    }

    #[tokio::test]
    async fn test_neon_test_service_disconnected() {
        let service = NeonTestService::new(false);
        assert!(!service.is_connected().await);
    }
}
