use anyhow::Result;
use empire::configuration::get_configuration;
use empire::db::connection;
use empire::db::migrations::run_migrations;
use empire::net::server;
use empire::net::server::AppState;
use empire::{setup_tracing, shutdown_signal};
use std::env;
use std::sync::Arc;
use tracing::info;

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

    let configuration = get_configuration().expect("Failed to read configuration.");

    setup_tracing()?;
    info!("Starting Empire server...");

    let pool = connection::get_pool(configuration.database);
    {
        let mut conn = pool.get()?;
        run_migrations(&mut conn).expect("Should execute pending migrations");
    }

    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    let (listener, router) = server::init(configuration.server).await?;
    let router = router.with_state(app_state);

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
