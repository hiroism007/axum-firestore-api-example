mod app_error;
mod app_state;
mod controller;
mod database;
mod entity;
mod route;

use dotenv::dotenv;
use route::create_route;
use std::env;

pub async fn run() {
    dotenv().ok();

    let project_id = env::var("PROJECT_ID").expect("PROJECT_ID must be set");
    let app = create_route(&project_id).await;

    axum::Server::bind(
        &format!(
            "0.0.0.0:{port}",
            port = env::var("PORT").unwrap_or("8080".to_string())
        )
        .parse()
        .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
