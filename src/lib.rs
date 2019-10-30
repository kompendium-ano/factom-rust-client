#![doc(html_logo_url = "https://seeklogo.com/images/F/factom-fct-logo-08400C829C-seeklogo.com.png",
 html_favicon_url = "https://www.factom.com/wp-content/uploads/2019/06/cropped-factom_favicon_azul-05-192x192.png",
 html_playground_url = "https://play.rust-lang.org/")]
#![forbid(unsafe_code)]
#![allow(dead_code, non_camel_case_types)]

//! A rust client for the Factom network API.
//! 
//! Documentation for the api calls are located on the [Factom struct page](struct.Factom.html)
// pub mod address;
// pub mod balance;
// pub mod block;
// pub mod chain;
// pub mod compose;
pub mod constants;
// pub mod debug;
// pub mod entry;
// pub mod errors;
// pub mod generate;
// pub mod identity;
// pub mod import;
// pub mod tx;
pub mod utils;
// pub mod wallet;
pub mod requests;
pub mod responses;
// pub mod nodes;

// pub use futures;
pub use utils::*;
pub use tokio::prelude::*;
pub use tokio::runtime::Runtime;
pub use hyper::rt::{Future, Stream};
// pub use errors::{FetchError, FactomError};
pub use serde_json::{Value, json};
pub use serde::{Serialize, Deserialize};
pub use constants::*; 
pub use requests::Factom;
pub use responses::ApiResponse;
pub use requests::ApiRequest;

// use std::collections::HashMap;
use hyper_tls::HttpsConnector;
use hyper::{Client, client::HttpConnector};

/// Hyper client with custom https connector
pub type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

/// Async return type for API Calls 
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


// /// Handles the JSON result or error
// #[derive(Debug, Deserialize, PartialEq)]
// pub enum Outcome{
//   result(Value),
//   error(HashMap<String, Value>)
// }

// /// JSON responses are deserialized into this struct
// #[derive(Deserialize, Debug, PartialEq)]
// pub struct Response{
  
//   pub jsonrpc: String,
//   pub id: u32,
//   #[serde(flatten)]
//   pub result: Outcome
// }

// impl Response {
//   /**
//   Returns a boolean indicating whether the server successfully processed the request
//   # Example
//   ```
//   use factom::*;

//   let factom = Factom::from_host("192.168.121.133");
//   let query = factom.properties()
//                     .map(|response| response)
//                     .map_err(|err| err);
//   let response = fetch(query).unwrap();
//   if response.success(){
//     // it's working
//   };
//   ```
//   */

//   pub fn success(self)-> bool {
//     match self.result {
//       Outcome::result(_) => true,
//       Outcome::error(_) => false
//     }
//   }

//   // /**
//   // Returns a Result type containing either the successful API call result or a FactomError enum
//   // # Example
//   // ```
//   // use factom::*;
//   // use errors::FactomError;

//   // let factom = Factom::from_host("192.168.121.133");
//   // let query = factom.factoid_block("Not_a_valid_keymr")
//   //                   .map(|response| response)
//   //                   .map_err(|err| err);
//   // let response = fetch(query).unwrap();
//   // let result = response.get_result();
//   // assert!(result == Err(FactomError::InvalidParams))
//   // ```
//   // */
//   // pub fn get_result(self)-> Result<Value, FactomError>{
//   //   match self.result {
//   //     Outcome::result(res) => Ok(res),
//   //     Outcome::error(err) => {
//   //       match err["code"].as_i64().unwrap() {
//   //         -32700 => Err(FactomError::ParseError),
//   //         -32600 => Err(FactomError::InvalidRequest),
//   //         -32602 => Err(FactomError::InvalidParams),
//   //         -32603 => Err(FactomError::InternalError),
//   //         -32601 => Err(FactomError::MethodNotFound),
//   //         -32011 => Err(FactomError::RepeatedCommit),
//   //         _ => Err(FactomError::UndefinedError)
//   //       }
//   //     }
//   //   }
//   // }
// }

// /// Struct is serialized into the JSON body of the request
// #[derive(Serialize, Debug)]
// pub struct ApiRequest{
//   jsonrpc: String,
//   id: u32,
//   method: String,
//   params: HashMap<String, Value>
// }

// impl ApiRequest {
//   fn method(method: &str)-> ApiRequest{
//     ApiRequest{
//       jsonrpc: JSONRPC.to_string(),
//       id: ID,
//       method: method.to_string(),
//       params: HashMap::new()
//     }
//   }

//   fn parameters(&mut self, params: HashMap<String, Value>)-> &mut Self{
//     self.params = params;
//     self
//   }

//   fn id(&mut self, id: u32)-> &mut Self{
//     self.id = id;
//     self
//   }

//   fn to_json(&self)-> String{
//     serde_json::to_string(&self).expect("error parsing json")
//   }

// }

