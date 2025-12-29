use std::{error, fmt, io};

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T, E = Error> = anyhow::Result<T, E>;

/// Represents an application-wide error type with rich error categorization.
///
/// This error type wraps several kinds of errors that may occur in the system.
/// It enables seamless error conversions, categorizing errors into various kinds
/// such as I/O errors, database errors, as well as application-specific errors.
///
/// The `Error` struct carries additional context, such as descriptions and details for specific errors.
pub struct Error {
	repr: ErrorRepr,
}

#[derive(Debug)]
enum ErrorRepr {
	WithDescription(ErrorKind, &'static str),
	WithDescriptionAndDetail(ErrorKind, &'static str, String),
	IoError(io::Error),
	DbError(diesel::result::Error),
	DieselPoolError(diesel::r2d2::Error),
	PoolError(r2d2::Error),
	AnyhowError(anyhow::Error),
	SerdeError(serde_json::Error),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
	InternalError,

	// Packet Errors
	InvalidPacket,
	InvalidUsername,
	InvalidEmail,
	InvalidPassword,
	InvalidToken,
	InvalidMessage,
	InvalidBuilding,
	InvalidLevel,
	InvalidFaction,
	InvalidUpgrade,
	InvalidDestroy,
	InvalidCancel,
	InvalidError,
	InvalidData,
	InvalidSize,
	InvalidIndex,
	InvalidRead,
	InvalidWrite,
	InvalidFlush,
	InvalidCopy,
	InvalidPacketType,
	InvalidPacketBit,
	InvalidPacketByte,
	InvalidPacketBuffer,
	InvalidPacketLogin,
	InvalidPacketLogout,
	InvalidPacketChat,
	InvalidPacketBuild,
	InvalidPacketUpgrade,
	InvalidPacketDestroy,
	InvalidPacketCancel,
	InvalidPacketError,
	InvalidPacketUsername,
	InvalidPacketPassword,
	InvalidPacketToken,
	InvalidPacketMessage,
	InvalidPacketBuilding,
	InvalidPacketLevel,
	InvalidPacketFaction,
	UnreadBytesError,

	// Cache Errors
	CacheError,
	CacheExpiredError,
	CacheMissError,
	CacheWriteError,
	CacheLimitError,

	// Service Errors
	ConstructBuildingError,
	UpgradeBuildingError,
	ConfirmUpgradeError,

	// Training Errors
	StartTrainingError,
	CancelTrainingError,
	CompleteTrainingError,
	TrainingQueueFullError,
	InsufficientResourcesError,
	InvalidBuildingTypeError,
	InvalidQuantityError,

	// Auth errors
	NoSessionError,
	SessionExpiredError,
}

impl Error {
	pub fn new(kind: ErrorKind, desc: &'static str) -> Self {
		Self {
			repr: ErrorRepr::WithDescription(kind, desc),
		}
	}
}

impl Default for Error {
	fn default() -> Self {
		Self {
			repr: ErrorRepr::WithDescription(ErrorKind::InternalError, "Internal error"),
		}
	}
}

impl From<ErrorKind> for StatusCode {
	fn from(value: ErrorKind) -> Self {
		match value {
			ErrorKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,

			// Packet Errors
			ErrorKind::InvalidPacket
			| ErrorKind::InvalidUsername
			| ErrorKind::InvalidEmail
			| ErrorKind::InvalidPassword
			| ErrorKind::InvalidToken
			| ErrorKind::InvalidMessage
			| ErrorKind::InvalidBuilding
			| ErrorKind::InvalidLevel
			| ErrorKind::InvalidFaction
			| ErrorKind::InvalidUpgrade
			| ErrorKind::InvalidDestroy
			| ErrorKind::InvalidCancel
			| ErrorKind::InvalidError
			| ErrorKind::InvalidData
			| ErrorKind::InvalidSize
			| ErrorKind::InvalidIndex
			| ErrorKind::InvalidRead
			| ErrorKind::InvalidWrite
			| ErrorKind::InvalidFlush
			| ErrorKind::InvalidCopy
			| ErrorKind::InvalidPacketType
			| ErrorKind::InvalidPacketBit
			| ErrorKind::InvalidPacketByte
			| ErrorKind::InvalidPacketBuffer
			| ErrorKind::InvalidPacketLogin
			| ErrorKind::InvalidPacketLogout
			| ErrorKind::InvalidPacketChat
			| ErrorKind::InvalidPacketBuild
			| ErrorKind::InvalidPacketUpgrade
			| ErrorKind::InvalidPacketDestroy
			| ErrorKind::InvalidPacketCancel
			| ErrorKind::InvalidPacketError
			| ErrorKind::InvalidPacketUsername
			| ErrorKind::InvalidPacketPassword
			| ErrorKind::InvalidPacketToken
			| ErrorKind::InvalidPacketMessage
			| ErrorKind::InvalidPacketBuilding
			| ErrorKind::InvalidPacketLevel
			| ErrorKind::InvalidPacketFaction
			| ErrorKind::UnreadBytesError => StatusCode::BAD_REQUEST,

			// Cache Errors
			ErrorKind::CacheError
			| ErrorKind::CacheExpiredError
			| ErrorKind::CacheMissError
			| ErrorKind::CacheWriteError
			| ErrorKind::CacheLimitError => StatusCode::NOT_FOUND,

			// Service Errors
			ErrorKind::ConstructBuildingError
			| ErrorKind::UpgradeBuildingError
			| ErrorKind::ConfirmUpgradeError => StatusCode::CONFLICT,

			// Training Errors
			ErrorKind::StartTrainingError
			| ErrorKind::CancelTrainingError
			| ErrorKind::CompleteTrainingError => StatusCode::CONFLICT,
			ErrorKind::TrainingQueueFullError => StatusCode::CONFLICT,
			ErrorKind::InsufficientResourcesError => StatusCode::UNPROCESSABLE_ENTITY,
			ErrorKind::InvalidBuildingTypeError => StatusCode::BAD_REQUEST,
			ErrorKind::InvalidQuantityError => StatusCode::BAD_REQUEST,

			// Auth errors
			ErrorKind::NoSessionError => StatusCode::UNAUTHORIZED,
			ErrorKind::SessionExpiredError => StatusCode::FORBIDDEN,
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error {
			repr: ErrorRepr::IoError(err),
		}
	}
}

impl From<diesel::result::Error> for Error {
	fn from(err: diesel::result::Error) -> Error {
		Error {
			repr: ErrorRepr::DbError(err),
		}
	}
}

impl From<diesel::r2d2::Error> for Error {
	fn from(err: diesel::r2d2::Error) -> Error {
		Error {
			repr: ErrorRepr::DieselPoolError(err),
		}
	}
}

impl From<r2d2::Error> for Error {
	fn from(err: r2d2::Error) -> Error {
		Error {
			repr: ErrorRepr::PoolError(err),
		}
	}
}

impl From<anyhow::Error> for Error {
	fn from(err: anyhow::Error) -> Error {
		Error {
			repr: ErrorRepr::AnyhowError(err),
		}
	}
}

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Error {
		Error {
			repr: ErrorRepr::SerdeError(err),
		}
	}
}

impl From<config::ConfigError> for Error {
	fn from(err: config::ConfigError) -> Error {
		Error {
			repr: ErrorRepr::AnyhowError(err.into()),
		}
	}
}

impl From<(ErrorKind, &'static str)> for Error {
	fn from((kind, desc): (ErrorKind, &'static str)) -> Error {
		Error {
			repr: ErrorRepr::WithDescription(kind, desc),
		}
	}
}

impl From<(ErrorKind, &'static str, String)> for Error {
	fn from((kind, desc, detail): (ErrorKind, &'static str, String)) -> Error {
		Error {
			repr: ErrorRepr::WithDescriptionAndDetail(kind, desc, detail),
		}
	}
}

impl error::Error for Error {
	fn cause(&self) -> Option<&dyn error::Error> {
		match self.repr {
			ErrorRepr::IoError(ref err) => Some(err as &dyn error::Error),
			_ => None,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.repr {
			ErrorRepr::WithDescription(_, desc) => desc.fmt(f),
			ErrorRepr::WithDescriptionAndDetail(_, desc, ref detail) => {
				desc.fmt(f)?;
				f.write_str(": ")?;
				detail.fmt(f)
			}
			ErrorRepr::IoError(ref err) => err.fmt(f),
			ErrorRepr::DbError(ref err) => err.fmt(f),
			ErrorRepr::DieselPoolError(ref err) => err.fmt(f),
			ErrorRepr::PoolError(ref err) => err.fmt(f),
			ErrorRepr::AnyhowError(ref err) => err.fmt(f),
			ErrorRepr::SerdeError(ref err) => err.fmt(f),
		}
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let (status, message) = match self.repr {
			ErrorRepr::WithDescription(kind, desc) => (kind.into(), desc),
			ErrorRepr::WithDescriptionAndDetail(kind, desc, _) => (kind.into(), desc),
			ErrorRepr::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal I/O error"),
			ErrorRepr::DbError(_) | ErrorRepr::DieselPoolError(_) | ErrorRepr::PoolError(_) => {
				(StatusCode::INTERNAL_SERVER_ERROR, "Internal Database error")
			}
			ErrorRepr::AnyhowError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
			ErrorRepr::SerdeError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
		};
		let body = json!({"error": message});
		(status, Json(body)).into_response()
	}
}
