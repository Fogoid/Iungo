use axum::{Router, routing::get, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use tracing::{event, Level};

use crate::api_base::Api;

pub struct StudentAPI;

impl Api for StudentAPI {
    fn register() -> Router {
        Router::new()
            .route("/api/student", get(students))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Student {
    name: String,
    age: i32
}

impl Student {
    fn new(name: String, age: i32) -> Student {
        Student { name, age }
    }
}

pub async fn students() -> (StatusCode, Json<Vec<Student>>) {
    event!(Level::INFO, "Obtaining students");

    let s1 = Student::new("Ana".to_string(), 25);
    let s2 = Student::new("Diogo".to_string(), 25);

    (StatusCode::OK,  vec![s1, s2].into())
}

