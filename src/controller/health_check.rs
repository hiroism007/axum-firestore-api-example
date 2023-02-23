use crate::app_state::AppState;
use crate::entity::test::Test;
use crate::utility::app_error::AppError;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use std::{env, sync::Arc};

pub async fn hc_server() -> String {
    let port = env::var("PORT").unwrap();
    format!("Server is running on port: {}", port)
}

pub async fn hc_db(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let db = state.db.clone();

    let test = Test {
        id: Some("hige".to_string()),
        name: "hige".to_string(),
        created_at: Some(DateTime::from(Utc::now())),
    };

    let res: Test = db
        .fluent()
        .insert()
        .into("tests")
        .generate_document_id()
        .object(&test)
        .execute()
        .await
        .map_err(|err| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert test: {:?}", err),
            )
        })?;

    println!("res: {:?}", res);

    Ok((StatusCode::CREATED, "OK").into_response())
}
