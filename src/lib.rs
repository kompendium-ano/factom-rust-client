#![allow(dead_code)]
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use hyper::{Client, Uri};
use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request, Body};
use http::header::HeaderValue;
use hyper_tls::HttpsConnector;
use serde_json::Value;

const JSONRPC : &str = "2.0";
const ID: u32 = 0;
const DAEMON_PORT: u16 = 8088;
const WALLET_PORT: u16 = 8089;
const API_VERSION: u8 = 2;

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

impl ApiRequest {
    fn method(method: &str)-> ApiRequest{
        ApiRequest{
            jsonrpc: JSONRPC.to_string(),
            id: ID,
            method: method.to_string(),
            parameters: HashMap::new()
        }
    }

    fn parameters(&mut self, parameters: HashMap<String, Value>){
        self.parameters = parameters;
    }

    fn to_json(&self)-> String{
        serde_json::to_string(&self).expect("error parsing json")
    }

}


fn api_call(json_str: String, uri: Uri )->  impl Future<Item=Response, Error=FetchError> {
    // let uri: hyper::Uri = "https://api.factomd.net/v2".parse().unwrap();
    // let uri: hyper::Uri = "http://jsonplaceholder.typicode.com/todos/1".parse().unwrap();

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
    client
        .request(req)
        .and_then(|res| {res.into_body().concat2()})
        .from_err::<FetchError>()
        .and_then(|json| {
                        dbg!(&json);
                        let output: Response = serde_json::from_slice(&json)?;
                        Ok(output)
                        })

}

// Define a type so we can return multiple types of errors
#[derive(Debug)]
enum FetchError {
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

struct Factomd {
    scheme: &'static str,
    host: &'static str,
    port: u16,
    api_version: u8,
    json_rpc_version: &'static str
}

impl Factomd {
    fn new()->Factomd{
        Factomd {
            scheme: "http",
            host: "api.factomd.net",
            port: DAEMON_PORT,
            api_version: API_VERSION,
            json_rpc_version: JSONRPC
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
        dbg!(&authority);
        dbg!(&path);
        Uri::builder()
            .scheme(self.scheme)
            .authority(authority.as_str())
            .path_and_query(path.as_str())
            .build()
            .expect("Error building URI from Factomd struct")
    }

}



#[cfg(test)]
mod tests {
    use super::*;   
    #[test]
    fn heights() {
        let mut factomd = Factomd::new();
        factomd.port(443);
        factomd.https();
        let uri = factomd.uri();
        dbg!(&uri);
        let request = ApiRequest::method("heights").to_json();
        let response = api_call(request, uri)
                            .map(|result| {dbg!(result.result);})
                            .map_err(|err| {println!("{:?}", err);});
        rt::run(response);
    }



    
}

