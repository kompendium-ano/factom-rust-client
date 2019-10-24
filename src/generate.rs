use super::*;

impl Factom {
  /**
Create a new Entry Credit Address and store it in the wallet.
# Example
```
use factom::*;
let factom = Factom::new();
let query = factom
            .generate_ec_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn generate_ec_address(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("generate-ec-address", HashMap::new())
  }

/**
Create a new Entry Credit Address and store it in the wallet.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .generate_factoid_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn generate_factoid_address(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("generate-factoid-address", HashMap::new())
  }

/**
Creates a new identity key and adds it to the wallet. New keys are generated 
from the same mnemonic seed used for FCT and EC addresses. If the wallet is 
encrypted, it must be unlocked prior to using this command.

# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .generate_factoid_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn generate_identity_key(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("generate-identity-key", HashMap::new())
  }

}

/// Deserialises from generate-ec-address, generate-fct-address and 
/// generate-identity-key
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Generate {
    public: String,
    secret: String,
}
