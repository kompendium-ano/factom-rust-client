use super::*;

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
  pub error: Error
}

// Generic Factom API Error struct
#[derive(Deserialize, PartialEq, Default, Debug)]
pub struct Error {
  code: i16,
  message: String
}