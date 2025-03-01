use axum::{extract::Path, response::IntoResponse};

use crate::presentation::components::{HelloTemplate, HtmlTemplate, LandingPage, Todo, TodoList};

pub async fn index() -> impl IntoResponse {
    let todos = vec![
        Todo {
            id: 1,
            title: "Buy groceries".to_string(),
            done: false,
        },
        Todo {
            id: 2,
            title: "Write Rust code".to_string(),
            done: true,
        },
    ];

    let todo_list = TodoList { todos };
    let template = LandingPage { todo_list };
    HtmlTemplate(template)
}

pub async fn list() -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

pub async fn get_todo(Path(id): Path<i32>) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

pub async fn create() -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

pub async fn toggle(Path(id): Path<i32>) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

pub async fn update(Path(id): Path<i32>, title: String) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}

pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template)
}
