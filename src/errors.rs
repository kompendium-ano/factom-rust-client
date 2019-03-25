//! Error handling for networking, json parsing and the API.
//! 
//! Networking errors are handled by [hyper](https://docs.rs/hyper/0.12.25/hyper/)
//!
//! Problems parsing JSON by [serde_json](https://docs.serde.rs/serde_json/error/index.html)
use super::*;
/// Errors relating to network requests or json serialiazation
#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

struct ApiError {
    code: i16,
    message: &'static str
}

/// [Potential API errors](https://docs.factom.com/api?json#errors) returned by the server are enumerated by FactomError 
#[derive(Debug, Deserialize, PartialEq)]
pub enum FactomError {
    ParseError,
    InvalidRequest,
    InvalidParams,
    InternalError,
    MethodNotFound,
    RepeatedCommit,
    UndefinedError
}
