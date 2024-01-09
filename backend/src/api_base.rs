use axum::Router;

pub trait Api {
    fn register() -> Router;
}

