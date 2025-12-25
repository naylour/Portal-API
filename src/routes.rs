use crate::{app::AppState, doc::ApiDoc};
use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub mod status;
pub mod v1;
pub mod v2;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(v1::router())
        .merge(v2::router())
        .merge(status::api_status())
        .merge(Scalar::with_url("/doc", ApiDoc::openapi()))
}
