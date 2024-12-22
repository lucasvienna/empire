// while in development, ignore dead code and unused variables warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::db::conn::get_connection_pool;
use crate::db::migrations::run_migrations;
use crate::net::server;
use crate::net::server::AppState;
use anyhow::Result;
use empire::setup_tracing;
use std::env;
use std::sync::Arc;
use tokio::signal;
use tracing::info;

mod controllers;
mod db;
mod game;
mod models;
mod net;
mod rpc;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    match env::var("RUST_LOG").ok() {
        Some(v) => {
            if !v.contains("diesel") {
                env::set_var("RUST_LOG", format!("{},diesel=debug", v));
            }
        }
        None => env::set_var("RUST_LOG", "empire=trace,diesel=debug"),
    }

    setup_tracing()?;
    info!("Starting Empire server...");

    let pool = get_connection_pool();
    {
        let mut conn = pool.get()?;
        run_migrations(&mut conn).expect("Should execute pending migrations");
    }

    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    let (listener, router) = server::init().await?;
    let router = router.with_state(app_state);

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
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
