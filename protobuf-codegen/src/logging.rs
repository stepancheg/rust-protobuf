use std::path::PathBuf;

use anyhow::anyhow;

use tracing::{event, Level};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::Customize;

pub(crate) fn init_logging(customize: &Customize) -> anyhow::Result<()> {
    if let Some(log_file) = &customize.log_file {
        let file_name = PathBuf::from(log_file);
        let file_appender = tracing_appender::rolling::never(
            file_name.parent().ok_or_else(|| {
                anyhow!(
                    "Specified log_file did not have a parent directory: {}",
                    file_name.display().to_string()
                )
            })?,
            file_name.file_name().ok_or_else(|| {
                anyhow!(
                    "Specified log_file was invalid: {}",
                    file_name.display().to_string()
                )
            })?,
        );
        let fmt = tracing_subscriber::fmt::layer()
            .with_level(true)
            .with_target(true)
            .with_ansi(false)
            .with_writer(file_appender)
            .with_span_events(FmtSpan::ENTER)
            .with_span_events(FmtSpan::EXIT)
            .json();
        tracing_subscriber::registry()
            .with(fmt)
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .init();
        event!(
            Level::INFO,
            log_file = file_name.display().to_string(),
            "Logging initialized"
        );
    }
    Ok(())
}
