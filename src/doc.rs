use crate::routes::{status, v1::threat};
use database::models::{
    threat::Threat,
    threat_source::{ThreatSource, ThreatSourceCreateDTO},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        threat::get_threats::get_threats,
        threat::create_threat::create_threat,
        threat::source::create_source::create_source,
        status::ready_handler,
        status::live_handler,
    ),
    components(schemas(Threat, ThreatSource, ThreatSourceCreateDTO))
)]
pub struct ApiDoc;
