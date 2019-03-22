#![allow(dead_code, non_camel_case_types)]

/// 
pub mod api;
pub mod errors;
mod tests;

use std::collections::HashMap;
use http::header::HeaderValue;
use serde_json::{Value, json};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
pub use hyper::rt::{Future, Stream};
pub use tokio::runtime::Runtime;
pub use tokio::prelude::*;
pub use errors::{FetchError, FactomError};
use hyper::{Method, Request, Body, Client};

const WALLET_URI: &str = "http://localhost:8088/v2";
const FACTOMD_URI: &str = "http://localhost:8089/v2";
const API_VERSION: u8 = 2;
const JSONRPC : &str = "2.0";
const ID: u32 = 0;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Outcome{
    result(Value),
    error(HashMap<String, Value>)
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response{
    pub jsonrpc: String,
    pub id: u32,
    #[serde(flatten)]
    pub result: Outcome
}

impl Response {
    /**
    Returns a boolean indicating whether the server sucessfully processed the request
    # Example
    ```
    use factom::*;

    let factom = Factom::from_host("192.168.121.131");
    let query = factom.properties()
                        .map(|response| response)
                        .map_err(|err| err);
    let response = fetch(query).unwrap();
    if response.success(){
        // it's working
    };
    ```
    */

    pub fn success(self)-> bool {
        match self.result {
            Outcome::result(_) => true,
            Outcome::error(_) => false
        }
    }

    /**
    Returns a Result type containing either the successful API call result or a FactomError enum
    # Example
    ```
    use factom::*;
    use errors::FactomError;

    let factom = Factom::from_host("192.168.121.131");
    let query = factom.factoid_block("Not_a_valid_keymr")
                        .map(|response| response)
                        .map_err(|err| err);
    let response = fetch(query).unwrap();
    let result = response.get_result();
    assert!(result == Err(FactomError::InvalidParams))
    ```
    */
    pub fn get_result(self)-> Result<Value, FactomError>{
        match self.result {
            Outcome::result(res) => Ok(res),
            Outcome::error(err) => {
                match err["code"].as_i64().unwrap() {
                    -32700 => Err(FactomError::ParseError),
                    -32600 => Err(FactomError::InvalidRequest),
                    -32602 => Err(FactomError::InvalidParams),
                    -32603 => Err(FactomError::InternalError),
                    -32601 => Err(FactomError::MethodNotFound),
                    -32011 => Err(FactomError::RepeatedCommit),
                    _ => Err(FactomError::UndefinedError)
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ApiRequest{
    jsonrpc: String,
    id: u32,
    method: String,
    params: HashMap<String, Value>
}

impl ApiRequest {
    fn method(method: &str)-> ApiRequest{
        ApiRequest{
            jsonrpc: JSONRPC.to_string(),
            id: ID,
            method: method.to_string(),
            params: HashMap::new()
        }
    }

    fn parameters(&mut self, params: HashMap<String, Value>)-> &mut Self{
        self.params = params;
        self
    }

    fn id(&mut self, id: u32)-> &mut Self{
        self.id = id;
        self
    }

    fn to_json(&self)-> String{
        serde_json::to_string(&self).expect("error parsing json")
    }

}


#[derive(Clone, Copy, Default)]
pub struct Factom{
    uri: &'static str,    wallet_uri: &'static str,
    id: u32
}

impl Factom {
    /**
    Creates a Factom struct containing the default paths for both factomd and factom-walletd
    Requests will go to "http://localhost:8088/v2" and "http://localhost:8089/v2" respectively.
    Is used to construct queries which can be passed to a runtime or fetched synchronously.
    # Example
    ```
    use factom::Factom;

    let api = Factom::new();
    ```
    */
    pub fn new()->Factom{
        Factom {
            uri: FACTOMD_URI,
            wallet_uri: WALLET_URI,
            id: ID
        }
    }

    /**
    Constructs a new  Factom struct for a specific host. All other default parmaeters stay the same
    # Example
    ```
    use factom::Factom;

    let api = Factom::from_host("192.168.42.42");
    // factomd uri => "http://192.168.42.42:8088/v2"
    ```
    */
    pub fn from_host(host: &str)->Factom{
        Factom {
            uri: to_static_str(format!("http://{}:8088/v{}", host, API_VERSION)),
            wallet_uri: to_static_str(format!("http://{}:8089/v{}", host, API_VERSION)),
            id: ID
        }
    }

    /**
    Same as from_host but with tls implemented. All other default parmaeters stay the same
    # Example
    ```
    use factom::Factom;

    let api = Factom::from_https_host("https://api.factomd.net/v2");
    // factomd uri => "https://api.factomd.net/v2"
    ```
    */
    pub fn from_https_host(host: &str)->Factom{
        Factom {
            uri: to_static_str(format!("https://{}:8088/v{}", host, API_VERSION)),
            wallet_uri: to_static_str(format!("https://{}:8089/v{}", host, API_VERSION)),
            id: ID
        }
    }

    /**
    Sets the ID parameter used in asynchronous JSON-RPC calls a returns a copy of the Factom struct
    Will default to 0 if not set.
    # Example
    ```
    use factom::Factom;

    let api = Factom::new()

    let query = api.properties()
                    .id(1888)
                    .map(|res| res)
                    .map_err(|rr| err);
    ```
    */
    pub fn set_id(self, id: u32)-> Factom{
        Factom{
            id,
            uri: self.uri,
            wallet_uri: self.wallet_uri
        }
    }   

    fn call(self, method: &str, params: HashMap<String, Value>)
                        ->  impl Future<Item=Response, Error=FetchError> {
            let uri = self.uri;
            self.inner_api_call(method, params, uri)
    }

    fn walletd_call(self, method: &str, params: HashMap<String, Value>)
                        ->  impl Future<Item=Response, Error=FetchError>{
            let uri = self.wallet_uri;
            self.inner_api_call(method, params, uri)
    }

    fn inner_api_call(self, method: &str, params: HashMap<String, Value>, uri: &str)
                                        ->  impl Future<Item=Response, Error=FetchError> {
        let json_str = ApiRequest::method(method)
                                    .parameters(params)
                                    .id(self.id)
                                    .to_json();
        let mut req = Request::new(Body::from(json_str));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.parse().unwrap_or_else(|_| panic!("Unable to parse URI: {}", uri));
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );

        // https connector
        let https = HttpsConnector::new(4).expect("TLS initialization failed");

        let client = Client::builder().build::<_, hyper::Body>(https);
        client
            .request(req)
            .and_then(|res| {res.into_body().concat2()})
            .from_err::<FetchError>()
            .and_then(|json| {
                                let output: Response = serde_json::from_slice(&json)?;
                                Ok(output)
                            })
    }
}

// Retrieves future, blocks until Result is returned
pub fn fetch<F, R, E>(fut: F)-> Result<R, E>
    where
        F: Send + 'static + Future<Item = R, Error = E>,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut runtime = Runtime::new().expect("Unable to create a tokio runtime");
        runtime.block_on(fut)
    }

fn to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


