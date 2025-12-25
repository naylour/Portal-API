use crate::app::AppState;
use axum::extract::State;

use utoipa;

#[
    utoipa::path(
    get,
    path="/v1/threat",
    tag="Угрозы",
    operation_id = "GetThreats",
    summary = "Получение угроз",
    responses(
        (status = 200, description = "Угрозы получены успешно", body = String),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        // ("id" = u64, Path, description = "Pet database id to get Pet for"),
    ))
]
// GET http://localhost:4000/v1/menace?page_size=10
pub async fn get_threats(State(_app_state): State<AppState>) -> &'static str {
    "Menace root"
}
