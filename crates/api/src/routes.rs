use crate::PersistentStructureDataService;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;

/// Health check response handler.
/// Returns 200 OK if database is connected, 503 Service Unavailable otherwise.
pub async fn is_health<S: PersistentStructureDataService + Send + Sync>(
    State(service): State<Arc<S>>,
) -> impl IntoResponse {
    if service.is_connected().await {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

/// Create the API router with health check endpoint.
pub fn create_router<S: PersistentStructureDataService + Send + Sync + 'static>(
    service: Arc<S>,
) -> Router {
    Router::new()
        .route("/is_health", get(is_health::<S>))
        .with_state(service)
}
