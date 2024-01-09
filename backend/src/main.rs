use axum::Router;
use tracing::{event, Level};

use crate::api_base::Api;

pub mod student_api;
pub mod api_base;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(student_api::StudentAPI::register());

    event!(Level::INFO, "Hello world");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
