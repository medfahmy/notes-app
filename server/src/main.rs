use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

mod user;
mod note;

fn main() {
    println!("Hello, world!");
}
