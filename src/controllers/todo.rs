use sqlx::SqlitePool;

use crate::{
    domain::todo::{Mappable, Todo, TodoCreateForm},
    repositories::repo_todos,
};

pub async fn create_todo(db_pool: &SqlitePool, create_todo: TodoCreateForm) -> Todo {
    let new_todo = repo_todos::create(db_pool, create_todo).await;

    new_todo.map_to()
}

pub async fn get_list(db_pool: &SqlitePool) -> Vec<Todo> {
    let todo_list = repo_todos::list(db_pool).await;

    let todos: Vec<Todo> = todo_list.into_iter().map(|todo| todo.map_to()).collect();

    todos
}

pub async fn get(dp_pool: &SqlitePool, id: i64) -> Todo {
    let todo = repo_todos::get(dp_pool, id).await;

    todo.map_to()
}

pub async fn delete_todo(db_pool: &SqlitePool, id: i64) {
    repo_todos::delete(db_pool, id).await;
}

pub async fn toggle_todo(dp_pool: &SqlitePool, id: i64) -> Todo {
    let entitytodo = repo_todos::toggle(dp_pool, id).await;

    entitytodo.map_to()
}
