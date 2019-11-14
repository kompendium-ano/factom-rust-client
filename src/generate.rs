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
  pub async fn generate_ec_address(self)
    -> Result<ApiResponse<Generate>>
  {
    let req =  ApiRequest::new("generate-ec-address");
    let response = self.walletd_call(req).await;
    parse(response).await
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
  pub async fn generate_factoid_address(self)
    -> Result<ApiResponse<Generate>>
  {
    let req =  ApiRequest::new("generate-factoid-address");
    let response = self.walletd_call(req).await;
    parse(response).await
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
  pub async fn generate_identity_key(self)
    -> Result<ApiResponse<Generate>>
    {
    let req =  ApiRequest::new("generate-identity-key");
    let response = self.walletd_call(req).await;
    parse(response).await
  }

}

/// Deserialises from generate-ec-address, generate-fct-address and 
/// generate-identity-key
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Generate {
    pub public: String,
    pub secret: String,
}
