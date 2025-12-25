use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct ThreatType {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,

    pub code: String,
    pub description: Option<String>,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

impl ThreatType {}
