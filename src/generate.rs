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

}