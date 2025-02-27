use axum::{extract::Path, response::IntoResponse};

use crate::presentation::components::{HelloTemplate, HtmlTemplate};

#[axum::debug_handler]
pub async fn list() -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

#[axum::debug_handler]
pub async fn get_todo(Path(id): Path<i32>) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

#[axum::debug_handler]
pub async fn create() -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

#[axum::debug_handler]
pub async fn toggle(Path(id): Path<i32>) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

#[axum::debug_handler]
pub async fn update(Path(id): Path<i32>, title: String) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

#[axum::debug_handler]
pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}
