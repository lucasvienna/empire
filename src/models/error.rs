use std::{error, fmt, io};

pub type EmpResult<T> = Result<T, EmpError>;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
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
}

#[derive(Debug)]
enum ErrorRepr {
    WithDescription(ErrorKind, &'static str),
    WithDescriptionAndDetail(ErrorKind, &'static str, String),
    IoError(io::Error),
    DbError(diesel::result::Error),
}

pub struct EmpError {
    repr: ErrorRepr,
}

impl From<io::Error> for EmpError {
    fn from(err: io::Error) -> EmpError {
        EmpError {
            repr: ErrorRepr::IoError(err),
        }
    }
}

impl From<(ErrorKind, &'static str)> for EmpError {
    fn from((kind, desc): (ErrorKind, &'static str)) -> EmpError {
        EmpError {
            repr: ErrorRepr::WithDescription(kind, desc),
        }
    }
}

impl From<(ErrorKind, &'static str, String)> for EmpError {
    fn from((kind, desc, detail): (ErrorKind, &'static str, String)) -> EmpError {
        EmpError {
            repr: ErrorRepr::WithDescriptionAndDetail(kind, desc, detail),
        }
    }
}

impl From<diesel::result::Error> for EmpError {
    fn from(err: diesel::result::Error) -> EmpError {
        EmpError {
            repr: ErrorRepr::DbError(err),
        }
    }
}

impl error::Error for EmpError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self.repr {
            ErrorRepr::IoError(ref err) => Some(err as &dyn error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for EmpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.repr {
            ErrorRepr::WithDescription(_, desc) => desc.fmt(f),
            ErrorRepr::WithDescriptionAndDetail(_, desc, ref detail) => {
                desc.fmt(f)?;
                f.write_str(": ")?;
                detail.fmt(f)
            }
            ErrorRepr::IoError(ref err) => err.fmt(f),
            ErrorRepr::DbError(ref err) => err.fmt(f),
        }
    }
}

impl fmt::Debug for EmpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}
