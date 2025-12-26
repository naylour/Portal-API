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
    pub files_urls: Vec<String>,

    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct ThreatSourceCreateDTO {
    pub name: String,
    pub url: Option<String>,
    pub files_urls: Vec<String>,
}

impl ThreatSource {
    pub async fn create(pool: &PgPool, data: ThreatSourceCreateDTO) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as::<_, Self>(
            r"
            INSERT INTO threat_source (name, url, files_urls)
            VALUES ($1, $2, $3)
            RETURNING id, name, url, files_urls, updated_at, created_at;
        ",
        )
        .bind(data.name)
        .bind(data.url)
        .bind(data.files_urls)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as::<_, Self>(
            r"
            SELECT id, name, url, files_urls, updated_at, created_at
            FROM threat_source
            WHERE id = $1;
        ",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
