use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form,
};
use sqlx::SqlitePool;
use tracing::info;

use crate::{
    controllers::{self},
    domain::todo::{convert_list, Mappable, TodoCreateForm},
    presentation::components::{HelloTemplate, HtmlTemplate, LandingPage, TodoList, TodoTemplate},
};

pub async fn index(State(db_pool): State<SqlitePool>) -> impl IntoResponse {
    let todos = controllers::todo::get_list(&db_pool).await;
    let todos: Vec<TodoTemplate> = convert_list(todos);

    let todo_list = TodoList { todos };
    let template = LandingPage { todo_list };

    HtmlTemplate(template)
}

pub async fn list(State(db_pool): State<SqlitePool>) -> impl IntoResponse {
    let todos = controllers::todo::get_list(&db_pool).await;
    let todos: Vec<TodoTemplate> = convert_list(todos);

    let template_todo_list = TodoList { todos };
    HtmlTemplate(template_todo_list)
}

pub async fn get(State(db_pool): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    let todo = controllers::todo::get(&db_pool, id).await;
    HtmlTemplate(todo.map_to())
}

pub async fn create(
    State(db_pool): State<SqlitePool>,
    Form(input): Form<TodoCreateForm>,
) -> impl IntoResponse {
    info!("Adding Todo '{0}'", input.title);
    let new_todo = controllers::todo::create_todo(&db_pool, input).await;
    HtmlTemplate(new_todo.map_to())
}

pub async fn toggle(State(db_pool): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    let new_toggle_todo = controllers::todo::toggle_todo(&db_pool, id).await;
    HtmlTemplate(new_toggle_todo.map_to())
}

pub async fn update(
    State(db_pool): State<SqlitePool>,
    Path(id): Path<i64>,
    title: String,
) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

pub async fn delete(State(db_pool): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    controllers::todo::delete_todo(&db_pool, id).await;
    StatusCode::OK
}
