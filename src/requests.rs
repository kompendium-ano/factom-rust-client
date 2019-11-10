use super::*; 
use constants::*;
use serde_json::Value;
use hyper_tls::HttpsConnector;
use std::collections::HashMap;
use futures_util::TryStreamExt;
use crate::responses::ApiResponse;
use serde::{Serialize, de::DeserializeOwned};
use http::{Uri, request::Builder, header::CONTENT_TYPE};
use hyper::{Request, Chunk, Body, Client, client::ResponseFuture};

pub enum RequestType{
  Factomd,
  Walletd,
  Debug
}

/// Generic request struct is serialized into the JSON body 
#[derive(Serialize, Debug)]
pub struct ApiRequest {
  pub jsonrpc: &'static str,
  pub id: usize,
  pub method: String,
  pub params: HashMap<String, Value>
}

impl ApiRequest {
  /// Creates a new ApiRequest with the specified json-rpc method
  pub fn new(method: &str) -> ApiRequest {
    ApiRequest {
      jsonrpc: JSONRPC,
      id: ID,
      method: method.to_string(),
      params: HashMap::new()
    }
  }


  fn json(self) -> String {
    serde_json::to_string(&self).expect("Serializing json")
  }
}

/// Main struct from which API requests are built
#[derive(Clone)]
pub struct Factom{
  pub client: HttpsClient,
  pub factomd: Builder,
  pub walletd: Builder,
  pub factomd_uri: Uri,
  pub walletd_uri: Uri,
  pub id: usize
}
 
impl Factom {

  pub fn new() -> Factom {
    let factomd_uri: Uri = FACTOMD_DEFAULT.parse().unwrap(); 
    let walletd_uri: Uri = WALLETD_DEFAULT.parse().unwrap();
    Factom{
      client: new_client(),
      factomd: request_builder(factomd_uri.clone()),
      walletd: request_builder(walletd_uri.clone()),
      factomd_uri,
      walletd_uri,
      id: 0
    }
  }

  pub async fn factomd_call(mut self, mut req: ApiRequest) -> ResponseFuture {
    req.id = self.id;
    let req = self.factomd.body(Body::from(req.json())).unwrap();
    self.client.request(req)
  }

  pub async fn walletd_call(mut self, mut req: ApiRequest) -> ResponseFuture {
    req.id = self.id;
    let req = self.walletd.body(Body::from(req.json())).unwrap();
    self.client.request(req)
  }

  pub async fn debug_call(self, req: ApiRequest) -> ResponseFuture {
    // TODO: Replace factomd uri in contructors
    self.inner(RequestType::Debug, req).await
  }

  pub async fn inner(self, req_type: RequestType, mut req: ApiRequest) -> ResponseFuture {
    req.id = self.id;

    let mut node_request = match req_type {
      RequestType::Factomd => self.factomd,
      RequestType::Walletd => self.walletd,
      RequestType::Debug => self.factomd
    };
    let json = Body::from(req.json());
    let req = node_request.body(json)
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
  let body = res.into_body().try_concat().await?;
  dbg!(&body);
  let res: ApiResponse<T> = deser(body).await;
  Ok(res)
}

// Inner deserialisation method for parse_response 
async fn deser<T>(body: Chunk) -> ApiResponse<T> 
  where T: DeserializeOwned + Default
{
  serde_json::from_slice(&body).expect("Deserialising JSON")
}

// Creates a http client, this is placed in the Factom struct and is responsible
// for making network requests
fn new_client() -> HttpsClient {
  let https = HttpsConnector::new().expect("TLS initialization");
  Client::builder().build::<_, hyper::Body>(https)
}

// Builds the basis of a request minus the body, this is kept in the Factom
// struct to avoid rebuilding the request everytime
fn request_builder(uri: Uri) -> Builder {
  let mut req = Request::builder();
  req.method("POST")
      .header(CONTENT_TYPE, "application/json")
      .uri(uri);
  req
}

/// Fetch is a convenience function to run a future to it's completion,  is a 
/// wrapper  around futures::executor::block_on()
pub fn fetch<T, F>(fut: F) ->  F::Output
  where T: DeserializeOwned + Default,
        F: Send + Future
{
  futures::executor::block_on(fut)
}

// #[derive(Deserialize, PartialEq, Default, Debug)]
// pub struct Heights {
//   directoryblockheight: usize,
//   leaderheight: usize,
//   entryblockheight: usize,
//   entryheight: usize
// }

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


