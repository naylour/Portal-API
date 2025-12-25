use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::app::AppState;

#[derive(Deserialize)]
pub struct TextInput {
    text: String,
}

#[derive(Serialize)]
pub struct Answer {
    text: String
}

pub async fn query(
    State(_app_state): State<AppState>,
    Json(payload): Json<TextInput>
) -> Json<Answer> {
    let input_text = payload.text;

    println!("{}", input_text);

    Json(Answer { text: input_text })
}
