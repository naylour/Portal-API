use crate::app::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub mod class;
pub mod source;

pub mod create_threat;
pub mod get_threats;

pub use create_threat::create_threat;
pub use get_threats::get_threats;

pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/threat",
        Router::new()
            .route("/", get(get_threats))
            .route("/", post(create_threat))
            .merge(source::router())
            .merge(class::router()),
    )
}
