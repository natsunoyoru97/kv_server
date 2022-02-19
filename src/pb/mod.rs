pub mod abi;

use std::convert::TryFrom;

use abi::{command_request::RequestData, *};
use bytes::Bytes;
use http::StatusCode;
use prost::Message;

use crate::KvError;

impl CommandRequest {
    /// Create HSET 
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset { 
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    /// Create HMSET
    pub fn new_hmset(table: impl Into<String>, pairs: Vec<Kvpair>) -> Self {
        Self {
            request_data: Some(RequestData::Hmset(Hmset {
                table: table.into(),
                pairs: pairs,
            })),
        }
    }

    /// Create HGET 
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    /// Create HMGET
    pub fn new_hmget(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hmget(Hmget {
                table: table.into(),
                keys: keys,
            })),
        }
    }

    /// Create HGETALL 
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            }))
        }
    }

    /// Create HDEL
    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            }))
        }
    }

    /// Create HMDEL
    pub fn new_hmdel(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hmdel(Hmdel {
                table: table.into(),
                keys: keys,
            }))
        }
    }

    /// Create HEXISTS
    pub fn new_hexists(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hexists(Hexists {
                table: table.into(),
                key: key.into(),
            }))
        }
    }

    /// Create HMEXISTS
    pub fn new_hmexists(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self { 
            request_data: Some(RequestData::Hmexists(Hmexists {
                table: table.into(),
                keys: keys,
            }))
        }
    }
}

impl Kvpair {
    /// 创建一个新的 kv pair
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// (String, Value) -> Kvpair
impl From<(String, Value)> for Kvpair {
    fn from(t: (String, Value)) -> Self {
        Self {
            key: t.0.into(),
            value: Some(t.1),
        }
    }
}

/// String -> Value
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

/// &str -> Value
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.to_owned())),
        }
    }
}

/// i64 -> Value
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i)),
        }
    }
}

/// bool -> Value
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(b)),
        }
    }
}

/// 从 Value 转换成 CommandResponse
impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(v: Vec<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: v,
            ..Default::default()
        }
    }
}

/// 从 Vec<Kvpair> 转换成 CommandResponse
impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

/// Bool -> CommandResponse
impl From<bool> for CommandResponse {
    fn from(v: bool) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v.into()],
            ..Default::default()
        }
    }
}

/// 从 KvError 转换成 CommandResponse
impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {},
        }

        result
    }
}