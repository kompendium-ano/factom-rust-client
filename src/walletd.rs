use super::*;
use utils::*;

impl Factom{

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
Adds an input to the transaction from the given address. The public address is given, and the wallet must have the private key associated with the address to successfully sign the transaction.

The input is measured in factoshis, so to send ten factoids, you must input 1,000,000,000 factoshis (without commas in JSON)

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
Adds a factoid address output to the transaction. Keep in mind the output is done in factoshis. 1 factoid is 1,000,000,000 factoshis.

So to send ten factoids, you must send 1,000,000,000 factoshis (no commas in JSON).

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
Retrieve the public and private parts of a Factoid or Entry Credit address stored in the wallet.


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

/**
This method, compose-chain, will return the appropriate API calls to create a chain in factom. You must first call the commit-chain, then the reveal-chain API calls. To be safe, wait a few seconds after calling commit.

Note: The firstentry fields are automatically hex encoded for the server to process.
# Example
```
use factom::*;

let ec_address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let extids = vec!("Cargo Test", "test harness");
let content = "Here be the content";
let factom = Factom::new();
let query = factom
            .compose_chain(extids, content, ec_address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn compose_chain(self, extids: Vec<&str>, content: &str, ecpub: &str)
                                                -> impl Future<Item=Response, Error=FetchError>{
        
        let mut params = HashMap::new();
        let hex_content = str_to_hex(content);
        let mut hex_extids = Vec::new();
        for extid in extids{
            hex_extids.push(str_to_hex(extid));
        }
        let chain = json!({
            "firstentry": {
                "extids": hex_extids,
                "content": hex_content
            }
        });
        params.insert("chain".to_string(), chain);
        params.insert("ecpub".to_string(), json!(ecpub));
        dbg!(&params);
        self.walletd_call("compose-chain", params)
    }

/**
This method, compose-entry, will return the appropriate API calls to create an entry in factom. You must first call the commit-entry, then the reveal-entry API calls. To be safe, wait a few seconds after calling commit.

Note: The entry fields are automatically hex encoded for the server to process.
# Example
```
use factom::*;

let chainid = "9dec48601fba6ddb4bcea12066ba0f2b2467f89c788c5a243eb253c3de0f815b";
let ec_address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let extids = vec!("Cargo Test", "test harness");
let content = "Here be the content";
let factom = Factom::new();
let query = factom
            .compose_entry(chainid, extids, content, ec_address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 
```
*/
    pub fn compose_entry(self, chainid: &str, extids: Vec<&str>, content: &str, ecpub: &str)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        let mut hex_extids = Vec::new();
        for extid in extids {
            hex_extids.push(str_to_hex(extid));
        }
        let entry = json!({
        "chainid": chainid,
        "extids": hex_extids,
        "content": str_to_hex(content)
        });
        params.insert("entry".to_string(), entry);
        params.insert("ecpub".to_string(), json!(ecpub));
        self.walletd_call("compose-entry", params)
    }

/**
Compose transaction marshals the transaction into a hex encoded string. The string can be inputted into the factomd API factoid-submit to be sent to the network.
# Example
```
use factom::*;

```
*/
    pub fn compose_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.walletd_call("compose-transaction", params)
    }

/**
Deletes a working transaction in the wallet. The full transaction will be returned, and then deleted.
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
Create a new Entry Credit Address and store it in the wallet.
# Example
```
use factom::*;
let factom = Factom::new();
let query = factom
            .generate_ec_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn generate_ec_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_call("generate-ec-address", HashMap::new())
    }

/**
Create a new Entry Credit Address and store it in the wallet.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .generate_factoid_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn generate_factoid_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_call("generate-factoid-address", HashMap::new())
    }

/**
Get the current hight of blocks that have been cached by the wallet while syncing.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .get_height()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn get_height(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_call("get-height", HashMap::new())
    }

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
This will create a new transaction. The txid is in flux until the final transaction is signed. Until then, it should not be used or recorded.

When dealing with transactions all factoids are represented in factoshis. 1 factoid is 1e8 factoshis, meaning you can never send anything less than 0 to a transaction (0.5).
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
Retrieve current properties of factom-walletd, including the wallet and wallet API versions.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .properties()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn walletd_properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_call("properties", HashMap::new())
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
When paying from a transaction, you can also make the receiving transaction pay for it. Using sub fee, you can use the receiving address in the parameters, and the fee will be deducted from their output amount.

This allows a wallet to send all it’s factoids, by making the input and output the remaining balance, then using sub fee on the output address.

*/  
    pub fn sub_fee(self, tx_name: &str, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        params.insert("address".to_string(), json!(address));
        self.walletd_call("sub-fee", params)
    }

/**
Lists all the current working transactions in the wallet. These are transactions that are not yet sent.
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
This will retrieve a transaction by the given TxID. This call is the fastest way to retrieve a transaction, but it will not display the height of the transaction. If a height is in the response, it will be 0. To retrieve the height of a transaction, use the 'By Address’ method

This call in the backend actually pushes the request to factomd. For a more informative response, it is advised to use the factomd transaction method

### By Address
Retrieves all transactions that involve a particular address.
# Example
```
use factom::*;


let tx = api::SearchBy::Range(1,2);
let factom = Factom::new();
let query = factom
            .transactions(tx)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success()); 

let address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let add_tx = api::SearchBy::Address(address);
let add_query = factom
                .transactions(add_tx)
                .map(|response| response).map_err(|err| err);
let add_response = fetch(add_query).unwrap();
assert!(add_response.success());  

let txid = "21fc64855771f2ee12da2a85b1aa0108007ed3a566425f3eaec7c8c7d2db6c6d";
let id_tx = api::SearchBy::Txid(txid);
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

/**
Return the wallet seed and all addresses in the wallet for backup and offline storage.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .wallet_backup()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn wallet_backup(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_call("wallet-backup", HashMap::new())
    } 

/**
The wallet-balances API is used to query the acknowledged and saved balances for all addresses in the currently running factom-walletd. The saved balance is the last saved to the database and the acknowledged or “ack” balance is the balance after processing any in-flight transactions known to the Factom node responding to the API call. The factoid address balance will be returned in factoshis (a factoshi is 10^8 factoids) not factoids(FCT) and the entry credit balance will be returned in entry credits.

* If walletd and factomd are not both running this call will not work.

* If factomd is not loaded up all the way to last saved block it will return: “result”:{“Factomd Error”:“Factomd is not fully booted, please wait and try again.”}

* If an address is not in the correct format the call will return: “result”:{“Factomd Error”:”There was an error decoding an address”}

* If an address does not have a public and private address known to the wallet it will not be included in the balance.

* "fctaccountbalances" are the total of all factoid account balances returned in factoshis.

* "ecaccountbalances" are the total of all entry credit account balances returned in entry credits.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .wallet_balances()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/

    pub fn wallet_balances(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_call("wallet-balances", HashMap::new())
    } 
}
