use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use dinosaur_backend::{create_app, AppState, PersistentStructureDataService};
use std::sync::Arc;
use tower::ServiceExt;

/// Test service for integration testing that returns configurable connection status
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
async fn test_is_health_returns_ok_when_connected() {
    let service = NeonTestService::new(true);
    let state = Arc::new(AppState { service });
    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/is_health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_is_health_returns_service_unavailable_when_disconnected() {
    let service = NeonTestService::new(false);
    let state = Arc::new(AppState { service });
    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/is_health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}
