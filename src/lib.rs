#![doc(html_logo_url = "https://seeklogo.com/images/F/factom-fct-logo-08400C829C-seeklogo.com.png",
 html_favicon_url = "https://www.factom.com/wp-content/uploads/2019/06/cropped-factom_favicon_azul-05-192x192.png",
 html_playground_url = "https://play.rust-lang.org/")]
#![forbid(unsafe_code)]
#![allow(dead_code, non_camel_case_types)]

//! A rust client for the Factom network API.
//!
//! Documentation for the api calls are located on the [Factom struct page](struct.Factom.html)
pub mod address;
pub mod api;
pub mod balance;
pub mod block;
pub mod chain;
pub mod compose;
pub mod constants;
pub mod debug;
pub mod entry;
// pub mod errors;
pub mod generate;
pub mod identity;
pub mod import;
pub mod tx;
pub mod factomd;
pub mod walletd;
pub mod requests;
pub mod responses;

pub use api::Factom;
pub use factomd::*;
pub use tokio::prelude::*;
pub use tokio::runtime::Runtime;
pub use hyper::rt::{Future, Stream};
pub use serde_json::{Value, json};
pub use serde::{Serialize, Deserialize};
pub use constants::*; 
pub use responses::ApiResponse;
pub use requests::{ApiRequest, parse};

use hyper_tls::HttpsConnector;
use hyper::{Client, client::HttpConnector};

/// Hyper client with custom https connector
pub type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

/// Async return type for API Calls 
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
