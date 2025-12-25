use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{self, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct ThreatSource {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,

    pub name: String,
    pub url: Option<String>,
    pub file_url: String,

    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct ThreatSourceCreateDTO {
    pub name: String,
    pub url: Option<String>,
    pub file_url: String,
}

impl ThreatSource {
    pub async fn create(pool: &PgPool, data: ThreatSourceCreateDTO) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as::<_, Self>(
            r"
            INSERT INTO threat_source (name, url, file_url)
            VALUES ($1, $2, $3)
            RETURNING id, name, url, file_url, updated_at, created_at;
        ",
        )
        .bind(data.name)
        .bind(data.url)
        .bind(data.file_url)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
