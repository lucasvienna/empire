use anyhow::Result;
use logs_wheel::LogFileInitializer;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::signal;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

pub mod controllers;
pub mod db;
mod game;
pub mod models;
pub mod net;
mod rpc;
pub mod schema;

pub fn setup_tracing() -> Result<()> {
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
    tracing::subscriber::set_global_default(subscriber).expect("Failed to setup tracing");

    Ok(())
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
