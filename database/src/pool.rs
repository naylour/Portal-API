use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing;

pub async fn create_database_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Подключение к базе данных...");

    match PgPoolOptions::new().max_connections(5).connect(url).await {
        Ok(pool) => {
            tracing::info!("Подключение прошло успешно");
            Ok(pool)
        }
        Err(error) => {
            tracing::error!("Ошибка подключения: {:#?}", error);
            Err(error)
        }
    }
}
