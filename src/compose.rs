use super::*;

impl Factom {
/**
This method, compose-chain, will return the appropriate API calls to create a 
chain in factom. You must first call the commit-chain, then the reveal-chain 
API calls. To be safe, wait a few seconds after calling commit.

Note: The firstentry fields are automatically hex encoded for the server to 
process.
# Example
```
use factom::*;

let ec_address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let extids = vec!("Cargo Test", "test harness");
let content = "Here be the content";
let factom = Factom::new();
let query = factom
              .compose_chain(extids, content, ec_address)
              .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn compose_chain(self, extids: Vec<&str>, content: &str, ecpub: &str)
                        -> impl Future<Item=Response, Error=FetchError>{
    
    let mut params = HashMap::new();
    let hex_content = str_to_hex(content);
    let mut hex_extids = Vec::new();
    for extid in extids{
      hex_extids.push(str_to_hex(extid));
    }
    let chain = json!({
      "firstentry": {
        "extids": hex_extids,
        "content": hex_content
      }
    });
    params.insert("chain".to_string(), chain);
    params.insert("ecpub".to_string(), json!(ecpub));
    dbg!(&params);
    self.walletd_call("compose-chain", params)
  }

/**
This method, compose-entry, will return the appropriate API calls to create an 
entry in factom. You must first call the commit-entry, then the reveal-entry 
API calls. To be safe, wait a few seconds after calling commit.

Note: The entry fields are automatically hex encoded for the server to process.
# Example
```
use factom::*;

let chainid = "9dec48601fba6ddb4bcea12066ba0f2b2467f89c788c5a243eb253c3de0f815b";
let ec_address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let extids = vec!("Cargo Test", "test harness");
let content = "Here be the content";
let factom = Factom::new();
let query = factom
      .compose_entry(chainid, extids, content, ec_address)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 
```
*/
  pub fn compose_entry(self, chainid: &str, extids: Vec<&str>, content: &str, ecpub: &str)
                        -> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    let mut hex_extids = Vec::new();
    for extid in extids {
      hex_extids.push(str_to_hex(extid));
    }
    let entry = json!({
    "chainid": chainid,
    "extids": hex_extids,
    "content": str_to_hex(content)
    });
    params.insert("entry".to_string(), entry);
    params.insert("ecpub".to_string(), json!(ecpub));
    self.walletd_call("compose-entry", params)
  }

/**
Compose transaction marshals the transaction into a hex encoded string. The 
string can be inputted into the factomd API factoid-submit to be sent to 
the network.
# Example
```
use factom::*;

```
*/
  pub fn compose_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(tx_name));
    self.walletd_call("compose-transaction", params)
  }

}