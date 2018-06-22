
#![allow(dead_code)]
// extern crate serde-json;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use hyper::{Client, Uri};
use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request, Body};
use http::header::HeaderValue;
use hyper_tls::HttpsConnector;
use serde_json::Value;

const JSONRPC : &str  = "2.0";

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    jsonrpc: String,
    id: u32,
    result: HashMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiRequest{
    jsonrpc: String,
    id: u32,
    method: String,
    parameters: HashMap<String, Value>
}

fn api_call(json_str: String)-> impl Future<Item=(), Error=()>{
    let uri: hyper::Uri = "https://api.factomd.net/v2".parse().unwrap();
    let json = r#"{
                    "jsonrpc": "2.0",
                    "id": 0,
                    "method": "heights",
                    "parameters": {}
                }"#;
    let mut req = Request::new(Body::from(json_str));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
        );

    // https connector
    let https = HttpsConnector::new(4).expect("TLS initialization failed");

    let client = Client::builder().build::<_, hyper::Body>(https);
    let result = client
                    .request(req)
                    .and_then(|res| {res.into_body().concat2()})
                    .from_err::<FetchError>()
                    .and_then(|json| {
                                    dbg!(&json);
                                    let output: Response = serde_json::from_slice(&json)?;
                                    Ok(output)
                                    });
    result.map(|posted| posted)
        .map_err(|err| err)
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

struct Factomd {
    scheme: String,
    host: String,
    port: u32,
    version: String,
    full_uri: Uri,
    json_rpc_version: u8
}

impl Factomd {

    fn new()->Factomd{
        Factomd {
            scheme: "http".to_string(),
            host: "api.factomd.net/v2".to_string(),
            port: 8088,
            version: "2".to_string(),
            full_uri: Uri::default(),
            json_rpc_version: 2
        }
    }

    fn chain_head(keymr: &str)-> &str{
        let method = "chain-head";
        method
    }

    fn set_json_rpc_version(mut self, version: u8)-> Factomd{
        self.json_rpc_version = version;
        self
    }

    fn port(mut self, port: u32)-> Factomd{
        self.port = port;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api_request() {
        let body = ApiRequest{
            jsonrpc: JSONRPC.to_string(),
            id: 0,
            method: "heights".to_string(),
            parameters: HashMap::new()
        };
        let payload = serde_json::to_string(&body).expect("error parsing json");
        let request = api_call(payload);
        rt::run(request);
    }

    #[test]
    fn jsontest() {
        let payload1 = b"{\"result\":{\"entryblockheight\":123456,\"leaderheight\":183429,\"directoryblockheight\":183429,\"entryheight\":183429},\"jsonrpc\":\"2.0\",\"id\":0}";
        let payload2 = b"{\"result\":{\"entryblockheight\":\"shouldfail\",\"leaderheight\":183429,\"directoryblockheight\":183429,\"entryheight\":183429},\"jsonrpc\":\"2.0\",\"id\":0}";
        
        let result1: Response = serde_json::from_slice(payload2).expect("Error parsing json");
        let result2: Value = serde_json::from_slice(payload2).expect("Error parsing json");

        dbg!(result2);
    }
    
}
