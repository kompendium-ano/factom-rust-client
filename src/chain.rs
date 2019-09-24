use super::*;

impl Factom {
/**
Return the keymr of the head of the chain for a chain ID (the unique hash 
created when the chain was created).
# Example
```
use factom::*;

let chainid = "9dec48601fba6ddb4bcea12066ba0f2b2467f89c788c5a243eb253c3de0f815b";
let factom = Factom::new();
let query = factom
            .chain_head(chainid)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn chain_head(self, chainid: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("chainid".to_string(), json!(chainid));
    self.call("chain-head", params)
  }

/**
Send a Chain Commit Message to factomd to create a new Chain. 
The commit chain hex encoded string is documented here: 
[Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry-commit)

The commit-chain API takes a specifically formated message encoded in hex that 
includes signatures. If you have a factom-walletd instance running, you can 
construct this commit-chain API call with compose-chain which takes easier 
to construct arguments.

The compose-chain api call has two api calls in it’s response: 
commit-chain and reveal-chain. To successfully create a chain, the reveal-chain 
must be called after the commit-chain.

Notes:
It is possible to be unable to send a commit, if the commit already exists
 (if you try to send it twice). This is a mechanism to prevent you from double 
 spending. If you encounter this error, just skip to the reveal-chain. The 
 error format can be found here: repeated-commit

*/
  pub fn commit_chain(self, message: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("message".to_string(), json!(message));
    self.call("commit-chain", params)
  }

/**
Reveal the First Entry in a Chain to factomd after the Commit to complete the 
Chain creation. The reveal-chain hex encoded string is documented here: 
[Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)

The reveal-chain API takes a specifically formatted message encoded in hex that 
includes signatures. If you have a factom-walletd instance running, you can 
construct this reveal-chain API call with compose-chain which takes easier to 
construct arguments.

The compose-chain api call has two api calls in its response: commit-chain and 
reveal-chain. To successfully create a chain, the reveal-chain must be called 
after the commit-chain.

*/
  pub fn reveal_chain(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("entry".to_string(), json!(entry));
    self.call("reveal-chain", params)
  }
}