use axum::extract::FromRef;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use sqlx::SqlitePool;

use super::todo;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

pub fn configure_app_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(todo::index))
        .route("/todos", get(todo::list))
        .route("/todos/create", post(todo::create))
        .route("/todos/{id}", get(todo::get))
        .route("/todos/{id}/toggle", patch(todo::toggle))
        .route("/todos/{id}/update", put(todo::update))
        .route("/todos/{id}/delete", delete(todo::delete))
        .with_state(app_state)
}
