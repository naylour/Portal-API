use crate::app::AppState;
use axum::extract::State;

use utoipa;

#[
    utoipa::path(
    post,
    path="/v1/threat",
    tag="Угрозы",
    operation_id = "CreateThreat",
    summary = "Добавление угрозы",
    responses(
        (status = 200, description = "Угроза успешно добавлена", body = String)
    )
)
]
// POST http://localhost:4000/v1/menace
pub async fn create_threat(State(_app_state): State<AppState>) -> &'static str {
    "Menace root"
}
