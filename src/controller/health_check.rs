use crate::app_state::AppState;
use crate::entity::test::Test;
use crate::utility::app_error::AppError;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use std::{env, sync::Arc};

pub async fn hc_server() -> String {
    let port = env::var("PORT").unwrap();
    format!("Server is running on port: {}", port)
}

#[derive(Deserialize)]
pub struct TestQuery {
    id: Option<String>,
}

pub async fn hc_db_r(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TestQuery>,
) -> Result<impl IntoResponse, AppError> {
    let db = state.db.clone();

    let id = params.id;
    if id.is_none() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "id is required".to_string(),
        ));
    }

    let res: Option<Test> = db
        .fluent()
        .select()
        .by_id_in("tests")
        .obj()
        .one(id.unwrap())
        .await
        .map_err(|err| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to select test: {:?}", err),
            )
        })?;

    println!("res: {:?}", res);

    Ok((StatusCode::OK, Json(res)).into_response())
}

pub async fn hc_db_w(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let db = state.db.clone();

    let test = Test {
        id: None,
        name: "Yuki".to_string(),
        created_at: Utc::now(),
        updated_at: None,
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

    Ok((StatusCode::CREATED, Json(res)).into_response())
}
