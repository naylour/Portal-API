use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{Utc, DateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Country {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,

    pub code: String,
    pub name: String,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

impl Country {}
