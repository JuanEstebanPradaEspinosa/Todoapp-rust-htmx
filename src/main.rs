use api::configure::{configure_app_router, AppState};
use axum::Router;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing::info;
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

    info!("Enabling logging!");

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db_pool = SqlitePool::connect(DB_URL).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    println!("Running migrations from: {}", migrations.display());
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db_pool)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    // build our application
    let app_state = AppState { db_pool };
    let app = Router::new()
        .nest_service("/styles", tower_http::services::ServeDir::new("styles")) // Is needed to have those file access to the browser!
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .merge(configure_app_router(app_state))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // This code is only compiled when in the debug mode for livereload
    #[cfg(debug_assertions)]
    let app = {
        use notify::Watcher;
        let livereload = tower_livereload::LiveReloadLayer::new().request_predicate(
            |req: &axum::http::Request<axum::body::Body>| !req.headers().contains_key("hx-request"),
        );
        let reloader = livereload.reloader();
        let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();
        watcher
            .watch(
                std::path::Path::new("styles"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        watcher
            .watch(
                std::path::Path::new("templates"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        tracing::info!("Reloading!");
        app.layer(livereload)
    };

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//TODO: Adding error handling so that if somethings doesnt work it get an small error message
//TODO: Adding Edits functionality for editing the todos like change title
//TODO: Make the todo list look better and make use of the whole screen
//TODO: Add Readme file for how to get the projects and use it for production etc.... so other people can use it too!
