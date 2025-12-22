use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

/// Trait for persistent structure data service that provides database connectivity status
#[async_trait]
pub trait PersistentStructureDataService: Send + Sync {
    /// Check if the database connection is active
    async fn is_connected(&self) -> bool;
}

/// NeonService wraps sqlx PgPool to provide database connectivity
pub struct NeonService {
    pool: PgPool,
}

impl NeonService {
    /// Create a new NeonService with the given PgPool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersistentStructureDataService for NeonService {
    async fn is_connected(&self) -> bool {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .is_ok()
    }
}

/// Application state that holds the data service
pub struct AppState<S: PersistentStructureDataService> {
    pub service: S,
}

/// Health check handler that uses the data service to check database connectivity
pub async fn is_health<S: PersistentStructureDataService>(
    State(state): State<Arc<AppState<S>>>,
) -> StatusCode {
    if state.service.is_connected().await {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

/// Create the application router with the health endpoint
pub fn create_app<S: PersistentStructureDataService + 'static>(state: Arc<AppState<S>>) -> Router {
    Router::new()
        .route("/is_health", get(is_health::<S>))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test service for integration testing
    pub struct NeonTestService {
        connected: bool,
    }

    impl NeonTestService {
        pub fn new(connected: bool) -> Self {
            Self { connected }
        }
    }

    #[async_trait]
    impl PersistentStructureDataService for NeonTestService {
        async fn is_connected(&self) -> bool {
            self.connected
        }
    }

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
