use logs_wheel::LogFileInitializer;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, io};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

pub fn init_tracing() -> crate::Result<()> {
    let writer = rotate_log_file(&env::var("LOGFILE").unwrap_or("empire.log".to_string()))?;

    let subscriber = registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::Layer::new().with_writer(io::stdout))
        .with(fmt::Layer::new().with_writer(writer).with_ansi(false));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default.");

    Ok(())
}

pub fn rotate_log_file(filename: &str) -> crate::Result<Mutex<File>> {
    let tmp_dir = PathBuf::new().join("log");
    create_dir_all(&tmp_dir)?;
    let log_file = LogFileInitializer {
        filename,
        max_n_old_files: 2,
        directory: tmp_dir,
        preferred_max_file_size_mib: 1,
    }
    .init()?;
    Ok(Mutex::new(log_file))
}
