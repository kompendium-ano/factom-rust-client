//!  Request handling functions intrinsic to the factom struct
use super::*; 
use constants::*;
use serde_json::Value;
use std::collections::HashMap;
use std::num::Wrapping;
use bytes::buf::BufExt as _;
use crate::responses::ApiResponse;
use serde::{Serialize, de::DeserializeOwned};
use hyper::{Body, body, Request, client::ResponseFuture};
use http::{Uri, request::Builder, header::CONTENT_TYPE};

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

  /// Builds the basis of a request minus the json body
  pub fn builder(uri: &Uri) -> Builder {
    let req = Request::builder()
                  .method("POST")
                  .header(CONTENT_TYPE, "application/json")
                  .uri(uri);
    req
  }
}

/// Makes a request to the current factomd node
pub async fn factomd_call(api: &Factom, req: ApiRequest) -> ResponseFuture {
  inner_call(api, &api.factomd_uri, req).await
}

/// Makes a request to the current walletd node
pub async fn walletd_call(api: &Factom, req: ApiRequest) -> ResponseFuture {
  inner_call(api, &api.walletd_uri, req).await
}

/// Makes a request to the current factomd node using the debug path 
pub async fn debug_call(api: &Factom, req: ApiRequest) -> ResponseFuture {
  inner_call(api, &api.debug_uri, req).await
}

async fn inner_call(
  api: &Factom,
  uri: &Rc<Uri>,
  req: ApiRequest
) -> ResponseFuture 
{
  let json = Body::from(req.json());
  let builder = ApiRequest::builder(&uri);
  let payload = builder.body(json)
                        .expect("Constructing request body");
  api.client.request(payload)
}

/// Parses the response and deserialises the API call into an appropriate 
/// ApiResponse struct
pub async fn parse<T>(fut: ResponseFuture) -> Result<ApiResponse<T>> 
  where T: DeserializeOwned + Default
{
  let res = fut.await?;
  let body = body::aggregate(res.into_body())
                  .await
                  .expect("Parsing response body");
  let res: ApiResponse<T> =serde_json::from_reader(body.reader())
                            .expect("Deserialising JSON");
  Ok(res)
}

/// Fetch is a convenience function that will run a future to it's completion,
/// the function will create a new runtime for every call, if making multiple
/// api calls for synchronous usage it's recommended to create 
/// a single runtime and re-use it's blocking method instead
#[cfg(not(feature="no-runtime"))]
pub fn fetch<F: Future>(query: F) -> F::Output {
  let mut rt = Runtime::new().expect("Initialising Runtime");
  rt.block_on(query)
}
