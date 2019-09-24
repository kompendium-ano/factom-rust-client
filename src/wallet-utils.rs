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

}