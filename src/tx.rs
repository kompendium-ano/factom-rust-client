use super::*;

impl Factom {
/**
This api call is used to find the status of a transaction, whether it be a 
factoid, reveal entry, or commit entry. When using this, you must specify the 
type of the transaction by giving the chainid field 1 of 3 values:

* f for factoid transactions
* c for entry credit transactions (commit entry/chain)
* <ChainID> for reveal entry/chain

The status types returned are as follows:0

* “Unknown” : Not found anywhere
* “NotConfirmed” : Found on local node, but not in network (Holding Map)
* “TransactionACK” : Found in network, but not written to the blockchain 
yet (ProcessList)
* “DBlockConfirmed” : Found in Blockchain

You may also provide the full marshaled transaction, instead of a hash, and it 
will be hashed for you.

The responses vary based on the type:

### Entries
Requesting an entry requires you to specify if the hash you provide is a commit 
or an entry hash. The chainid field is used to specify this. If you are 
searching for a commit, put c as the chainid field, otherwise, put the chainid 
that the entry belongs too.

For commit/reveal acks, the response has 2 sections, one for the commit, one 
for the reveal. If you provide the entryhash and chainid, both will be 
filled (if found). If you only provide the commit txid and c as the chainid, 
then only the commitdata is guaranteed to come back with data. The committxid
 and entryhash fields correspond to the commitdata and entrydata objects.

### Factoid Transactions

The hash field for a factoid transaction is equivalent to txid. To indicate 
the hash is a factoid transaction, put f in the chainid field and the 
txid in the hash field.

The response will look different than entry related ack calls.

### Extra notes:

Why c? It is short for 
000000000000000000000000000000000000000000000000000000000000000c,
which is the chainid for all entry credit blocks. All commits are placed in the 
entry credit block (assuming they are valid and are properly paid for)

Why f? It is short for 
000000000000000000000000000000000000000000000000000000000000000f, 
which is the chainid for all factoid blocks. All factoid transactions are placed 
in the factoid (assuming they are valid)

# Example
```
use factom::*;

let hash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let tx_type = "f";
let factom = Factom::new();
let query = factom
            .ack(hash, tx_type, None)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn ack(self, hash: &str, chainid: &str, full_transaction: Option<&str>)
                  -> impl Future<Item=Response, Error=FetchError> {
    let mut params = HashMap::new();
    params.insert("hash".to_string(), json!(hash));
    params.insert("chainid".to_string(), json!(chainid));
    if let Some(tx) = full_transaction{
      params.insert("fulltransaction".to_string(), json!(tx));
    }
    self.call("ack", params)
  }

  /**
Submit a factoid transaction. The transaction hex encoded string is 
documented here: 
[Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#factoid-transaction)

The factoid-submit API takes a specifically formatted message encoded in hex 
that includes signatures. If you have a factom-walletd instance running, you 
can construct this factoid-submit API call with compose-transaction which 
takes easier to construct arguments.
# Example
```
use factom::*;

let tx = "0201565d109233010100b0a0e100646f3e8750c550e4582eca5047546ffef89c13a175985e320232bacac81cc428afd7c200ce7b98bfdae90f942bc1fe88c3dd44d8f4c81f4eeb88a5602da05abc82ffdb5301718b5edd2914acc2e4677f336c1a32736e5e9bde13663e6413894f57ec272e28dc1908f98b79df30005a99df3c5caf362722e56eb0e394d20d61d34ff66c079afad1d09eee21dcd4ddaafbb65aacea4d5c1afcd086377d77172f15b3aa32250a";
let factom = Factom::new();
let query = factom
      .factoid_submit(tx)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 
```
*/
  pub fn factoid_submit(self, transaction: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("transaction".to_string(), json!(transaction));
    self.call("factoid-submit", params)
  }

/**
Retrieve details of a factoid transaction using a transaction’s hash 
(or corresponding transaction id).

Note that information regarding the

* directory block height,
* directory block keymr, and
* transaction block keymr

are also included.

The "blockheight” parameter in the response will always be 0 when using this 
call, refer to “includedindirectoryblockheight” if you need the height.

### Notes 

This call will also accept an entry hash as input, in which case the returned 
data concerns the entry. The returned fields and their format are shown in the 
2nd Example Response at right.

If the input hash is non-existent, the returned fields will be as follows:

* “includedintransactionblock”:“”
* “includedindirectoryblock”:“”
* “includedindirectoryblockheight”:-1
# Example
```
use factom::*;

let hash = "21fc64855771f2ee12da2a85b1aa0108007ed3a566425f3eaec7c8c7d2db6c6d";
let factom = Factom::new();
let query = factom.transaction(hash)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn transaction(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("hash".to_string(), json!(hash));
    self.call("transaction", params)
  }

/**
Returns an array of factoid transactions that have not yet been recorded in the 
blockchain, but are known to the system.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom.pending_transactions(None)
            .map(|response| response).map_err(|err| err);
let result = fetch(query);
let response = result.unwrap();
assert!(response.success());   
```
*/
  pub fn pending_transactions(self, address: Option<&str>)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    if let Some(add) = address {
      params.insert("address".to_string(), json!(add));
    }
    self.call("pending-transactions", params)
  }

/**
When adding entry credit outputs, the amount given is in factoshis, not entry credits. This means math is required to determine the correct amount of factoshis to pay to get X EC.

(ECRate * ECTotalOutput)

In our case, the rate is 1000, meaning 1000 entry credits per factoid. We added 10 entry credits, so we need 1,000 * 10 = 10,000 factoshis

To get the ECRate search in the search bar above for “entry-credit-rate”

*/
  pub fn add_ec_output(self, txname: &str, address: &str, amount: u64)
                        -> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(txname));
    params.insert("address".to_string(), json!(address));
    params.insert("amount".to_string(), json!(amount));
    self.walletd_call("add-ec-output", params)
  }

/**
Addfee is a shortcut and safeguard for adding the required additional factoshis to covert the fee. The fee is displayed in the returned transaction after each step, but addfee should be used instead of manually adding the additional input. This will help to prevent overpaying.

Addfee will complain if your inputs and outputs do not match up. For example, in the steps above we added the inputs first. This was done intentionally to show a case of overpaying. Obviously, no one wants to overpay for a transaction, so addfee has returned an error and the message: ‘Inputs and outputs don’t add up’. This is because we have 2,000,000,000 factoshis as input and only 1,000,000,000 + 10,000 as output. Let’s correct the input by doing 'add-input’, and putting 1000010000 as the amount for the address. It will overwrite the previous input.

Run the addfee again, and the feepaid and feerequired will match up
*/
  pub fn add_fee(self, txname: &str, address: &str)
                        -> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(txname));
    params.insert("address".to_string(), json!(address));
    self.walletd_call("add-fee", params)
  }

/**
Adds an input to the transaction from the given address. The public address is 
given, and the wallet must have the private key associated with the address to 
successfully sign the transaction.

The input is measured in factoshis, so to send ten factoids, you must input 
1,000,000,000 factoshis (without commas in JSON)

*/
  pub fn add_input(self, txname: &str, address: &str, amount: u64)
                        -> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(txname));
    params.insert("address".to_string(), json!(address));
    params.insert("amount".to_string(), json!(amount));
    self.walletd_call("add-input", params)
  }

/**
Adds a factoid address output to the transaction. Keep in mind the output is 
done in factoshis. 1 factoid is 1,000,000,000 factoshis.

So to send ten factoids, you must send 1,000,000,000 factoshis 
(no commas in JSON).
*/
  pub fn add_output(self, txname: &str, address: &str, amount: u64)
                        -> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(txname));
    params.insert("address".to_string(), json!(address));
    params.insert("amount".to_string(), json!(amount));
    self.walletd_call("add-output", params)
  }
/**
Deletes a working transaction in the wallet. The full transaction will be 
returned, and then deleted.
# Example
```
use factom::*;

let txname = "test-tx";
let factom = Factom::new();
let handler = factom.clone();
fetch(handler.new_transaction(txname)
              .map(|res| res)
              .map_err(|err| err)).unwrap();
let query = factom
            .delete_transaction(txname)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn delete_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(tx_name));
    self.walletd_call("delete-transaction", params)
  }

/**
This will create a new transaction. The txid is in flux until the final 
transaction is signed. Until then, it should not be used or recorded.

When dealing with transactions all factoids are represented in factoshis. 
1 factoid is 1e8 factoshis, meaning you can never send anything less than 0 to 
a transaction (0.5).
# Example
```
use factom::*;

let txname = "new-tx-test";
let factom = Factom::new();
let handler = factom.clone();
let query = factom
              .new_transaction(txname)
              .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
fetch(handler.delete_transaction(txname).map(|_| ())).map_err(|_| ()).unwrap();
```
*/
  pub fn new_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(tx_name));
    self.walletd_call("new-transaction", params)
  }
/**
Signs the transaction. It is now ready to be executed.

*/
  pub fn sign_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(tx_name));
    self.walletd_call("sign-transaction", params)
  }

/**
When paying from a transaction, you can also make the receiving transaction 
pay for it. Using sub fee, you can use the receiving address in the parameters, 
and the fee will be deducted from their output amount.

This allows a wallet to send all it’s factoids, by making the input and output 
the remaining balance, then using sub fee on the output address.

*/  
  pub fn sub_fee(self, tx_name: &str, address: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("tx-name".to_string(), json!(tx_name));
    params.insert("address".to_string(), json!(address));
    self.walletd_call("sub-fee", params)
  }

/**
Lists all the current working transactions in the wallet. These are transactions 
that are not yet sent.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .tmp_transactions()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn tmp_transactions(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("tmp-transactions", HashMap::new())
  } 

/**
There are a few ways to search for a transaction

### Using a Range
This will retrieve all transactions within a given block height range.

### By TxID
This will retrieve a transaction by the given TxID. This call is the fastest 
way to retrieve a transaction, but it will not display the height of the 
transaction. If a height is in the response, it will be 0. To retrieve the 
height of a transaction, use the 'By Address’ method

This call in the backend actually pushes the request to factomd. For a more 
informative response, it is advised to use the factomd transaction method

### By Address
Retrieves all transactions that involve a particular address.
# Example
```
use factom::*;


let tx = utils::SearchBy::Range(1,2);
let factom = Factom::new();
let query = factom
            .transactions(tx)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 

let address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let add_tx = utils::SearchBy::Address(address);
let add_query = factom
                .transactions(add_tx)
                .map(|response| response).map_err(|err| err);
let add_response = fetch(add_query).unwrap();
assert!(add_response.success());  

let txid = "21fc64855771f2ee12da2a85b1aa0108007ed3a566425f3eaec7c8c7d2db6c6d";
let id_tx = utils::SearchBy::Txid(txid);
let id_query = factom
                .transactions(id_tx)
                .map(|response| response).map_err(|err| err);
let id_response = fetch(id_query).unwrap();
assert!(id_response.success());  
```
*/
  pub fn transactions(self, filter: SearchBy )-> impl Future<Item=Response, Error=FetchError>{
     
    let mut params = HashMap::new();

    match filter {
      SearchBy::Txid(txid) => {
                params.insert("txid".to_string(), json!(txid));
                }
      SearchBy::Address(address) => {
                params.insert("address".to_string(), json!(address));
                }
      SearchBy::Range(start, end) => {
                let mut range = HashMap::new();
                range.insert("start", json!(start));
                range.insert("end", json!(end));
                params.insert("range".to_string(),json!(range));
                }
    };      
    self.walletd_call("transactions", params)
  } 
}