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

  /**
Allows a user to add one or more identity keys to the wallet. Using the secret 
keys as input, the command will return the corresponding key pairs that were 
imported. If the wallet is encrypted, it must be unlocked prior to using this 
command.

# Example
```
use factom::*;

let addresses = vec!("idsec2rWrfNTD1x9HPPesA3fz8dmMNZdjmSBULHx8VTXE1J4D9icmAK");
let factom = Factom::new();
let query = factom
            .import_identity_keys(addresses)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn import_identity_keys(self, keys: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    let mut secrets: Vec<HashMap<&str, &str>> = Vec::new();
    for address in keys{
      let mut tmp = HashMap::new();
      tmp.insert("secret", address);
      secrets.push(tmp);
    }
    params.insert("keys".to_string(), json!(secrets));
    self.walletd_call("import-identity-keys", params)
  }

/**
Import a Koinify crowd sale address into the wallet. In our examples we used 
the word “yellow” twelve times, note that in your case the master passphrase 
will be different. If the wallet is encrypted, it must be unlocked prior to 
using this command.

# Example
```
use factom::*;

let koinify_phrase = "yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow";
let factom = Factom::new();
let query = factom
            .import_koinify(koinify_phrase)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn import_koinify(self, phrase: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("words".to_string(), json!(phrase));
    self.walletd_call("import-koinify", params)
  }
}

/// import-addresses function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Addresses {
    addresses: Vec<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Address {
    public: String,
    secret: String,
}

/// import-identity-keys function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Keys {
    keys: Vec<Key>,
}

/// import-koinify function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Key {
    public: String,
    secret: String,
}




