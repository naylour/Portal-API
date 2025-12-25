use axum::{Router, serve};
use dotenvy::dotenv;
use tracing;
mod logging;

mod app;
mod config;
mod doc;
mod routes;

use app::AppState;
use config::Config;
use database::pool::create_database_pool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    if let Err(err) = logging::init_logging() {
        eprintln!("Ошибка инициализации логирования: {}", err);
        std::process::exit(1);
    }
    tracing::info!("Запуск приложения...");

    let config = Config::load();

    tracing::info!("Получение конфигурации...");

    tracing::info!("{:#?}", config);

    let pool = create_database_pool(&config.database.url).await?;

    let app_state = AppState { db: pool };

    let app = Router::new().merge(routes::router()).with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.api_port))
        .await
        .unwrap();

    tracing::info!("Запуск сервера...");
    serve(listener, app).await.unwrap();

    Ok(())
}
