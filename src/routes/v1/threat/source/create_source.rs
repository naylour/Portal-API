use crate::app::{ApiJson, ApiResponse, ApiResult, AppState};
use axum::{extract::State, http::StatusCode};
use tracing;
use utoipa;

use database::models::threat_source::{ThreatSource, ThreatSourceCreateDTO};

#[utoipa::path(
    post,
    path="/v1/threat/source",
    tag="Источники данных",
    operation_id = "CreateThreatSource",
    summary = "Добавление источника данных",
    request_body = ThreatSourceCreateDTO,
    responses(
        (status = 201, description = "Источник успешно добавлен", body = ApiResponse<ThreatSource>),
        // (status = 500, description = "Внутренняя ошибка", body = ApiResponse<()>)
    )
)]
pub async fn create_source(
    State(app_state): State<AppState>,
    ApiJson(data): ApiJson<ThreatSourceCreateDTO>,
) -> ApiResult<ThreatSource> {
    let new_source = ThreatSource::create(&app_state.db, data)
        .await
        .map_err(|e| {
            tracing::error!("Ошибка создания источника: {}", e);
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, Some(e.to_string()), None)
        })?;

    Ok(ApiResponse::success(StatusCode::CREATED, new_source, None))
}
