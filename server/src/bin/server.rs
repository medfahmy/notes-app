use std::net::SocketAddr;
use axum::routing::{get, post};
use axum::{Router, Server};
use server::{user, note};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(user::get_users).post(user::create_user))
        .route("/notes", get(note::get_notes).post(note::create_note));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}