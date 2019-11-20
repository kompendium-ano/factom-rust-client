use super::*; 
use constants::*;
use serde_json::Value;
use std::collections::HashMap;
use std::num::Wrapping;
use futures_util::TryStreamExt;
use crate::responses::ApiResponse;
use serde::{Serialize, de::DeserializeOwned};
use hyper::{Chunk, Body, client::ResponseFuture};

/// Generic request struct is serialized into the JSON body 
#[derive(Serialize, Debug, Clone)]
pub struct ApiRequest {
  pub jsonrpc: &'static str,
  pub id: Wrapping<usize>,
  pub method: String,
  pub params: HashMap<String, Value>
}

impl ApiRequest {
  /// Creates a new ApiRequest with the specified json-rpc method
  pub fn new(method: &str) -> ApiRequest {
    ApiRequest {
      jsonrpc: JSONRPC,
      id: Wrapping(ID),
      method: method.to_string(),
      params: HashMap::new()
    }
  }

  /// Serialises the request into a valid json string, serde will panic 
  /// upon failure
  pub fn json(self) -> String {
    serde_json::to_string(&self).expect("Serializing json")
  }
}

enum RequestType{
  Factomd,
  Walletd,
  Debug
}

impl Factom {
  pub async fn factomd_call(self, req: ApiRequest) -> ResponseFuture {
    self.inner_call(RequestType::Factomd, req).await
  }

  pub async fn walletd_call(self, req: ApiRequest) -> ResponseFuture {
    self.inner_call(RequestType::Walletd, req).await
  }

  pub async fn debug_call(self, req: ApiRequest) -> ResponseFuture {
    self.inner_call(RequestType::Debug, req).await
  }

  async fn inner_call(
    self, 
    req_type: RequestType, 
    mut req: ApiRequest
  ) -> ResponseFuture 
  {
    req.id = self.id;
    let node_request = match req_type {
      RequestType::Factomd => self.factomd,
      RequestType::Walletd => self.walletd,
      RequestType::Debug => self.debug
    };
    let json = Body::from(req.json());
    let req = node_request.borrow_mut()
                          .body(json)
                          .expect("Constructing request body");
    self.client.request(req)
  }
}

/// Parses the response and deserialises the API call into an appropriate 
/// ApiResponse struct
pub async fn parse<T>(fut: ResponseFuture) -> Result<ApiResponse<T>> 
  where T: DeserializeOwned + Default
{
  let res = fut.await?;
  let body = res.into_body()
              .try_concat()
              .await
              .expect("Parsing response body");
  let res: ApiResponse<T> = deser(body).await;
  Ok(res)
}

// Inner deserialisation method for parse_response 
async fn deser<T>(body: Chunk) -> ApiResponse<T> 
  where T: DeserializeOwned + Default
{
  serde_json::from_slice(&body).expect("Deserialising JSON")
}

/// Fetch is a convenience function that will run a future to it's completion,
/// the function will create a new runtime for every call, if making multiple
/// api calls for synchronous usage for efficiency it's recommended to create 
/// a single runtime and re-use it's blocking method instead
pub fn fetch<F: Future>(query: F) -> F::Output {
  let rt = Runtime::new().expect("Initialising Runtime");
  rt.block_on(query)
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//     #[test]
//     fn heights() {
//       let rt = Runtime::new().expect("Initialising Runtime");
//       let api = Factom::new();
//       let query = api.heights();
//       let res = rt.block_on(query).expect("Runtime blocking thread");
//       assert!(res.result.directoryblockheight > 1)
//     }

//     #[test]
//     fn method_not_found() {
//       let rt = Runtime::new().expect("Initialising Runtime");
//       let api = Factom::new();
//       let query = api.method_not_found();
//       let res = rt.block_on(query).expect("Runtime blocking thread");
//       assert_eq!(res.error.code, -32601);
//     }
// }


