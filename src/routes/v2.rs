use axum::Router;
use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/v2", Router::new())
}
