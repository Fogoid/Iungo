pub mod api;

use axum::routing::get;
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = axum::Router::new()
        .route("/", get(|| async {
            event!{Level::INFO, "I'm Here"};

            "Hello, World!"
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
    
