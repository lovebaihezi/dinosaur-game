use axum::http::StatusCode;
use axum_test::TestServer;
use dinosaur_api::{create_router, NeonTestService};
use std::sync::Arc;

#[tokio::test]
async fn test_is_health_returns_ok_when_connected() {
    let service = Arc::new(NeonTestService::new(true));
    let app = create_router(service);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/is_health").await;
    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_is_health_returns_service_unavailable_when_disconnected() {
    let service = Arc::new(NeonTestService::new(false));
    let app = create_router(service);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/is_health").await;
    response.assert_status(StatusCode::SERVICE_UNAVAILABLE);
}
