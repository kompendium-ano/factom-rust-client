use super::*;

impl Factom{

  /**
Get the current hight of blocks that have been cached by the wallet while syncing.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .get_height()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn wallet_height(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("get-height", HashMap::new())
  }
  
/**
Retrieve current properties of factom-walletd, including the wallet and wallet 
API versions.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .properties()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn wallet_properties(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("properties", HashMap::new())
  }

/**
 Sign arbitrary data using a secret key stored in the wallet using ed25519 
 signatures. signer can be a human readable Factoid Address (FA), Entry Credit 
 Address (EC), or Identity Key (idpub). data is a base64-encoded string. 
 Returns both the public key component and the signature as base64-encoded 
 strings. Wallet must be unlocked prior to using this command.

Note: For signing large amounts of data it may be advisable to sign a hash of 
the data rather than the data itself.
# Example
```
use factom::*;

let factom = Factom::new();
let signer = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let data = "Here be data";
let query = factom
            .sign_data(
              signer,
              data
            )
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn sign_data(self, signer: &str, data: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = Hashmap::new();
    params.insert("signer".to_string(), json!(signer));
    params.insert("data".to_string(), json!(data));
    self.walletd_call("sign-data", params)
  }
}