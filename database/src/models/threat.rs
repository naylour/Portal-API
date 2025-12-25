use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct Threat {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,

    pub title: String,

    #[schema(value_type = String, format = "uuid")]
    pub source_id: Uuid,

    #[schema(value_type = String, format = "uuid")]
    pub type_id: Uuid,

    #[schema(value_type = String, format = "uuid")]
    pub country_id: Uuid,

    pub description: Option<String>,

    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

impl Threat {}
