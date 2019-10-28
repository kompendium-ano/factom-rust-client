use super::*;
use hyper::client::HttpConnector;
use http::request::Builder;

type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

/// Main struct from which API requests are built
// #[derive(Clone)]
pub struct Factom{
  pub client: HttpsClient,
  pub factomd_req: Builder,
  pub uri: &'static str,
  pub wallet_uri: &'static str,
  pub id: u32
}

 
impl Factom {
  pub fn call(self, method: &str, params: HashMap<String, Value>)
            ->  impl Future<Item=Response, Error=FetchError> {
      let uri = self.uri.clone();
      self.inner_call(method, params, uri)
  }

  pub fn walletd_call(self, method: &str, params: HashMap<String, Value>)
            ->  impl Future<Item=Response, Error=FetchError>{
      let uri = self.wallet_uri.clone();
      self.inner_call(method, params, uri)
  }

  pub fn debug_call(self, method: &str, params: HashMap<String, Value>)
            ->  impl Future<Item=Response, Error=FetchError>{
      self.inner_call(method, params, DEBUG_URI)
  }

  fn inner_call(
      mut self, 
      method: &str, 
      params: HashMap<String, Value>, 
      _uri: &str
  ) ->  impl Future<Item=Response, Error=FetchError> 
  {

    let json_str = ApiRequest::method(method)
                              .parameters(params)
                              .id(self.id)
                              .to_json();

  // This should serialise directly into the body
  let req = self.factomd_req.body(Body::from(json_str)).unwrap();

  self.client.request(req)
        .and_then(|res| {res.into_body().concat2()})
        .from_err::<FetchError>()
        .and_then(|json| 
          {
            let output: Response = serde_json::from_slice(&json)?;
            Ok(output)
          }
        )
  }
}



