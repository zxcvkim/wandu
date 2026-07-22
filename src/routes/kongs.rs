use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;

use crate::{
    AppError, AppResult, AppState,
    database::kongs::{self, Kong, KongId, MiniKong},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_list).post(handle_create))
        .route(
            "/{id}",
            get(handle_get).put(handle_update).delete(handle_delete),
        )
}

#[derive(Deserialize)]
struct CreateRequest {
    pub profile: String,
    pub content: String,
}

#[derive(Deserialize)]
struct UpdateRequest {
    pub profile: Option<String>,
    pub content: Option<String>,
}

async fn handle_create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> AppResult<(StatusCode, Json<KongId>)> {
    let id = kongs::create(&state.pool, &payload.profile, &payload.content).await?;

    Ok((StatusCode::CREATED, Json(id)))
}

async fn handle_get(
    State(state): State<AppState>,
    Path(id): Path<KongId>,
) -> AppResult<Json<Kong>> {
    let kong = kongs::get(&state.pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(kong))
}

async fn handle_list(State(state): State<AppState>) -> AppResult<Json<Vec<MiniKong>>> {
    let list = kongs::get_all(&state.pool).await?;

    Ok(Json(list))
}

async fn handle_update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRequest>,
) -> AppResult<StatusCode> {
    let updated = kongs::update(
        &state.pool,
        id,
        payload.profile.as_deref(),
        payload.content.as_deref(),
    )
    .await?;

    updated.then_some(StatusCode::OK).ok_or(AppError::NotFound)
}

async fn handle_delete(
    State(state): State<AppState>,
    Path(id): Path<KongId>,
) -> AppResult<StatusCode> {
    kongs::delete(&state.pool, id)
        .await?
        .then_some(StatusCode::NO_CONTENT)
        .ok_or(AppError::NotFound)
}
