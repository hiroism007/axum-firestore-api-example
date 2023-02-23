use crate::{app_state::AppState, controller::*, database::firestore::initialize_db};
use axum::{routing::get, Router};
use hyper::Method;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_route(project_id: &str) -> Router {
    let db = initialize_db(project_id.to_string()).await;
    let app_state = Arc::new(AppState::new(db));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hc", get(health_check::hc_server))
        .route("/hc_db_r", get(health_check::hc_db_r))
        .route("/hc_db_w", get(health_check::hc_db_w))
        .layer(cors)
        .with_state(app_state)
}
