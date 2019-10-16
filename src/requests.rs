use super::*;

impl Factom {
  pub fn call(self, method: &str, params: HashMap<String, Value>)
            ->  impl Future<Item=Response, Error=FetchError> {
      self.inner_call(method, params, self.uri)
  }

  pub fn walletd_call(self, method: &str, params: HashMap<String, Value>)
            ->  impl Future<Item=Response, Error=FetchError>{
      self.inner_call(method, params, self.wallet_uri)
  }

  pub fn debug_call(self, method: &str, params: HashMap<String, Value>)
            ->  impl Future<Item=Response, Error=FetchError>{
      self.inner_call(method, params, DEBUG_URI)
  }

  fn inner_call(
      self, 
      method: &str, 
      params: HashMap<String, Value>, 
      uri: &str
    ) ->  impl Future<Item=Response, Error=FetchError> 
    {
      let json_str = ApiRequest::method(method)
                                .parameters(params)
                                .id(self.id)
                                .to_json();
      let mut req = Request::new(Body::from(json_str));
      *req.method_mut() = Method::POST;
      *req.uri_mut() = uri.parse()
                          .unwrap_or_else(|_| 
                            panic!("Parsing URI: {}", uri)
                          );
      req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
      );

    // Use a https connector instead of the default
    let https = HttpsConnector::new(4).expect("TLS initialization");

    // Hyper client
    let client = Client::builder().build::<_, hyper::Body>(https);
    
    client.request(req)
          .and_then(|res| 
            {res.into_body().concat2()})
          .from_err::<FetchError>()
          .and_then(|json| 
            {
              let output: Response = serde_json::from_slice(&json)?;
              Ok(output)
            }
          )
  }
}