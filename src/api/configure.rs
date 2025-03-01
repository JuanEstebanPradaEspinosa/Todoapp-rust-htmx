use axum::extract::FromRef;
use axum::routing::{delete, get, post, put};
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
        .route("/todos", post(todo::create))
        .route("/todos/{id}", get(todo::get_todo))
        .route("/todos/{id}/toggle", put(todo::toggle))
        .route("/todos/{id}/text", put(todo::update))
        .route("/todos/{id}", delete(todo::delete))
        .with_state(app_state)
}
