use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::Path;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    id: usize,
    username: String,
    email: String
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
}

// GET users/
pub async fn get_users() -> impl IntoResponse {

}

// GET users/:id
pub async fn get_user_by_id(Path(id): Path<Uuid>) -> impl IntoResponse {
    let users: Vec<User> = vec![];

    (StatusCode::OK, Json(users))
}

// POST users/
pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User { // db.save()
        id: 1337,
        username: payload.username,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(user))
}

// POST users/:id
pub async fn update_user_by_id(id: usize) -> impl IntoResponse {
    // db.update({ id })
    
    StatusCode::OK
}