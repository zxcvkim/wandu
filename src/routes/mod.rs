mod kongs;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/kongs", kongs::router())
}
