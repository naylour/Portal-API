pub mod ai;
pub mod country;
pub mod threat;

use crate::app::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/v1",
        Router::new()
            .merge(threat::router())
            .merge(ai::router())
            .merge(country::router()),
    )
}
