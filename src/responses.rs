use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Resp<T> {
  pub jsonrpc: String,
  pub id: u32,
  pub result: T
}
