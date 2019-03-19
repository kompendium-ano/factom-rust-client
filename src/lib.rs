#![allow(dead_code, non_camel_case_types)]
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use hyper::rt::{Future, Stream};
use hyper::{Method, Request, Body, Client, Uri};
use http::header::HeaderValue;
use hyper_tls::HttpsConnector;
use serde_json::{Value, json};

mod tests;
pub mod api;

const JSONRPC : &str = "2.0";
const ID: u32 = 0;
const DAEMON_PORT: u16 = 8088;
const WALLET_PORT: u16 = 8089;
const API_VERSION: u8 = 2;

#[derive(Debug, Deserialize, PartialEq, Clone)]
enum Outcome{
    result(Value),
    error(HashMap<String, Value>)
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response{
    jsonrpc: String,
    id: u32,
    #[serde(flatten)]
    result: Outcome
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

    fn parameters(self, params: HashMap<String, Value>)-> ApiRequest{
        ApiRequest{
            jsonrpc: self.jsonrpc,
            id: self.id,
            method: self.method,
            params
        }
        
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

struct Walletd{
    scheme: &'static str,
    host: &'static str,
    port: u16,
    api_version: u8,
    json_rpc_version: &'static str,
    uri: Uri
}




impl Walletd {
    fn new()-> Walletd{
        Walletd {
            scheme: "http",
            host: "api.factomd.net",
            port: WALLET_PORT,
            api_version: API_VERSION,
            json_rpc_version: JSONRPC,
            uri: Uri::default()
        }
    }

    pub fn https(&mut self){
        self.scheme = "https";
    }

    fn host(&mut self, host: &'static str){ 
        self.host = host;
    }

    fn port(&mut self, port: u16){
        self.port = port;
    }
    fn api_version(&mut self, version: u8){
        self.api_version = version;
    }

    fn json_rpc_version(&mut self, version: &'static str){
        self.json_rpc_version = version;
    }

    fn uri(self)-> Uri{
        let authority = [self.host, ":", &self.port.to_string()].concat();
        let path = ["/v", &self.api_version.to_string()].concat();
        Uri::builder()
            .scheme(self.scheme)
            .authority(authority.as_str())
            .path_and_query(path.as_str())
            .build()
            .expect("Unable to build URI from Factomd struct")
    }

    fn api_call(self, json_str: String)->  impl Future<Item=Response, Error=FetchError> {
        let mut req = Request::new(Body::from(json_str));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = self.uri();
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


pub struct Factomd {
    scheme: &'static str,
    host: &'static str,
    port: u16,
    api_version: u8,
    json_rpc_version: &'static str,
    uri: Uri
}

struct Factom{
    scheme: &'static str,
    host: &'static str,
    port: u16,
    api_version: u8,
    json_rpc_version: &'static str,
    uri: Uri
}

impl Factomd {
    fn new()->Factomd{
        Factomd {
            scheme: "http",
            host: "api.factomd.net",
            port: DAEMON_PORT,
            api_version: API_VERSION,
            json_rpc_version: JSONRPC,
            uri: Uri::default()
        }
    }

    fn https(&mut self){
        self.scheme = "https";
    }

    fn host(&mut self, host: &'static str){ 
        self.host = host;
    }

    fn port(&mut self, port: u16){
        self.port = port;
    }
    fn api_version(&mut self, version: u8){
        self.api_version = version;
    }

    fn json_rpc_version(&mut self, version: &'static str){
        self.json_rpc_version = version;
    }

    fn uri(self)-> Uri{
        let authority = [self.host, ":", &self.port.to_string()].concat();
        let path = ["/v", &self.api_version.to_string()].concat();
        // dbg!(&authority);
        // dbg!(&Fpath);
        Uri::builder()
            .scheme(self.scheme)
            .authority(authority.as_str())
            .path_and_query(path.as_str())
            .build()
            .expect("Unable to build URI from Factomd struct")
    }

    fn api_call(self, json_str: String)->  impl Future<Item=Response, Error=FetchError> {
        // dbg!(&json_str);
        let mut req = Request::new(Body::from(json_str));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = self.uri();
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
                            // dbg!(&json);
                            let output: Response = serde_json::from_slice(&json)?;
                            Ok(output)
                            })
    }
}




