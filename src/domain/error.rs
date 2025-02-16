use std::{error, fmt, io};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
    // Packet Errors
    InvalidPacket,
    InvalidUsername,
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

    // Service Errors
    ConstructBuildingError,
    UpgradeBuildingError,
    ConfirmUpgradeError,
}

#[derive(Debug)]
enum ErrorRepr {
    WithDescription(ErrorKind, &'static str),
    WithDescriptionAndDetail(ErrorKind, &'static str, String),
    IoError(io::Error),
    DbError(diesel::result::Error),
    AnyhowError(anyhow::Error),
}

pub struct Error {
    repr: ErrorRepr,
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

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Error {
        Error {
            repr: ErrorRepr::AnyhowError(err),
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
            ErrorRepr::AnyhowError(ref err) => err.fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
