use logs_wheel::LogFileInitializer;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

pub fn init_tracing() -> crate::Result<()> {
    let tmp_dir = PathBuf::new().join("log");
    create_dir_all(&tmp_dir)?;
    let log_file = LogFileInitializer {
        max_n_old_files: 2,
        directory: tmp_dir,
        filename: "empire.log",
        preferred_max_file_size_mib: 1,
    }
    .init()?;
    let writer = Mutex::new(log_file);

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish()
        .with(fmt::Layer::default().with_writer(writer).with_ansi(false));
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default.");

    Ok(())
}
