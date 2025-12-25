use std::io;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, Layer, fmt, prelude::*};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let console_layer = fmt::layer()
        .pretty()
        .with_writer(io::stderr)
        .with_filter(EnvFilter::try_new("info")?);

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("app")
        .filename_suffix("log")
        .build("./logs")?;

    let file_layer = fmt::layer()
        .pretty()
        .with_ansi(false)
        .with_writer(file_appender)
        .with_filter(EnvFilter::try_new("debug")?);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::from_default_env())
        .init();

    Ok(())
}
