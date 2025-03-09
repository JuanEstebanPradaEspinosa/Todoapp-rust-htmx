use sqlx::SqlitePool;

use crate::domain::todo::{EntityTodo, TodoCreateForm};

pub async fn create(db_pool: &SqlitePool, create_todo: TodoCreateForm) -> EntityTodo {
    let result = sqlx::query_as::<_, EntityTodo>(
        r#" 
        INSERT INTO todos (title,done) 
        VALUES ($1, $2) 
        RETURNING id, title, done
        "#,
    )
    .bind(create_todo.title)
    .bind(false)
    .fetch_one(db_pool)
    .await
    .unwrap();

    result
}

pub async fn list(db_pool: &SqlitePool) -> Vec<EntityTodo> {
    let result = sqlx::query_as::<_, EntityTodo>(
        r#"
        SELECT id, title, done FROM todos
        "#,
    )
    .fetch_all(db_pool)
    .await
    .unwrap();

    result
}

pub async fn get(db_pool: &SqlitePool, id: i64) -> EntityTodo {
    let result = sqlx::query_as::<_, EntityTodo>("SELECT id, title, done FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(db_pool)
        .await
        .unwrap();

    result
}

pub async fn delete(db_pool: &SqlitePool, id: i64) {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(db_pool)
        .await
        .unwrap();

    //When implementing error handling check if is is succeeded!
}

pub async fn toggle(db_pool: &SqlitePool, id: i64) -> EntityTodo {
    // Update the "done" field (toggle it between true/false)
    sqlx::query("UPDATE todos SET done = NOT done WHERE id = ?")
        .bind(id)
        .execute(db_pool)
        .await
        .unwrap();

    let updated_todo =
        sqlx::query_as::<_, EntityTodo>("SELECT id, title, done FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(db_pool)
            .await
            .unwrap();

    updated_todo
}
