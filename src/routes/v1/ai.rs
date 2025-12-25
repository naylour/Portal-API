use crate::{app::AppState};
use axum::{Router, routing::post};

mod query;

pub fn router() -> Router<AppState> {
    Router::new().nest("/ai", Router::new().route("/query", post(query::query)))
}
