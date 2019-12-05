use super::*;
use url::Url;
use constants::*;

use std::num::Wrapping;
use http::Uri;

/// Main struct from which API requests are built
/// * client holds the hyper http client with a https connector
/// * factomd/walletd/debug hold the request builders to which a json body 
/// is added
/// * uri is the current uri locations
/// * id is the json-rpc id field as a wrapped usize, 
/// it can be incremented without risking overflow
#[derive(Debug)]
pub struct Factom{
  pub client: HttpsClient,
  pub factomd_uri: Rc<Uri>,
  pub walletd_uri: Rc<Uri>,
  pub debug_uri: Rc<Uri>,
  pub id: Wrapping<usize>
}

impl Factom {
  /// Creates a factom struct with the default host locations, equivalent to
  /// Factom::local_node()
  /// * factomd: http://localhost:8088/v2
  /// * walletd: http://localhost:8089/v2
  /// * debug: http://localhost:8088/debug
  pub fn new() -> Factom {
    Factom::local_node()
  }

  /// Creates a factom struct with the default host locations 
  /// * factomd: http://localhost:8088/v2
  /// * walletd: http://localhost:8089/v2
  /// * debug: http://localhost:8088/debug
  pub fn local_node() -> Factom {
    let factomd_uri = parse_uri(FACTOMD_DEFAULT); 
    let walletd_uri = parse_uri(WALLETD_DEFAULT);
    let debug_uri = parse_debug_uri(FACTOMD_DEFAULT);
    Factom{
      client: new_client(),
      factomd_uri,
      walletd_uri,
      debug_uri,
      id: Wrapping(ID)
    }
  }

  /// Creates a factom struct using open node for factomd and a local wallet in
  /// the default location.
  /// * factomd: https://api.factomd.net
  /// * walletd: http://localhost:8089
  /// * debug: https://api.factomd.net/debug
  pub fn open_node()->Factom {
    let factomd_uri = parse_uri(OPENNODE_URI); 
    let walletd_uri = parse_uri(WALLETD_DEFAULT);
    let debug_uri = parse_debug_uri(OPENNODE_URI);
    Factom{
      client: new_client(),
      factomd_uri,
      walletd_uri,
      debug_uri,
      id: Wrapping(ID)
    }
  }

  /// Creates a factom struct using the testnet open node for factomd and a 
  /// local wallet in the default location.
  /// * factomd: https://dev.factomd.net
  /// * walletd: http://localhost:8089
  /// * debug: https://dev.factomd.net/debug
  pub fn testnet_node()->Factom {
    let factomd_uri = parse_uri(DEV_OPENNODE_URI); 
    let walletd_uri = parse_uri(WALLETD_DEFAULT);
    let debug_uri = parse_debug_uri(DEV_OPENNODE_URI);
    Factom{
      client: new_client(),
      factomd_uri,
      walletd_uri,
      debug_uri,
      id: Wrapping(ID)
    }
  }

  /// Creates a factom struct the provided custom hosts. Debug functions will 
  /// use the factomd host.
  /// 
  /// # Example
  /// ```
  /// use factom::Factom::custom_node;
  /// let api = custom_node("https://api.factomd.net", "http://localhost:12345");
  /// ```
  pub fn custom_node(factomd: &str, walletd: &str) -> Factom {
    let factomd_uri = parse_uri(factomd); 
    let walletd_uri = parse_uri(walletd);
    let debug_uri = parse_debug_uri(factomd);
    Factom{
      client: new_client(),
      factomd_uri,
      walletd_uri,
      debug_uri,
      id: Wrapping(ID)
    }
  }

  /// Increments the json-rpc id by one. Will wrap around to zero if it goes 
  /// over [std::usize::MAX](https://doc.rust-lang.org/std/usize/constant.MAX.html)
  pub fn increment_id(mut self) {
    self.id += Wrapping(1);
  }

  /// Sets the json-rpc id
  pub fn set_id(mut self, id: usize) {
    self.id = Wrapping(id);
  }
}

/// Creates a https client, this is placed in the Factom struct and is responsible
/// for making network requests
fn new_client() -> HttpsClient {
  let connector = HttpsConnector::new().expect("TLS initialization");
  let client = Client::builder().build::<_, hyper::Body>(connector);
  Rc::new(client)
}

impl Clone for Factom {
  fn clone(&self) -> Self {
    let client = Rc::clone(&self.client);
    Factom {
      client,
      factomd_uri: Rc::clone(&self.factomd_uri),
      walletd_uri: Rc::clone(&self.walletd_uri),
      debug_uri: Rc::clone(&self.debug_uri),
      id: self.id
    }
  }
}

/// Parses the host and adds the debug path if not already included 
/// Panics with a ParseError if provided with an invalid url 
pub fn parse_debug_uri(host: &str) -> Rc<Uri> {
  inner_parse_uri(host, DEBUG)
}

/// Parses the host and adds the api version path if not already included
/// Panics with a ParseError if provided with an invalid url
/// # Example
/// ```
/// let host = "http://localhost:7077"
/// let factomd_uri = parse_uri(host);
/// assert_eq!(factomd_uri, Uri::from_static("http://localhost:7077/v2"));
/// ```
pub fn parse_uri(host: &str) -> Rc<Uri> {
  inner_parse_uri(host, API_VERSION)
}

fn inner_parse_uri(host: &str, path: &str) -> Rc<Uri> {
  let mut url = Url::parse(host).expect("Parsing Url");
  url.set_path(path);
  let output: Uri = url.into_string()
                        .parse()
                        .expect("Parsing Uri");
  Rc::new(output)
}

// Uri parsing tests, confirm that even when given an incomplete host+path the
// api version or debug path will still be added
#[cfg(test)]
mod tests {
  use super::*; 
  #[test]
  fn inner_uri_parsing() {
    let test_uri = inner_parse_uri("http://host", "testing");
    assert_eq!(test_uri, Rc::new(Uri::from_static("http://host/testing")));
  }

  #[test]
  fn uri_parsing() {
    let test_uri = parse_uri("http://host");
    assert_eq!(test_uri, Rc::new(Uri::from_static("http://host/v2")));
  }

  #[test]
  fn debug_uri_parsing() {
    let test_uri = parse_debug_uri("http://host");
    assert_eq!(test_uri, Rc::new(Uri::from_static("http://host/debug")));
  }
}