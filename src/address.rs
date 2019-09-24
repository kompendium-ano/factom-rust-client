use super::*;

impl Factom {
/**
Retrieve the public and private parts of a Factoid or Entry Credit address 
stored in the wallet.
*/
  pub fn address(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("address".to_string(), json!(address));
    self.walletd_call("address", params)
  }

  /**
Retrieve all of the Factoid and Entry Credit addresses stored in the wallet.
*/  
  pub fn all_addresses(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("all-addresses", HashMap::new())
  }
}