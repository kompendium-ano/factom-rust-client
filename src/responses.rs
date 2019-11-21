use super::*;
use std::{error::Error, default::Default};
use std::fmt::{self, Debug, Display};

/// JSON responses are deserialized into this struct
#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiResponse<T>
  where T: Default
{
  pub jsonrpc: String,
  pub id: u32,
  #[serde(default)]
  pub result: T,
  #[serde(default)]
  pub error: ApiError
}

/// Generic Factom API Error struct
#[derive(Deserialize, PartialEq, Default, Debug)]
pub struct ApiError {
  pub code: i16,
  pub message: String
}

// TODO: Getting stack overflows implementing Display for the 
// struct, needs investigation.

// impl<T> Display for  ApiResponse<T> 
//   where T: Default + Debug + Display
// {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "id: {}\nresult: {}\n error: {}", 
//                   self.id, self.result, self.error)
//   }
// }

// impl Display for  ApiError {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "{}", self)
//   }
// }

// impl<T> Error for ApiResponse<T> 
//   where T: Default + Debug + Display
// {
//   fn description(&self) -> &str {
//     &self.error.message
//   }
// }

impl<T> ApiResponse<T>
  where T: Default
{
  /// Returns a boolean representing whether the api response returned an error
  /// This function does not deal with network errors, that is handled earlier
  /// by the Result from running the future query to completion.
  pub fn is_err(&self) -> bool {
    self.error.code != 0i16
  }
}