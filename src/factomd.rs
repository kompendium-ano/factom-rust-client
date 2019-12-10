use super::*;

/// The current-minute API call returns:
/// 
/// * `leaderheight` returns the current block height.
/// 
/// * `directoryblockheight` returns the last saved height.
/// 
/// * `minute` returns the current minute number for the open entry block.
/// 
/// * `currentblockstarttime` returns the start time for the current block.
/// 
/// * `currentminutestarttime` returns the start time for the current minute.
/// 
/// * `currenttime` returns the current nodes understanding of current time.
/// 
/// * `directoryblockinseconds` returns the number of seconds per block.
/// 
/// * `stalldetected` returns if factomd thinks it has stalled.
/// 
/// * `faulttimeout` returns the number of seconds before leader node is faulted for 
/// failing to provide a necessary message.
/// 
/// * `roundtimeout` returns the number of seconds between rounds of an election 
/// during a fault.
/// # Example
/// ```
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let response = factomd::current_minute(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.result.leaderheight > 0);
/// }
/// ```
pub async fn current_minute(api: &Factom)-> Result<ApiResponse<CurrentMinute>> {
  let req =  ApiRequest::new("current-minute");
  let response = factomd_call(api, req).await;
  parse(response).await
} 

///  * Retrieve basic system information along with a description of the node’s 
///  * current perception of the network. This includes the node’s role, the current 
///  * leader block height, block minute, syncing status, authority set, currently 
///  * running elections, and more.
/// 
/// ### General system information
/// 
/// id - the node’s identity chain ID
/// publickey - the current public key for the node
/// role - whether the node is a "Follower", "Leader", or "Audit"
/// leaderheight - the highest known block that the network leaders have completed
/// currentheight - the block that this node is currently processing (if fully 
/// synced, this is the block currently being built)
/// currentminute - the minute that this node is processing now
/// currentminuteduration - seconds that the node has been on this minute
/// previousminuteduration - seconds that the node spent on the previous minute
/// balancehash - the node’s understanding of the blockchain’s permanent balance 
/// hash (updated each block)
/// tempbalancehash - the node’s understanding of the blockchain’s temporary balance 
/// hash (updated each minute)
/// lastblockfromdbstate - whether the highest saved block was created from DBState 
/// messages (i.e. true if created by receiving whole blocks, false if built by 
/// following minutes)
/// 
/// ### syncing
/// 
/// This object describes whether this node is syncing, and if so, what messages 
/// it is looking for**
/// 
/// status - possible results are "Processing", "Syncing DBSigs", or "Syncing EOMs"
/// received - the number of DBSigs or EOMs that have been processed so far (omitted 
/// if processing)
/// expected - the number of DBSigs or EOMs that are expected (omitted if 
/// processing)
/// missing - a list of leader identities that we are missing DBSigs or EOMs from 
/// (omitted if processing)
/// authset
/// 
/// This contains two arrays. The first contains information about each Leader node.
/// 
/// id - the server’s identity chain ID
/// vm - the network VM that the server is assigned to for the current block minute
/// listheight - the height of messages that have been processed by this VM
/// listlength - the number of acknowledged messages (processed or not) for this 
/// VM’s process list
/// nextnil - the index of the highest processed message within a VM’s list of 
/// acknowledged messages
/// The second cotains information about each Audit node.
/// 
/// id - the server’s identity chain ID
/// online - the node’s “liveness” (i.e. whether or not we received a heartbeat 
/// from them for the previous minute)
/// elections
/// 
/// Describes this node’s understanding of what elections are happening on the 
/// network (likely to be inaccurate if this is the node being elected out)**
/// 
/// inprogress - whether or not an election is ongoing
/// vmindex - the VM being elected for (omitted if inprogress is false)
/// fedindex - index of the Federated server that the election is for (omitted if 
/// inprogress is false)
/// fedid - identity chain ID of the Federated server that the election is for 
/// (omitted if inprogress is false)
/// round - the current round of elections (omitted if inprogress is false).
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::open_node();
///   let response = factomd::diagnostics(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.result.leaderheight > 0);
/// }
/// ```
pub async fn diagnostics(api: &Factom) -> Result<ApiResponse<Diagnostics>> {
  let req =  ApiRequest::new("diagnostics");
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Returns the number of Factoshis (Factoids *10^-8) that purchase a single 
/// Entry Credit. The minimum factoid fees are also determined by this rate, along 
/// with how complex the factoid transaction is.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::open_node();
///   let response = factomd::entry_credit_rate(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.result.rate > 0);
/// }
/// ```
pub async fn entry_credit_rate(api: &Factom)-> Result<ApiResponse<EcRate>> {
  let req =  ApiRequest::new("entry-credit-rate");
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Returns various heights that allows you to view the state of the blockchain. 
/// The heights returned provide a lot of information regarding the state of factomd, 
/// but not all are needed by most applications. The heights also indicate the 
/// most recent block, which could not be complete, and still being built. The 
/// heights mean as follows:
/// 
/// * directoryblockheight : The current directory block height of the local 
/// factomd node.
/// * leaderheight : The current block being worked on by the leaders in the network. 
/// This block is not yet complete, but all transactions submitted will go into 
/// this block (depending on network conditions, the transaction may be delayed 
/// into the next block)
/// * entryblockheight : The height at which the factomd node has all the entry 
/// blocks. Directory blocks are obtained first, entry blocks could be lagging 
/// behind the directory block when syncing.
/// * entryheight : The height at which the local factomd node has all the 
/// entries. If you added entries at a block height above this, they will not be 
/// able to be retrieved by the local factomd until it syncs further.
/// 
/// A fully synced node should show the same number for all, (except between 
/// minute 0 and 1, when leaderheight will be 1 block ahead.)
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::open_node();
///   let response = factomd::heights(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.result.leaderheight > 0);
/// }
/// ```
pub async fn heights(api: &Factom)-> Result<ApiResponse<Heights>> {
  let req =  ApiRequest::new("heights");
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Retrieve current properties of the Factom system, including the software and 
/// the API versions.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let response = factomd::properties(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(!response.is_err());
/// }
/// ```
pub async fn properties(api: &Factom)-> Result<ApiResponse<Properties>> {
  let req =  ApiRequest::new("properties");
  let response = factomd_call(api, req).await;
  parse(response).await
}

// Retrieve a receipt providing cryptographically verifiable proof that 
// information was recorded in the factom blockchain. A boolean parameter 
//  "includerawentry" can be used to request that raw entry data be returned 
//  at receipt.entry.raw in the JSON result.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let hash = "0ae2ab2cf543eed52a13a5a405bded712444cc8f8b6724a00602e1c8550a4ec2";
///   let client = Factom::open_node();
///   let response = factomd::receipt(&client, hash, false).await.expect("Api Request");
///   dbg!(&response);
///   let entryblockkeymr = "041c3fed14469a3d0f1a022e3d5321583065e691edb9223605c86766ff881883";
///   assert_eq!(response.result.receipt.entryblockkeymr,  entryblockkeymr);
/// }
/// ```
pub async fn receipt(
  api: &Factom, 
  hash: &str,
  includerawentry: bool
)-> Result<ApiResponse<Receipt>> 
{
  let mut req =  ApiRequest::new("receipt");
  req.params.insert("hash".to_string(), json!(hash));
  if includerawentry {
    req.params.insert("includerawentry".to_string(), json!(true));
  }
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Send a raw hex encoded binary message to the Factom network. This is mostly 
/// just for debugging and testing.
pub async fn send_raw_message(
  api: &Factom, 
  msg: &str
)-> Result<ApiResponse<Receipt>> 
{
  let mut req =  ApiRequest::new("send-raw-message");
  req.params.insert("message".to_string(), json!(msg));
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Converts a string to its hexadecimal representation.
pub fn str_to_hex(utf8: &str) -> String {
  let strs: Vec<String> = utf8.as_bytes()
                              .iter()
                              .map(|b| format!("{:02X}", b))
                              .collect();
  strs.join("")
}

/// current-minute function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMinute {
  pub leaderheight: i64,
  pub directoryblockheight: i64,
  pub minute: i64,
  pub currentblockstarttime: i64,
  pub currentminutestarttime: i64,
  pub currenttime: i64,
  pub directoryblockinseconds: i64,
  pub stalldetected: bool,
  pub faulttimeout: i64,
  pub roundtimeout: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostics {
  pub name: String,
  pub id: String,
  pub publickey: String,
  pub role: String,
  pub leaderheight: i64,
  #[serde(default)]
  pub currentheight: i64,
  pub currentminute: i64,
  pub currentminuteduration: f64,
  pub previousminuteduration: f64,
  pub balancehash: String,
  pub tempbalancehash: String,
  pub lastblockfromdbstate: bool,
  pub syncing: Syncing,
  pub authset: Authset,
  pub elections: Elections,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Syncing {
  pub status: String,
  #[serde(default)]
  pub received: i64,
  #[serde(default)]
  pub expected: i64,
  #[serde(default)]
  pub missing: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authset {
  pub leaders: Vec<Leader>,
  pub audits: Vec<Audit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leader {
  pub id: String,
  pub vm: i64,
  pub listheight: i64,
  pub listlength: i64,
  pub nextnil: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audit {
  pub id: String,
  pub online: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elections {
  pub inprogress: bool,
}

// entry-credit-rate function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcRate {
  pub rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heights {
  pub directoryblockheight: i64,
  pub leaderheight: i64,
  pub entryblockheight: i64,
  pub entryheight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
  pub factomdversion: String,
  pub factomdapiversion: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
  pub receipt: ReceiptInner,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptInner {
  pub entry: Entry,
  pub merklebranch: Vec<Merklebranch>,
  pub entryblockkeymr: String,
  pub directoryblockkeymr: String,
  pub directoryblockheight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  pub entryhash: String,
  #[serde(default)]
  pub raw: String,
  pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Merklebranch {
  pub left: String,
  pub right: String,
  pub top: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn current_minute_test() {
    let client = Factom::open_node();
    let query = current_minute(&client);
    let response = fetch(query).expect("Fetching query");
    assert!(response.result.directoryblockheight > 1)
  }

  #[test]
  fn diagnostics_test() {
    let client = Factom::open_node();
    let query = diagnostics(&client);
    let response = fetch(query).expect("Fetching query");
    assert!(response.result.leaderheight > 1)
  }

  #[test]
  fn entry_credit_rate_test() {
    let client = Factom::open_node();
    let query = entry_credit_rate(&client);
    let response = fetch(query).expect("Fetching query");
    assert!(response.result.rate > 1)
  }

  #[test]
  fn heights_test() {
    let client = Factom::open_node();
    let query = heights(&client);
    let response = fetch(query).expect("Fetching query");
    assert!(response.result.directoryblockheight > 1)
  }
  
  #[test]
  fn properties_test() {
    let client = Factom::open_node();
    let query = properties(&client);
    let response = fetch(query).expect("Fetching query");
    assert!(response.result.factomdversion.len() > 1)
  } 
}