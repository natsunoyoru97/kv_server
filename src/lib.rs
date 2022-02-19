#![warn(missing_docs)]
//! A simple key-value pair server.

mod error;
mod pb;
mod service;
mod storage;

pub use error::KvError;
pub use pb::abi::*;
pub use service::*;
pub use storage::*;