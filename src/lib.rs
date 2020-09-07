#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://seeklogo.com/images/F/factom-fct-logo-08400C829C-seeklogo.com.png",
    html_favicon_url = "https://www.factom.com/wp-content/uploads/2019/06/cropped-factom_favicon_azul-05-192x192.png"
)]

//! A rust client for the Factom network API.
//! The official API docs can be found at: https://docs.factom.com/
//! For more information or support ask on discord: https://discord.gg/mYmcQM2
#[cfg(feature = "default")]
pub extern crate tokio;

pub mod address;
pub mod api;
pub mod balance;
pub mod block;
pub mod chain;
pub mod compose;
pub mod constants;
pub mod debug;
pub mod entry;
pub mod factomd;
pub mod generate;
pub mod identity;
pub mod import;
pub mod requests;
pub mod responses;
pub mod tx;
pub mod utils;
pub mod walletd;

pub use api::Factom;
pub use constants::*;
#[cfg(feature = "default")]
pub use requests::fetch;
pub use requests::ApiRequest;
pub use responses::ApiResponse;

#[cfg(feature = "default")]
pub use tokio::prelude::*;
#[cfg(feature = "default")]
pub use tokio::runtime::Runtime;

#[cfg(feature = "default")]
use futures::prelude::*;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use requests::{debug_call, factomd_call, parse, walletd_call};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::rc::Rc;

/// Reference counted Hyper client with custom https connector
pub type HttpsClient = Rc<Client<HttpsConnector<HttpConnector>, hyper::Body>>;

/// Async return type for API Calls
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
