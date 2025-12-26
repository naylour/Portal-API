use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};
use std::error::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ThreatStatus {
    New,
    InProgress,
    Done,
    Archived,
    Unknown,
}

impl ThreatStatus {
    pub fn from_ru_status(ru_status: &str) -> Self {
        match ru_status {
            "Опубликована" => ThreatStatus::InProgress,
            "Архивная" => ThreatStatus::Archived,
            "Закончена" => ThreatStatus::Done,
            "Новая" => ThreatStatus::New,
            _ => ThreatStatus::Unknown,
        }
    }
}

impl Type<Postgres> for ThreatStatus {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("threat_status") // используем текст вместо enum
    }
}

// Декодирование из базы
impl<'r> Decode<'r, Postgres> for ThreatStatus {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let s: &str = <&str as Decode<Postgres>>::decode(value)?;
        Ok(match s {
            "NEW" => ThreatStatus::New,
            "IN_PROGRESS" => ThreatStatus::InProgress,
            "DONE" => ThreatStatus::Done,
            "ARCHIVED" => ThreatStatus::Archived,
            _ => ThreatStatus::Unknown,
        })
    }
}

// Кодирование в базу
impl<'q> Encode<'q, Postgres> for ThreatStatus {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let s = match self {
            ThreatStatus::New => "NEW",
            ThreatStatus::InProgress => "IN_PROGRESS",
            ThreatStatus::Done => "DONE",
            ThreatStatus::Archived => "ARCHIVED",
            ThreatStatus::Unknown => "UNKNOWN",
        };

        <&str as Encode<'q, Postgres>>::encode_by_ref(&s, buf).map_err(|e| e.into())
    }
}

impl Default for ThreatStatus {
    fn default() -> Self {
        ThreatStatus::New
    }
}
