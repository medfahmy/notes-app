use actix_web::{post, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInput {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[post("/u/register")]
pub async fn register(input: Json<UserInput>) -> Json<UserInput> {
    println!("=== registering {:?} ===", input);

    // TODO: validate input
    // TODO: add to db

    Json(UserInput {
        username: input.username.clone(),
        email: input.email.clone(),
        password: input.password.clone(),
        confirm_password: input.confirm_password.clone(),
    })
}

#[post("/u/login")]
pub async fn login(input: Json<UserInput>) -> Json<UserInput> {
    println!("=== registering {:?} ===", input);

    // TODO: validate input
    // TODO: return access token (JWT)

    Json(UserInput {
        username: input.username.clone(),
        email: input.email.clone(),
        password: input.password.clone(),
        confirm_password: input.confirm_password.clone(),
    })
}