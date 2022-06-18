use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct User {
    id: usize,
    username: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}