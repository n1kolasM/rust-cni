// Copyright (c) 2024 https://github.com/divinerapier/cni-rs
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::io;
use std::result;
use thiserror::Error;

const CODE_INCOMPATIBLE_CNI_VERSION: u64 = 1;
const CODE_UNSUPPORTED_FIELD: u64 = 2;
const CODE_UNKNOWN_CONTAINER: u64 = 3;
const CODE_INVALID_ENVIRONMENT_VARIABLES: u64 = 4;
const CODE_IO_FAILURE: u64 = 5;
const CODE_DECODING_FAILURE: u64 = 6;
const CODE_INVALID_NETWORK_CONFIG: u64 = 7;
const CODE_TRY_AGAIN_LATER: u64 = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginErrorCode {
    IncompatibleCniVersion,
    UnsupportedField,
    UnknownContainer,
    InvalidEnvironmentVariables,
    IoFailure,
    DecodingFailure,
    InvalidNetworkConfig,
    TryAgainLater,
    UnknownReserved(u64),
    Custom(u64),
}

impl From<u64> for PluginErrorCode {
    fn from(value: u64) -> Self {
        match value {
            CODE_INCOMPATIBLE_CNI_VERSION => PluginErrorCode::IncompatibleCniVersion,
            CODE_UNSUPPORTED_FIELD => PluginErrorCode::UnsupportedField,
            CODE_UNKNOWN_CONTAINER => PluginErrorCode::UnknownContainer,
            CODE_INVALID_ENVIRONMENT_VARIABLES => PluginErrorCode::InvalidEnvironmentVariables,
            CODE_IO_FAILURE => PluginErrorCode::IoFailure,
            CODE_DECODING_FAILURE => PluginErrorCode::DecodingFailure,
            CODE_INVALID_NETWORK_CONFIG => PluginErrorCode::InvalidNetworkConfig,
            CODE_TRY_AGAIN_LATER => PluginErrorCode::TryAgainLater,
            reserved if reserved < 100 => PluginErrorCode::UnknownReserved(reserved),
            custom => PluginErrorCode::Custom(custom),
        }
    }
}

impl From<PluginErrorCode> for u64 {
    fn from(value: PluginErrorCode) -> Self {
        match value {
            PluginErrorCode::IncompatibleCniVersion => CODE_INCOMPATIBLE_CNI_VERSION,
            PluginErrorCode::UnsupportedField => CODE_UNSUPPORTED_FIELD,
            PluginErrorCode::UnknownContainer => CODE_UNKNOWN_CONTAINER,
            PluginErrorCode::InvalidEnvironmentVariables => CODE_INVALID_ENVIRONMENT_VARIABLES,
            PluginErrorCode::IoFailure => CODE_IO_FAILURE,
            PluginErrorCode::DecodingFailure => CODE_DECODING_FAILURE,
            PluginErrorCode::InvalidNetworkConfig => CODE_INVALID_NETWORK_CONFIG,
            PluginErrorCode::TryAgainLater => CODE_TRY_AGAIN_LATER,
            PluginErrorCode::UnknownReserved(val) | PluginErrorCode::Custom(val) => val,
        }
    }
}

impl Serialize for PluginErrorCode {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64((*self).into())
    }
}

impl<'de> Deserialize<'de> for PluginErrorCode {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = u64::deserialize(deserializer)?;
        Ok(val.into())
    }
}

impl std::fmt::Display for PluginErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PluginErrorCode::IncompatibleCniVersion => f.write_str("IncompatibleCniVersion"),
            PluginErrorCode::UnsupportedField => f.write_str("UnsupportedField"),
            PluginErrorCode::UnknownContainer => f.write_str("UnknownContainer"),
            PluginErrorCode::InvalidEnvironmentVariables => {
                f.write_str("InvalidEnvironmentVariables")
            }
            PluginErrorCode::IoFailure => f.write_str("IoFailure"),
            PluginErrorCode::DecodingFailure => f.write_str("DecodingFailure"),
            PluginErrorCode::InvalidNetworkConfig => f.write_str("InvalidNetworkConfig"),
            PluginErrorCode::TryAgainLater => f.write_str("TryAgainLater"),
            PluginErrorCode::UnknownReserved(val) => {
                f.write_fmt(format_args!("UnknownReserved({val})"))
            }
            PluginErrorCode::Custom(val) => f.write_fmt(format_args!("Custom({val})")),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Error)]
pub struct PluginError {
    #[serde(rename = "cniVersion")]
    cni_version: String,
    code: PluginErrorCode,
    msg: String,
    details: Option<String>,
}

impl Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.code, self.msg)?;
        if let Some(details) = &self.details {
            write!(f, ": {details}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CNIError {
    #[error("no net configuration with name {0:?} in {1}")]
    NotFound(String, String),
    #[error("no net configurations found in {0}")]
    NoConfigsFound(String),
    #[error("execute CNI error {0}")]
    ExecuteError(String),
    #[error("plugin error")]
    PluginError(PluginError),
    #[error("Invalid Configuration: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[source] Box<io::Error>),
    #[error("Empty key")]
    EmptyKey,
    #[error("{0}")]
    TooLong(String),
    #[error("Invalid checksum")]
    InvalidChecksum(String),
    #[error("Invalid filename")]
    InvalidFilename(String),
    // #[error("Invalid prost data: {0}")]
    // Decode(#[source] Box<prost::DecodeError>),
    #[error("Invalid data: {0}")]
    VarDecode(String),
    #[error("{0}")]
    TableRead(String),
    #[error("Database Closed")]
    DBClosed,
    #[error("{0}")]
    LogRead(String),
}

impl From<io::Error> for CNIError {
    #[inline]
    fn from(e: io::Error) -> CNIError {
        CNIError::Io(Box::new(e))
    }
}

// impl From<prost::DecodeError> for Error {
//     #[inline]
//     fn from(e: prost::DecodeError) -> Error {
//         Error::Decode(Box::new(e))
//     }
// }

pub type Result<T> = result::Result<T, CNIError>;
