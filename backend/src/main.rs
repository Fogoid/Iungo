pub mod api;

use std::env;

use axum::routing::post;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db_conn = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn)
        .await
        .expect("Can't connect to database");

    let app = axum::Router::new()
        .route("/api/posts", post(api::new_post))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
    
