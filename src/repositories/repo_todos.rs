// let result = sqlx::query("INSERT INTO todos (title) VALUES (?)")
//     .bind("bobby")
//     .execute(&db)
//     .await
//     .unwrap();

// println!("Query result: {:?}", result);

// let todo_results = sqlx::query_as::<_, Todo>("SELECT id, title,  FROM todos")
//     .fetch_all(&db)
//     .await
//     .unwrap();

// for todo in todo_results {
//     println!("[{}] name: {}, active: {}", todo.id, &todo.title, todo.done);
// }
