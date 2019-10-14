use super::*;

impl Factom {
/**
Retrieve the public and private parts of a Factoid or Entry Credit address 
stored in the wallet.

# Example
```
use factom::*;
let api = Factom::testnet_open_node();
let my_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let query = factom
            .address(my_address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
*/
  pub fn address(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("address".to_string(), json!(address));
    self.walletd_call("address", params)
  }

  /**
Retrieve all of the Factoid and Entry Credit addresses stored in the wallet.

# Example
```
use factom::*;
let api = Factom::testnet_open_node();
let query = factom
            .all_addresses()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
*/  
  pub fn all_addresses(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("all-addresses", HashMap::new())
  }

/**
Be careful using this function! Ensure that you have backups of important keys 
before removing them. Given a factoid or entry-credit address, this command 
deletes the corresponding key pair from the wallet. Once executed, the user will 
no longer be able to retrieve the private key or make transactions with the 
address from this wallet. If the wallet is encrypted, it must be unlocked prior 
to using this command.

# Example
```
use factom::*;
let api = Factom::testnet_open_node();
let my_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let query = factom
            .remove_address(my_address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```

*/
  pub fn remove_address(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("address".to_string(), json!(address));
    self.walletd_call("remove-address", params)
  }
}