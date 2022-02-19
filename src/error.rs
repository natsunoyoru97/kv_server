use crate::Value;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
/// Self-defined errors in the kv crate
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    /// The table or the key is not found
    NotFound(String, String),

    #[error("Cannot parse command: `{0}`")]
    /// Cannot parse the command
    InvalidCommand(String),
    #[error("Cannot convert value {:0} to {1}")]
    /// The type conversion between values failed
    ConvertError(Value, &'static str),
    #[error("Cannot process command {0} with table: {1}, key: {2}, Error: {3}")]
    /// Other errors with the table
    StorageError(&'static str, String, String, String),
    
    #[error("Failed to encode protobuf message")]
    /// Error in encoding protobuf
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    /// Error in decoding protobuf
    DecodeError(#[from] prost::DecodeError),

    // TODO: 添加 JSON 处理的 Error

    #[error("Internal error: {0}")]
    /// Any other errors
    Internal(String)
}