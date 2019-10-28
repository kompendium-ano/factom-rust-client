//! Basic helper functions
use super::*;

impl Factom {
  /**
The current-minute API call returns:

* `leaderheight` returns the current block height.

* `directoryblockheight` returns the last saved height.

* `minute` returns the current minute number for the open entry block.

* `currentblockstarttime` returns the start time for the current block.

* `currentminutestarttime` returns the start time for the current minute.

* `currenttime` returns the current nodes understanding of current time.

* `directoryblockinseconds` returns the number of seconds per block.

* `stalldetected` returns if factomd thinks it has stalled.

* `faulttimeout` returns the number of seconds before leader node is faulted for 
failing to provide a necessary message.

* `roundtimeout` returns the number of seconds between rounds of an election 
during a fault.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .current_minute()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn current_minute(self)-> impl Future<Item=Response, Error=FetchError>{
    self.call("current-minute", HashMap::new())
  }
  
/**
 * Retrieve basic system information along with a description of the node’s 
 * current perception of the network. This includes the node’s role, the current 
 * leader block height, block minute, syncing status, authority set, currently 
 * running elections, and more.

### General system information

id - the node’s identity chain ID
publickey - the current public key for the node
role - whether the node is a "Follower", "Leader", or "Audit"
leaderheight - the highest known block that the network leaders have completed
currentheight - the block that this node is currently processing (if fully 
synced, this is the block currently being built)
currentminute - the minute that this node is processing now
currentminuteduration - seconds that the node has been on this minute
previousminuteduration - seconds that the node spent on the previous minute
balancehash - the node’s understanding of the blockchain’s permanent balance 
hash (updated each block)
tempbalancehash - the node’s understanding of the blockchain’s temporary balance 
hash (updated each minute)
lastblockfromdbstate - whether the highest saved block was created from DBState 
messages (i.e. true if created by receiving whole blocks, false if built by 
following minutes)

### syncing

This object describes whether this node is syncing, and if so, what messages 
it is looking for**

status - possible results are "Processing", "Syncing DBSigs", or "Syncing EOMs"
received - the number of DBSigs or EOMs that have been processed so far (omitted 
if processing)
expected - the number of DBSigs or EOMs that are expected (omitted if 
processing)
missing - a list of leader identities that we are missing DBSigs or EOMs from 
(omitted if processing)
authset

This contains two arrays. The first contains information about each Leader node.

id - the server’s identity chain ID
vm - the network VM that the server is assigned to for the current block minute
listheight - the height of messages that have been processed by this VM
listlength - the number of acknowledged messages (processed or not) for this 
VM’s process list
nextnil - the index of the highest processed message within a VM’s list of 
acknowledged messages
The second cotains information about each Audit node.

id - the server’s identity chain ID
online - the node’s “liveness” (i.e. whether or not we received a heartbeat 
from them for the previous minute)
elections

Describes this node’s understanding of what elections are happening on the 
network (likely to be inaccurate if this is the node being elected out)**

inprogress - whether or not an election is ongoing
vmindex - the VM being elected for (omitted if inprogress is false)
fedindex - index of the Federated server that the election is for (omitted if 
inprogress is false)
fedid - identity chain ID of the Federated server that the election is for 
(omitted if inprogress is false)
round - the current round of elections (omitted if inprogress is false).

# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .diagnostics()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */

  pub fn diagnostics(self) -> impl Future<Item=Response, Error=FetchError> {
     self.call("diagnostics", HashMap::new())
  }

/**
Returns the number of Factoshis (Factoids *10^-8) that purchase a single 
Entry Credit. The minimum factoid fees are also determined by this rate, along 
with how complex the factoid transaction is.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .entry_credit_rate()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/

  pub fn entry_credit_rate(self)-> impl Future<Item=Response, Error=FetchError>{
    self.call("entry-credit-rate", HashMap::new())
  }

/**
Returns various heights that allows you to view the state of the blockchain. 
The heights returned provide a lot of information regarding the state of factomd, 
but not all are needed by most applications. The heights also indicate the 
most recent block, which could not be complete, and still being built. The 
heights mean as follows:

* directoryblockheight : The current directory block height of the local 
factomd node.
* leaderheight : The current block being worked on by the leaders in the network. 
This block is not yet complete, but all transactions submitted will go into 
this block (depending on network conditions, the transaction may be delayed 
into the next block)
* entryblockheight : The height at which the factomd node has all the entry 
blocks. Directory blocks are obtained first, entry blocks could be lagging 
behind the directory block when syncing.
* entryheight : The height at which the local factomd node has all the 
entries. If you added entries at a block height above this, they will not be 
able to be retrieved by the local factomd until it syncs further.

A fully synced node should show the same number for all, (except between 
minute 0 and 1, when leaderheight will be 1 block ahead.)
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom.heights()
                        .map(|response| response).map_err(|err| err);
let result = fetch(query);
let response = result.unwrap();
assert!(response.success());   
```
*/
  pub fn heights(self)-> impl Future<Item=Response, Error=FetchError>{
    self.call("heights", HashMap::new())
  }

/**
Retrieve current properties of the Factom system, including the software and 
the API versions.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom.properties()
                        .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());   
```
*/
  pub fn properties(self)-> impl Future<Item=Response, Error=FetchError>{
    self.call("properties", HashMap::new())
  }


/**
Retrieve a receipt providing cryptographically verifiable proof that information 
was recorded in the factom blockchain and that this was subsequently anchored 
in the bitcoin blockchain.
# Example
```
use factom::*;

let hash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let factom = Factom::new();
let query = factom.receipt(hash)
                        .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn receipt(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("hash".to_string(), json!(hash));
    self.call("receipt", params)
  }

/**
Send a raw hex encoded binary message to the Factom network. This is mostly 
just for debugging and testing.
# Example
```
use factom::*;

let hash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let factom = Factom::new();
let query = factom.send_raw_message(hash)
                        .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn send_raw_message(self, msg: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("message".to_string(), json!(msg));
    self.call("send-raw-message", params)
  }
}
// Search options for the transactions function
pub enum SearchBy{
  Range(u32, u32),
  Txid(&'static str),
  Address(&'static str)
}


pub fn str_to_hex(utf8: &str) -> String {
  let strs: Vec<String> = utf8.as_bytes()
                              .iter()
                              .map(|b| format!("{:02X}", b))
                              .collect();
  strs.join("")
}

// Retrieves future, blocks until Result is returned
pub fn fetch<F, R, E>(fut: F)-> Result<R, E>
  where
    F: Send + 'static + Future<Item = R, Error = E>,
    R: Send + 'static,
    E: Send + 'static,
  {
    let mut runtime = Runtime::new().expect("Unable to create a tokio runtime");
    runtime.block_on(fut)
  }

pub fn to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

/// current-minute function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CurrentMinute {
    leaderheight: i64,
    directoryblockheight: i64,
    minute: i64,
    currentblockstarttime: i64,
    currentminutestarttime: i64,
    currenttime: i64,
    directoryblockinseconds: i64,
    stalldetected: bool,
    faulttimeout: i64,
    roundtimeout: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Diagnostics {
    name: String,
    id: String,
    publickey: String,
    role: String,
    leaderheight: i64,
    currentheight: i64,
    currentminute: i64,
    currentminuteduration: f64,
    previousminuteduration: f64,
    balancehash: String,
    tempbalancehash: String,
    lastblockfromdbstate: bool,
    syncing: Syncing,
    authset: Authset,
    elections: Elections,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Syncing {
    status: String,
    received: i64,
    expected: i64,
    missing: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Authset {
    leaders: Vec<Leader>,
    audits: Vec<Audit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Leader {
    id: String,
    vm: i64,
    listheight: i64,
    listlength: i64,
    nextnil: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Audit {
    id: String,
    online: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Elections {
    inprogress: bool,
}

// entry-credit-rate function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct EcRate {
    rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Heights {
    directoryblockheight: i64,
    leaderheight: i64,
    entryblockheight: i64,
    entryheight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Properties {
    factomdversion: String,
    factomdapiversion: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ReceiptResult {
    receipt: Receipt,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Receipt {
    entry: Entry,
    merklebranch: Vec<Merklebranch>,
    entryblockkeymr: String,
    directoryblockkeymr: String,
    directoryblockheight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Entry {
    entryhash: String,
    raw: String,
    timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Merklebranch {
    left: String,
    right: String,
    top: String,
}
