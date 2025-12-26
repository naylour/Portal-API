use super::threat_status::ThreatStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct Threat {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,

    pub id_from_source: String,

    pub link: Option<String>,

    pub title: String,

    #[schema(value_type = String, format = "uuid")]
    pub source_id: Uuid,

    #[schema(value_type = String, format = "uuid")]
    pub type_id: Uuid,

    #[schema(value_type = String, format = "uuid")]
    pub country_id: Uuid,

    pub description: Option<String>,

    pub status: ThreatStatus,

    pub comment: Option<String>,

    #[schema(value_type = String, format = "date-time")]
    pub adding_in_source: Option<DateTime<Utc>>,

    #[schema(value_type = String, format = "date-time")]
    pub last_update_in_source: Option<DateTime<Utc>>,

    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct ThreatCreateDTO {
    pub title: String,

    pub id_from_source: String,

    pub link: Option<String>,

    pub description: Option<String>,

    pub status: ThreatStatus,

    pub comment: Option<String>,

    #[schema(value_type = String, format = "date-time")]
    pub adding_in_source: Option<DateTime<Utc>>,

    #[schema(value_type = String, format = "date-time")]
    pub last_update_in_source: Option<DateTime<Utc>>,
}
