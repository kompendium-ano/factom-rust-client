use super::*;

impl Factom {
/*!
The public facing API handler containing all method calls
Individual method examples all use a blocking fetch call for demonstration 
purposes here.
The methods return futures which can be run asynchronously in a runtime for 
better performance in a production environment.
Committing or revealing entries require the result of the compose methods and 
will need to wait for those queries to complete.
  
 # Example

 ```
 use factom::*;
 
let factom = Factom::new();
let query = factom.properties()
                  .map(|result| println!("{:?}", result))
                  .map_err(|err| panic!("{:?}", err));
 let result = fetch(query).unwrap();
```
 */

/**
Retrieve administrative blocks for any given height.

The admin block contains data related to the identities within the factom 
system and the decisions the system makes as it builds the block chain. The 
abentries’ (admin block entries) in the JSON response can be of various types, 
the most common is a directory block signature (DBSig). A majority of the 
federated servers sign every directory block, meaning every block after m5 
will contain 5 DBSigs in each admin block.

The ABEntries are detailed 
[here](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#adminid-bytes)

# Example
```
use factom::*;  

let factom = Factom::new();
let query = factom.ablock_by_height(2)
              .map(|response| response)
              .map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
*/
  pub fn ablock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("height".to_string(), json!(height));
    self.call("ablock-by-height", params)
  }

/**
Retrieve a specified admin block given its merkle root key.
# Example
```
use factom::*;

let keymr = "9f9b2d68e7f018a272e9331765ac8d353c7f58c6f18685405b5286353b58daee";
let factom = Factom::new();
let query = factom
              .admin_block(keymr)
              .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/   
  pub fn admin_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("keymr".to_string(), json!(keymr));
    self.call("admin-block", params)
  }

/**
 Retrieve information about the directory block anchors that have been confirmed 
 on Bitcoin and Ethereum.

Parameter options: - "height" - the directory block height (integer) to request 
anchors for - "hash" - the object’s hash (hex string) to request anchors for 
(e.g. entry hash, entry block keymr, factoid block keymr, admin block lookup 
hash, entry credit block header hash, or directory block keymr)
 */
  pub fn anchors(self, target: Anchor)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    match target {
      Anchor::Hash(h) => {params.insert("hash".to_string(), json!(h));},
      Anchor::Height(i) => {params.insert("height".to_string(), json!(i));}
    }
    self.call("anchors", params)
  }

/**
Retrieve a directory block given only its height.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
      .dblock_by_height(2)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn dblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("height".to_string(), json!(height));
    self.call("dblock-by-height", params)
  }

/**
Every directory block has a KeyMR (Key Merkle Root), which can be used to 
retrieve it. The response will contain information that can be used to 
navigate through all transactions (entry and factoid) within that block. The 
header of the directory block will contain information regarding the previous 
directory block’s keyMR, directory block height, and the timestamp. 
# Example
```
use factom::*;

let keymr = "5b372f4622c682c984dc922983d0c769db33c376d107c74e8023446029592011";
let factom = Factom::new();
let query = factom
      .directory_block(keymr)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());    
```
*/
  pub fn directory_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("keymr".to_string(), json!(keymr));
    self.call("directory-block", params)
  }
/**
The directory block head is the last known directory block by factom, or in 
other words, the most recently recorded block. This can be used to grab the 
latest block and the information required to traverse the entire blockchain. 
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
      .directory_block_head()
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn directory_block_head(self)-> impl Future<Item=Response, Error=FetchError>{
    self.call("directory-block-head", HashMap::new())
  }

/**
Retrieve the entry credit block for any given height. These blocks contain 
entry credit transaction information.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
      .ecblock_by_height(2)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn ecblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("height".to_string(), json!(height));
    self.call("ecblock-by-height", params)
  }


/**
Retrieve a specified entry block given its merkle root key. The entry block 
contains 0 to many entries
# Example
```
use factom::*;

let keymr = "1df118c1293858d1111762d6a0df92b12231c72deb14b53bfffc09b867db1f3b";
let factom = Factom::new();
let query = factom
      .entry_block(keymr)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  

```
*/
  pub fn entry_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("keymr".to_string(), json!(keymr));
    self.call("entry-block", params)
  }



/**
Retrieve a specified entrycredit block given its merkle root key. The numbers 
are minute markers.
# Example
```
use factom::*;

let keymr = "9b9e5b67b17f2e2d3d8405ea5fc227f6bf61fcc8c2422b36b11a7fce97018521";
let factom = Factom::new();
let query = factom
      .entry_credit_block(keymr)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn entry_credit_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("keymr".to_string(), json!(keymr));
    self.call("entrycredit-block", params)
  }

/**
Retrieve a specified factoid block given its merkle root key.
# Example
```
use factom::*;

let keymr = "aaaf4db6c1f5b716df0d63dcf9605f599d9e41eb635d8ba3e9ddfbe697ec426c";
let factom = Factom::new();
let query = factom
      .factoid_block(keymr)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn factoid_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("keymr".to_string(), json!(keymr));
    self.call("factoid-block", params)
  }
  
/**
Retrieve the factoid block for any given height. These blocks contain factoid transaction information.
# Example
```
use factom::*;
let factom = Factom::new();
let query = factom
      .fblock_by_height(1)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn fblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("height".to_string(), json!(height));
    self.call("fblock-by-height", params)
  }
}