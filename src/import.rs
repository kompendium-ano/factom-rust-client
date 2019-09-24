use super::*;

impl Factom {
  /**
Import Factoid and/or Entry Credit address secret keys into the wallet.
# Example
```
use factom::*;

let addresses = vec!("Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK");
let factom = Factom::new();
let query = factom
            .import_addresses(addresses)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn import_addresses(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    let mut secrets: Vec<HashMap<&str, &str>> = Vec::new();
    for address in addresses{
      let mut tmp = HashMap::new();
      tmp.insert("secret", address);
      secrets.push(tmp);
    }
    params.insert("addresses".to_string(), json!(secrets));
    self.walletd_call("import-addresses", params)
  }
}