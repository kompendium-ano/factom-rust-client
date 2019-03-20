#![allow(dead_code, non_camel_case_types)]
use std::collections::HashMap;
use http::header::HeaderValue;
use serde_json::{Value, json};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
pub use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request, Body, Client, Uri};

mod tests;
pub mod api;

const HOST: &str = "localhost";
const WALLET_HOST: &str = "localhost";
const DAEMON_PORT: u16 = 8088;
const WALLET_PORT: u16 = 8089;
const DEFAULT_SCHEME: &str = "http";
const API_VERSION: u8 = 2;
const JSONRPC : &str = "2.0";
const ID: u32 = 0;

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
    // check for error
    pub fn error(self)-> bool {
        match self.result {
            Outcome::error(_) => true,
            Outcome::result(_) => false
        }
    }
    //inverse method
    pub fn success(self)-> bool {
        match self.result {
            Outcome::error(_) => false,
            Outcome::result(_) => true
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

    fn to_json(&self)-> String{
        serde_json::to_string(&self).expect("error parsing json")
    }

}

#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
#[derive(Clone)]
pub struct Factom{
    scheme: &'static str,
    host: &'static str,
    wallet_host: &'static str,
    port: u16,
    wallet_port: u16,
    api_version: u8,
    json_rpc_version: &'static str,
    uri: Uri,
    wallet_uri: Uri
}

impl Factom {
    pub fn new()->Factom{
        let mut tmp_struct = Factom {
            scheme: DEFAULT_SCHEME,
            host: HOST,
            wallet_host: WALLET_HOST,
            port: DAEMON_PORT,
            wallet_port: WALLET_PORT,
            api_version: API_VERSION,
            json_rpc_version: JSONRPC,
            uri: Uri::default(),
            wallet_uri: Uri::default()
        };
        tmp_struct.build()
    }

    pub fn build(&mut self)-> Self{
        self.uri = self.build_uri(self.port);
        self.wallet_uri = self.build_uri(self.wallet_port);
        self.clone()
    }

    pub fn https(&mut self)-> Self{
        self.scheme = "https";
        self.clone()
    }

    pub fn host(&mut self, host: &'static str)-> Self{ 
        self.host = host;
        self.clone()
    }

    pub fn port(&mut self, port: u16)-> Self{
        self.port = port;
        self.clone()
    }
    pub fn api_version(&mut self, version: u8)-> Self{
        self.api_version = version;
        self.clone()
    }

    pub fn json_rpc_version(&mut self, version: &'static str)-> Self{
        self.json_rpc_version = version;
        self.clone()
    }

    fn build_uri(&self, port: u16)-> Uri{
        let authority = [self.host, ":", &port.to_string()].concat();
        let path = ["/v", &self.api_version.to_string()].concat();
        Uri::builder()
            .scheme(self.scheme)
            .authority(authority.as_str())
            .path_and_query(path.as_str())
            .build()
            .expect("Unable to build URI from Factomd struct")
    }

    fn api_call(self, method: &str, params: HashMap<String, Value>,)
                        ->  impl Future<Item=Response, Error=FetchError> {
            let uri = self.uri.clone();
            self.inner_api_call(method, params, uri)
    }

    fn walletd_api_call(self, method: &str, params: HashMap<String, Value>,)
                        ->  impl Future<Item=Response, Error=FetchError>{
            let uri = self.wallet_uri.clone();
            self.inner_api_call(method, params, uri)
    }

    fn inner_api_call(self, method: &str, params: HashMap<String, Value>, uri: Uri)
                        ->  impl Future<Item=Response, Error=FetchError> {
        let json_str = ApiRequest::method(method)
                                    .parameters(params)
                                    .to_json();
        let mut req = Request::new(Body::from(json_str));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri;
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

// Retrieves future synchronously, blocks until Result is returned
pub fn fetch<F, R, E>(fut: F)-> Result<R, E>
    where
        F: Send + 'static + Future<Item = R, Error = E>,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a tokio runtime");
        runtime.block_on(fut)
    }




