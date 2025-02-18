use axum::{routing::get, Router};
// use domain::todo::Todo;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;
pub mod controllers;
pub mod domain;
pub mod presentation;
pub mod repositories;

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    println!("Running migrations from: {}", migrations.display());
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

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
}
