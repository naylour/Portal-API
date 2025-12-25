use crate::app::AppState;
use axum::{Router, routing::post};

pub mod create_source;

pub use create_source::create_source;

pub fn router() -> Router<AppState> {
    Router::new().nest("/source", Router::new().route("/", post(create_source)))
}
