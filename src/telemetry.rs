use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::{env, io};

use logs_wheel::LogFileInitializer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

/// A static global variable, `TRACING_DIRECTIVES`, used to initialize and store
/// log directives for configuring the Rust `tracing` system's behavior.
///
/// This variable is lazily initialized using `LazyLock` to ensure that its value
/// is computed only once during runtime, when first accessed. The value is typically
/// used to configure the log level and log targets for the application and its
/// dependencies.
static TRACING_DIRECTIVES: LazyLock<String> = LazyLock::new(|| {
	dotenvy::dotenv().ok();

	match env::var("RUST_LOG").ok() {
		Some(mut directives) => {
			if !directives.contains("diesel") {
				directives.push_str(",diesel=debug");
			}
			if !directives.contains("tower_http") {
				directives.push_str(",tower_http=debug");
			}
			if !directives.contains("axum::rejection") {
				directives.push_str(",axum::rejection=trace");
			}
			directives
		}
		None => "empire=debug,tower_http=debug,diesel=debug".to_string(),
	}
});

/// Retrieves the computed tracing directives.
///
/// The directives are calculated from the `RUST_LOG` environment variable
/// on the first call and cached for subsequent uses. This ensures
/// consistent logging configuration without unsafe environment manipulation.
pub fn get_tracing_directives() -> &'static str {
	&TRACING_DIRECTIVES
}

/// Initializes the application's logging system with both console and file output.
/// Sets up a global tracing subscriber with filtering based on environment variables
/// and writes logs to both stdout and a rotating log file.
///
/// This function also reads the `.env` file and surfaces its contents with `dotenvy`
pub fn init_tracing() -> crate::Result<()> {
	let writer = rotate_log_file(&env::var("LOGFILE").unwrap_or("empire.log".to_string()))?;

	let subscriber = registry()
		.with(EnvFilter::new(get_tracing_directives()))
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_tracing_directives() {
		let directives = get_tracing_directives();
		assert!(directives.contains("diesel=debug"));
		assert!(directives.contains("tower_http=debug"));
	}
}
