// //! Error handling for networking, json parsing and the API.
// //! 
// //! Networking errors are handled by [hyper](https://docs.rs/hyper/0.12.25/hyper/)
// //!
// //! Problems parsing JSON by [serde_json](https://docs.serde.rs/serde_json/error/index.html)
// use super::*;
use hex::FromHexError;
use std::fmt;
// /// Errors relating to network requests or json serialiazation
// #[derive(Debug)]
// pub enum FetchError {
//   Http(hyper::Error),
//   Json(serde_json::Error),
// }

// impl From<hyper::Error> for FetchError {
//   fn from(err: hyper::Error) -> FetchError {
//     FetchError::Http(err)
//   }
// }

// impl From<serde_json::Error> for FetchError {
//   fn from(err: serde_json::Error) -> FetchError {
//     FetchError::Json(err)
//   }
// }

// struct ApiError {
//   code: i16,
//   message: &'static str
// }

// /// [Potential API errors](https://docs.factom.com/api?json#errors) returned by the server are enumerated by FactomError 
// #[derive(Debug, Deserialize, PartialEq)]
// pub enum FactomError {
//   ParseError,
//   InvalidRequest,
//   InvalidParams,
//   InternalError,
//   MethodNotFound,
//   RepeatedCommit,
//   UndefinedError
// }


/// Errors relating to address conversion and validation
#[derive(Debug)]
pub enum AddressError {
  FromHex(FromHexError),
  Length(String),
  InvalidAddress(String)
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError::FromHex(ref err) => write!(f, "FromHex Error: {:?}", err),
            AddressError::Length(ref err) => write!(f, "Length Error: {:?}", err),
            AddressError::InvalidAddress(ref err) => write!(f, "InvalidAddress Error: {:?}", err),
        }
    }
}

impl From<FromHexError> for AddressError {
  fn from(err: FromHexError) -> AddressError {
    AddressError::FromHex(err)
  }
}

pub type AddressResult<T> = std::result::Result<T, AddressError>;