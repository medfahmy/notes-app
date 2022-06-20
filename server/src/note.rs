use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
}

#[derive(Deserialize)]
pub struct CreateNote {
    title: String,
    content: String,
}

// GET notes/
pub async fn get_notes() -> impl IntoResponse {

}

// GET notes/:id
pub async fn get_note_by_id(Path(id): Path<Uuid>) -> impl IntoResponse {
    let notes: Vec<Note> = vec![];

    (StatusCode::OK, Json(notes))
}

// POST notes/
pub async fn create_note(Json(payload): Json<CreateNote>) -> impl IntoResponse {
    let note = Note { // db.save()
        id: 1337,
        title: payload.title,
        content: payload.content,
    };

    (StatusCode::CREATED, Json(note))
}

// POST notes/:id
pub async fn update_note_by_id(id: usize) -> impl IntoResponse {
    // db.update({ id })
    
    StatusCode::OK
}