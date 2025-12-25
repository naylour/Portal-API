use crate::app::AppState;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use sqlx;

use utoipa;

pub fn api_status() -> Router<AppState> {
    Router::new().nest(
        "/status",
        Router::new()
            .route("/ready", get(ready_handler))
            .route("/live", get(live_handler)),
    )
}

#[
    utoipa::path(
        get,
        path="/status/ready",
        tag="Статус API",
        operation_id = "Ready",
        summary = "Готовность API",
        responses(
            (status = 200, description = "API готов у работе", body = String)
        )
    )
]
pub async fn ready_handler() -> &'static str {
    "ok"
}

#[derive(Serialize, utoipa::ToSchema)]
#[schema(
    title="Ответ ручки Ready",
    description="Возвращает JSON с данными",
    example=json!({
        "status": "ok",
        "db": true
    })
)]
pub struct ReadyResponse {
    #[schema(example = "ok")]
    status: &'static str,

    #[schema(example = true)]
    db: bool,
}

#[
    utoipa::path(
        get,
        path="/status/live",
        tag="Статус API",
        operation_id = "Live",
        summary = "Статус API",
        responses(
            (status = 200, description = "API успешно вернул свой статус", body = ReadyResponse)
        )
    )
]
pub async fn live_handler(State(app_state): State<AppState>) -> (StatusCode, Json<ReadyResponse>) {
    let db_ok = sqlx::query("SELECT 1").execute(&app_state.db).await.is_ok();

    if db_ok {
        (
            StatusCode::OK,
            Json(ReadyResponse {
                status: "ok",
                db: true,
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadyResponse {
                status: "degraded",
                db: false,
            }),
        )
    }
}
