use envconfig::Envconfig;
use std::process;

#[derive(Envconfig, Debug)]
pub struct DatabaseConfig {
    #[envconfig(from = "DATABASE_NAME", default = "postgres")]
    pub name: String,

    #[envconfig(from = "DATABASE_PORT", default = "5432")]
    pub port: String,

    #[envconfig(from = "DATABASE_USER", default = "postgres")]
    pub user: String,

    #[envconfig(from = "DATABASE_PASSWORD", default = "postgres")]
    pub password: String,

    #[envconfig(from = "DATABASE_URL")]
    pub url: String,
}

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(nested = true)]
    pub database: DatabaseConfig,

    #[envconfig(from = "API_PORT", default = "3000")]
    pub api_port: String,
}

impl Config {
    pub fn load() -> Self {
        Config::init_from_env().unwrap_or_else(|error| {
            eprintln!("Отсутствуют необходимы переменные окружения: {}", error);
            process::exit(1);
        })
    }
}
