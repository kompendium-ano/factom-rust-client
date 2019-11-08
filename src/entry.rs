use super::*;

impl Factom {
  /**
Send an Entry Commit Message to factom to create a new Entry. The entry commit 
hex encoded string is documented here: 
[Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry-commit)

The commit-entry API takes a specifically formated message encoded in hex that 
includes signatures. If you have a factom-walletd instance running, you can 
construct this commit-entry API call with compose-entry which takes easier 
to construct arguments.

The compose-entry api call has two api calls in it’s response: commit-entry 
and reveal-entry. To successfully create an entry, the reveal-entry must be 
called after the commit-entry.

Notes:
It is possible to be unable to send a commit, if the commit already exists 
(if you try to send it twice). This is a mechanism to prevent you from double 
spending. If you encounter this error, just skip to the reveal-entry. 
The error format can be found here: repeated-commit

*/
  pub async fn commit_entry(
    self, 
    message: &str
  )-> Result<ApiResponse<CommitEntry>>
  {
    let mut req =  ApiRequest::new("commit-entry");
    req.params.insert("message".to_string(), json!(message));
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/**
Get an Entry from factomd specified by the Entry Hash.
# Example
```
use factom::*;

let hash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let factom = Factom::new();
let query = factom.entry(hash)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 
```
*/
  pub async fn entry(
    self, 
    hash: &str
  )-> Result<ApiResponse<Entry>>
  {
    let mut req =  ApiRequest::new("entry");
    req.params.insert("hash".to_string(), json!(hash));
    let response = self.factomd_call(req).await;
    parse(response).await
  }


/**
Retrieve an entry or transaction in raw format, the data is a hex encoded string. 
# Example
```
use factom::*;

let hash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let factom = Factom::new();
let query = factom.raw_data(hash)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());   
```
*/
  pub async fn raw_data(
    self, 
    hash: &str
  )-> Result<ApiResponse<RawData>>
  {
    let mut req =  ApiRequest::new("raw-data");
    req.params.insert("hash".to_string(), json!(hash));
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/**
  Returns an array of the entries that have been submitted but have not been 
  recorded into the blockchain.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom.pending_entries()
            .map(|response| response).map_err(|err| err);
let result = fetch(query);
let response = result.unwrap();
assert!(response.success());  
```
*/
  pub async fn pending_entries(self)
  -> Result<ApiResponse<Vec<PendingEntry>>>{
    let req =  ApiRequest::new("pending-entries");
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/**
Reveal an Entry to factomd after the Commit to complete the Entry creation. 
The reveal-entry hex encoded string is documented here: 
[Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)

The reveal-entry API takes a specifically formatted message encoded in hex that 
includes signatures. If you have a factom-walletd instance running, you can 
construct this reveal-entry API call with compose-entry which takes easier 
to construct arguments.

The compose-entry api call has two api calls in it’s response: commit-entry and 
reveal-entry. To successfully create an entry, the reveal-entry must be called 
after the commit-entry.

*/
  pub async fn reveal_entry(
    self, 
    entry: &str
  )-> Result<ApiResponse<RevealEntry>>
  {
    let mut req =  ApiRequest::new("reveal-entry");
    req.params.insert("entry".to_string(), json!(entry));
    let response = self.factomd_call(req).await;
    parse(response).await
  }

}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitEntry {
    message: String,
    txid: String,
    entryhash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitChain {
    chainid: String,
    content: String,
    extids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawData {
    data: String,
}

/// pending-entries function returns a Vec of PendingEntry 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingEntry {
    entryhash: String,
    chainid: String,
    status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevealEntry {
    message: String,
    entryhash: String,
    chainid: String,
}