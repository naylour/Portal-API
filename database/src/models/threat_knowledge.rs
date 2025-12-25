use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct ThreatKnowledge {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,

    #[schema(value_type = String, format = "uuid")]
    pub threat_id: Uuid,

    pub summary: String,

    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

impl ThreatKnowledge {}
