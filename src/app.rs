use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiResponse<T: Serialize> {
    /// "success" или "error"
    pub status: String,

    /// HTTP-код (дублируется для удобства клиента)
    #[schema(value_type = i32)]
    pub code: u16,

    /// Краткое сообщение (автоматически из StatusCode или кастомное)
    pub message: String,

    /// Детальные причины ошибки (только при error)
    pub reasons: Option<Vec<String>>,

    /// Данные при успехе
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Успешный ответ — message берётся из canonical_reason()
    pub fn success(code: StatusCode, data: T, custom_message: Option<String>) -> Self {
        let message =
            custom_message.unwrap_or_else(|| code.canonical_reason().unwrap_or("Ok").to_string());

        Self {
            status: "success".to_string(),
            code: code.as_u16(),
            message,
            reasons: None,
            data: Some(data),
        }
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<()>>;

impl ApiResponse<()> {
    /// Ответ с ошибкой
    pub fn error(
        code: StatusCode,
        custom_message: Option<String>,
        reasons: Option<Vec<String>>,
    ) -> Self {
        let message = custom_message
            .unwrap_or_else(|| code.canonical_reason().unwrap_or("Error").to_string());

        Self {
            status: "error".to_string(),
            code: code.as_u16(),
            message,
            reasons,
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

pub struct ApiJson<T>(pub T);

impl<S, T> FromRequest<S> for ApiJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(ApiJson(value)),
            Err(err) => Err(ApiResponse::error(
                StatusCode::UNPROCESSABLE_ENTITY,
                Some("Невалидный JSON".into()),
                // Some(err.to_string()),
                None,
            )),
        }
    }
}
