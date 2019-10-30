use super::*;
use requests::*;
use hyper::client::HttpConnector;
use http::{Uri, request::Builder, header::CONTENT_TYPE};

type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>; 

impl Factom {
  pub fn open_node()->Factom{
    Factom {
      client: new_client(),
      // TODO: Constants should be URI's not &str
      factomd_req: request_builder(OPEN_NODE_URI.parse().unwrap()),
      uri: OPEN_NODE_URI,
      wallet_uri: WALLETD_URI,
      id: ID
    }
  }

  pub fn testnet_open_node()->Factom{
    Factom {
      client: new_client(),
      factomd_req: request_builder(DEV_OPEN_NODE_URI.parse().unwrap()),
      uri: DEV_OPEN_NODE_URI,
      wallet_uri: WALLETD_URI,
      id: ID
    }
  }

  /**
  Creates a Factom struct containing the default paths for both factomd and 
  factom-walletd. Requests will go to "http://localhost:8088/v2" and 
  "http://localhost:8089/v2" respectively. Is used to construct queries which 
  can be passed to a runtime or fetched synchronously.
  # Example
  ```
  use factom::Factom;

  let api = Factom::new();
  ```
  */
  pub fn new()->Factom{
    Factom {
      client: new_client(),
      factomd_req: request_builder(FACTOMD_URI.parse().unwrap()),
      uri: FACTOMD_URI,
      wallet_uri: WALLETD_URI,
      id: ID
    }
  }

  /**
  Constructs a new  Factom struct for a specific host. All other default 
  parameters stay the same
  # Example
  ```
  use factom::Factom;

  let api = Factom::from_host("192.168.42.42");
  // factomd uri => "http://192.168.42.42:8088/v2"
  ```
  */
  pub fn from_host(host: &str)->Factom{
    Factom {
      client: new_client(),
      factomd_req: request_builder(host.parse().unwrap()),
      uri: to_static_str(format!("http://{}:8088/v{}", host, API_VERSION)),
      wallet_uri: to_static_str(format!("http://{}:8089/v{}", host, API_VERSION)),
      id: ID
    }
  }

  /**
  Same as from_host but with tls implemented. All other default parameters 
  stay the same
  # Example
  ```
  use factom::Factom;

  let api = Factom::from_https_host("https://api.factomd.net/v2");
  // factomd uri => "https://api.factomd.net/v2"
  ```
  */
  pub fn from_https_host(host: &str)->Factom{
    Factom {
      client: new_client(),
      factomd_req: request_builder(host.parse().unwrap()),
      uri: to_static_str(format!("https://{}:8088/v{}", host, API_VERSION)),
      wallet_uri: to_static_str(format!("https://{}:8089/v{}", host, API_VERSION)),
      id: ID
    }
  }

  /**
  Sets the ID parameter used in asynchronous JSON-RPC calls a returns a copy 
  of the Factom struct
  Will default to 0 if not set.
  # Example
  ```
  use factom::Factom;

  let api = Factom::new();
  let query = api.properties()
                  .set_id(1888)
                  .map(|res| res)
                  .map_err(|err| err);
  ```
  */
  pub fn set_id(mut self, id: u32){
    self.id = id;
  }   
}
