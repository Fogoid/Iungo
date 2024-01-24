use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tracing::{event, Level};

#[derive(sqlx::Type)]
#[derive(Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub author: String,
    pub content: String,
    pub visibility: Visibility
}

pub async fn new_post(
    State(pool): State<MySqlPool>,
    Json(post): Json<Post>
) -> Result<String, StatusCode> {
    event!(Level::TRACE, "Starting new post request");

    let query = sqlx::query!(
            r#"
            INSERT INTO posts (author, content, visibility)
              VALUES (?, ?, ?)
            "#,
        post.author, post.content, post.visibility)
        .execute(&pool)
        .await;

    let new_id = match query {
        Ok(x) => {
            x.last_insert_id()
        }
        Err(x) => {
            event!(Level::ERROR, "Error inserting post: {}", x);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    return Ok(new_id.to_string());
}
