use super::*;


pub fn str_to_hex(utf8: &str) -> String {
  let strs: Vec<String> = utf8.as_bytes()
                                .iter()
                                .map(|b| format!("{:02X}", b))
                                .collect();
  strs.join("")
}

// Daemon
impl Factom{
/**
Retrieve administrative blocks for any given height.

The admin block contains data related to the identities within the factom system and the decisions the system makes as it builds the block chain. The ‘abentries’ (admin block entries) in the JSON response can be of various types, the most common is a directory block signature (DBSig). A majority of the federated servers sign every directory block, meaning every block after m5 will contain 5 DBSigs in each admin block.

The ABEntries are detailed [here](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#adminid-bytes)
*/
    pub fn ablock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call("ablock-by-height", params)
    }

/**
This api call is used to find the status of a transaction, whether it be a factoid, reveal entry, or commit entry. When using this, you must specify the type of the transaction by giving the chainid field 1 of 3 values:

* f for factoid transactions
* c for entry credit transactions (commit entry/chain)
* <ChainID> for reveal entry/chain

The status types returned are as follows:

* “Unknown” : Not found anywhere
* “NotConfirmed” : Found on local node, but not in network (Holding Map)
* “TransactionACK” : Found in network, but not written to the blockchain yet (ProcessList)
* “DBlockConfirmed” : Found in Blockchain

You may also provide the full marshaled transaction, instead of a hash, and it will be hashed for you.

The responses vary based on the type:

### Entries
Requesting an entry requires you to specify if the hash you provide is a commit or an entry hash. The chainid field is used to specify this. If you are searching for a commit, put c as the chainid field, otherwise, put the chainid that the entry belongs too.

For commit/reveal acks, the response has 2 sections, one for the commit, one for the reveal. If you provide the entryhash and chainid, both will be filled (if found). If you only provide the commit txid and c as the chainid, then only the commitdata is guaranteed to come back with data. The committxid and entryhash fields correspond to the commitdata and entrydata objects.

### Factoid Transactions

The hash field for a factoid transaction is equivalent to txid. To indicate the hash is a factoid transaction, put f in the chainid field and the txid in the hash field.

The response will look different than entry related ack calls.

##### Extra notes:

Why c? It is short for 000000000000000000000000000000000000000000000000000000000000000c, which is the chainid for all entry credit blocks. All commits are placed in the entry credit block (assuming they are valid and are properly paid for)

Why f? It is short for 000000000000000000000000000000000000000000000000000000000000000f, which is the chainid for all factoid blocks. All factoid transactions are placed in the factoid (assuming they are valid)
*/
    pub fn ack(self, hash: &str, chainid: &str, full_transaction: Option<&str>)
                                    -> impl Future<Item=Response, Error=FetchError> {
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        params.insert("chainid".to_string(), json!(chainid));
        if let Some(tx) = full_transaction{
            params.insert("fulltransaction".to_string(), json!(tx));
        }
        self.api_call("ack", params)
    }

/**
Retrieve a specified admin block given its merkle root key.
*/   
    pub fn admin_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call("admin-block", params)
    }

/**
Return the keymr of the head of the chain for a chain ID (the unique hash created when the chain was created).
*/
    pub fn chain_head(self, chainid: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("chainid".to_string(), json!(chainid));
        self.api_call("chain-head", params)
    }

/**
Send a Chain Commit Message to factomd to create a new Chain. The commit chain hex encoded string is documented here: Github Documentation

The commit-chain API takes a specifically formated message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this commit-chain API call with compose-chain which takes easier to construct arguments.

The compose-chain api call has two api calls in it’s response: commit-chain and reveal-chain. To successfully create a chain, the reveal-chain must be called after the commit-chain.

Notes:
It is possible to be unable to send a commit, if the commit already exists (if you try to send it twice). This is a mechanism to prevent you from double spending. If you encounter this error, just skip to the reveal-chain. The error format can be found here: repeated-commit
*/
    pub fn commit_chain(self, message: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("message".to_string(), json!(message));
        self.api_call("commit-chain", params)
    }

/**
Send an Entry Commit Message to factom to create a new Entry. The entry commit hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry-commit)

The commit-entry API takes a specifically formated message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this commit-entry API call with compose-entry which takes easier to construct arguments.

The compose-entry api call has two api calls in it’s response: commit-entry and reveal-entry. To successfully create an entry, the reveal-entry must be called after the commit-entry.

Notes:
It is possible to be unable to send a commit, if the commit already exists (if you try to send it twice). This is a mechanism to prevent you from double spending. If you encounter this error, just skip to the reveal-entry. The error format can be found here: repeated-commit
*/
    pub fn commit_entry(self, message: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("message".to_string(), json!(message));
        self.api_call("commit-entry", params)
    }

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

* `faulttimeout` returns the number of seconds before leader node is faulted for failing to provide a necessary message.

* `roundtimeout` returns the number of seconds between rounds of an election during a fault.

*/
    pub fn current_minute(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call("current-minute", HashMap::new())
    }

/**
Retrieve a directory block given only its height.
*/
    pub fn dblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call("dblock-by-height", params)
    }


/**
Every directory block has a KeyMR (Key Merkle Root), which can be used to retrieve it. The response will contain information that can be used to navigate through all transactions (entry and factoid) within that block. The header of the directory block will contain information regarding the previous directory block’s keyMR, directory block height, and the timestamp. 
*/
    pub fn directory_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call("directory-block", params)
    }

/**
The directory block head is the last known directory block by factom, or in other words, the most recently recorded block. This can be used to grab the latest block and the information required to traverse the entire blockchain. 
*/
    pub fn directory_block_head(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call("directory-block-head", HashMap::new())
    }

/**
Retrieve the entry credit block for any given height. These blocks contain entry credit transaction information.
*/
    pub fn ecblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call("ecblock-by-height", params)
    }

/**
Get an Entry from factomd specified by the Entry Hash.
*/
    pub fn entry(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call("entry", params)
    }

/**
Retrieve a specified entry block given its merkle root key. The entry block contains 0 to many entries

*/
    pub fn entry_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call("entry-block", params)
    }

/**
Return its current balance for a specific entry credit address.

*/
    pub fn entry_credit_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call("entry-credit-balance", params)
    }

/**
Retrieve a specified entrycredit block given its merkle root key. The numbers are minute markers.

*/
    pub fn entry_credit_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call("entrycredit-block", params)
    }

/**
Returns the number of Factoshis (Factoids *10^-8) that purchase a single Entry Credit. The minimum factoid fees are also determined by this rate, along with how complex the factoid transaction is.

*/
    pub fn entry_credit_rate(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call("entry-credit-rate", HashMap::new())
    }

/**
This call returns the number of Factoshis (Factoids *10^-8) that are currently available at the address specified.

*/
    pub fn factoid_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call("factoid-balance", params)
    }

/**
Retrieve a specified factoid block given its merkle root key.

*/
    pub fn factoid_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call("factoid-block", params)
    }

/**
Submit a factoid transaction. The transaction hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#factoid-transaction)

The factoid-submit API takes a specifically formatted message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this factoid-submit API call with compose-transaction which takes easier to construct arguments.

*/
    pub fn factoid_submit(self, transaction: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("transaction".to_string(), json!(transaction));
        self.api_call("factoid-submit", params)
    }

/**
Retrieve the factoid block for any given height. These blocks contain factoid transaction information.

*/
    pub fn fblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call("fblock-by-height", params)
    }

/**
Returns various heights that allows you to view the state of the blockchain. The heights returned provide a lot of information regarding the state of factomd, but not all are needed by most applications. The heights also indicate the most recent block, which could not be complete, and still being built. The heights mean as follows:

* directoryblockheight : The current directory block height of the local factomd node.
* leaderheight : The current block being worked on by the leaders in the network. This block is not yet complete, but all transactions submitted will go into this block (depending on network conditions, the transaction may be delayed into the next block)
* entryblockheight : The height at which the factomd node has all the entry blocks. Directory blocks are obtained first, entry blocks could be lagging behind the directory block when syncing.
* entryheight : The height at which the local factomd node has all the entries. If you added entries at a block height above this, they will not be able to be retrieved by the local factomd until it syncs further.

A fully synced node should show the same number for all, (except between minute 0 and 1, when leaderheight will be 1 block ahead.)

*/
    pub fn heights(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call("heights", HashMap::new())
    }

/**
The multiple-ec-balances API is used to query the acknowledged and saved balances for a list of entry credit addresses.

* currentheight is the current height that factomd was loading.
* lastsavedheight is the height last saved to the database.

* In balances it returns "ack", "saved" and "err".
    * ack is the balance after processing any in-flight transactions known to the Factom node responding to the API call
    * saved is the last saved to the database
    * err is just used to display any error that might have happened during the request. If it is empty that means there was no error.

* If the syntax of the parameters is off e.g. missing a quote, a comma, or a square bracket, it will return: `{“jsonrpc”:“2.0”,“id”:null,“error”:{“code”:-32600,“message”:“Invalid Request”}}`

* If the parameters are labeled incorrectly the call will return: `{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed in, expected addresses”}`

* If factomd is not loaded up all the way to the last saved block it will return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Not fully booted”}]}`

* If the list of addresses contains an incorrectly formatted address the call will return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Error decoding address”}]}`

* If an address in the list is valid but has never been part of a transaction the call will return: `“balances”:[{“ack”:0,“saved”:0,“err”:“Address has not had a transaction”}]`

*/
    pub fn multiple_ec_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("addresses".to_string(), json!(addresses));
        self.api_call("multiple-ec-balances", params)
    }

/**
The multiple-fct-balances API is used to query the acknowledged and saved balances in factoshis (a factoshi is 10^8 factoids) not factoids(FCT) for a list of FCT addresses.

* currentheight is the current height that factomd was loading.
* lastsavedheight is the height last saved to the database.

* In balances it returns "ack", "saved" and "err".
    * ack is the balance after processing any in-flight transactions known to the Factom node responding to the API call
    * saved is the last saved to the database
    * err is just used to display any error that might have happened during the request. If it is "" that means there was no error.

* If the syntax of the parameters is off e.g. missing a quote, a comma, or a square bracket, it will return: `{”jsonrpc”:“2.0”,“id”:null,“error”:{“code”:-32600,“message”:“Invalid Request”}}`

* If the parameters are labeled incorrectly the call will return: `{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed in, expected 'addresses’”}`

* If factomd is not loaded up all the way to the last saved block it will return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Not fully booted”}]}`

* If the list of addresses contains an incorrectly formatted address the call will return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Error decoding address”}]}`

* If an address in the list is valid but has never been part of a transaction it will return: `“balances”:[{“ack”:0,“saved”:0,“err”:“Address has not had a transaction”}]`

*/
    pub fn multiple_fct_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("addresses".to_string(), json!(addresses));
        self.api_call("multiple-fct-balances", params)
    }

/**
Returns an array of the entries that have been submitted but have not been recorded into the blockchain.

*/
    pub fn pending_entries(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call("pending-entries", HashMap::new())
    }

/**
Returns an array of factoid transactions that have not yet been recorded in the blockchain, but are known to the system.

*/
    pub fn pending_transactions(self, address: Option<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        if let Some(add) = address {
            params.insert("address".to_string(), json!(add));
        }
        self.api_call("pending-transactions", params)
    }

/**
Retrieve current properties of the Factom system, including the software and the API versions.

*/
    pub fn properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call("properties", HashMap::new())
    }

/**
Retrieve an entry or transaction in raw format, the data is a hex encoded string. 

*/
    pub fn raw_data(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call("raw-data", params)
    }

/**
Retrieve a receipt providing cryptographically verifiable proof that information was recorded in the factom blockchain and that this was subsequently anchored in the bitcoin blockchain.

*/
    pub fn receipt(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call("receipt", params)
    }

/**
Reveal the First Entry in a Chain to factomd after the Commit to complete the Chain creation. The reveal-chain hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)

The reveal-chain API takes a specifically formatted message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this reveal-chain API call with compose-chain which takes easier to construct arguments.

The compose-chain api call has two api calls in its response: commit-chain and reveal-chain. To successfully create a chain, the reveal-chain must be called after the commit-chain.
*/
    pub fn reveal_chain(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("entry".to_string(), json!(entry));
        self.api_call("reveal-chain", params)
    }

/**
Reveal an Entry to factomd after the Commit to complete the Entry creation. The reveal-entry hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)

The reveal-entry API takes a specifically formatted message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this reveal-entry API call with compose-entry which takes easier to construct arguments.

The compose-entry api call has two api calls in it’s response: commit-entry and reveal-entry. To successfully create an entry, the reveal-entry must be called after the commit-entry.
*/
    pub fn reveal_entry(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("entry".to_string(), json!(entry));
        self.api_call("reveal-entry", params)
    }

/**
Retrieve details of a factoid transaction using a transaction’s hash (or corresponding transaction id).

Note that information regarding the

* directory block height,
* directory block keymr, and
* transaction block keymr

are also included.

The "blockheight” parameter in the response will always be 0 when using this call, refer to “includedindirectoryblockheight” if you need the height.

### Notes 

This call will also accept an entry hash as input, in which case the returned data concerns the entry. The returned fields and their format are shown in the 2nd Example Response at right.

If the input hash is non-existent, the returned fields will be as follows:

* “includedintransactionblock”:“”
* “includedindirectoryblock”:“”
* “includedindirectoryblockheight”:-1

*/
    pub fn transaction(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call("transaction", params)
    }
}

// Wallet functions
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
        self.walletd_api_call("add-ec-output", params)
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
        self.walletd_api_call("add-fee", params)
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
        self.walletd_api_call("add-input", params)
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
        self.walletd_api_call("add-output", params)
    }

/**
Retrieve the public and private parts of a Factoid or Entry Credit address stored in the wallet.

*/
    pub fn address(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.walletd_api_call("address", params)
    }

/**
Retrieve all of the Factoid and Entry Credit addresses stored in the wallet.

*/  
    pub fn all_addresses(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("all-addresses", HashMap::new())
    }

/**
This method, compose-chain, will return the appropriate API calls to create a chain in factom. You must first call the commit-chain, then the reveal-chain API calls. To be safe, wait a few seconds after calling commit.

Note: The firstentry fields are automatically hex encoded for the server to process.

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
        self.walletd_api_call("compose-chain", params)
    }

/**
This method, compose-entry, will return the appropriate API calls to create an entry in factom. You must first call the commit-entry, then the reveal-entry API calls. To be safe, wait a few seconds after calling commit.

Note: The entry fields are automatically hex encoded for the server to process.

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
        self.walletd_api_call("compose-entry", params)
    }

/**
Compose transaction marshals the transaction into a hex encoded string. The string can be inputted into the factomd API factoid-submit to be sent to the network.

*/
    pub fn compose_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.walletd_api_call("compose-transaction", params)
    }

/**
Deletes a working transaction in the wallet. The full transaction will be returned, and then deleted.

*/
    pub fn delete_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.walletd_api_call("delete-transaction", params)
    }

/**
Create a new Entry Credit Address and store it in the wallet.

*/
    pub fn generate_ec_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("generate-ec-address", HashMap::new())
    }

/**
Create a new Entry Credit Address and store it in the wallet.

*/
    pub fn generate_factoid_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("generate-factoid-address", HashMap::new())
    }

/**
Get the current hight of blocks that have been cached by the wallet while syncing.

*/
    pub fn get_height(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("get-height", HashMap::new())
    }

/**
Import Factoid and/or Entry Credit address secret keys into the wallet.

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
        self.walletd_api_call("import-addresses", params)
    }

/**
This will create a new transaction. The txid is in flux until the final transaction is signed. Until then, it should not be used or recorded.

When dealing with transactions all factoids are represented in factoshis. 1 factoid is 1e8 factoshis, meaning you can never send anything less than 0 to a transaction (0.5).

*/
    pub fn new_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.walletd_api_call("new-transaction", params)
    }

/**
Retrieve current properties of factom-walletd, including the wallet and wallet API versions.

*/
    pub fn walletd_properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("properties", HashMap::new())
    }

/**
Signs the transaction. It is now ready to be executed.

*/
    pub fn sign_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.walletd_api_call("sign-transaction", params)
    }

/**
When paying from a transaction, you can also make the receiving transaction pay for it. Using sub fee, you can use the receiving address in the parameters, and the fee will be deducted from their output amount.

This allows a wallet to send all it’s factoids, by making the input and output the remaining balance, then using sub fee on the output address.

*/  
    pub fn sub_fee(self, tx_name: &str, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        params.insert("address".to_string(), json!(address));
        self.walletd_api_call("sub-fee", params)
    }

/**
Lists all the current working transactions in the wallet. These are transactions that are not yet sent.

*/
    pub fn tmp_transactions(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("tmp-transactions", HashMap::new())
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
        self.walletd_api_call("transactions", params)
    } 

/**
Return the wallet seed and all addresses in the wallet for backup and offline storage.

*/
    pub fn wallet_backup(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("wallet-backup", HashMap::new())
    } 

/**
The wallet-balances API is used to query the acknowledged and saved balances for all addresses in the currently running factom-walletd. The saved balance is the last saved to the database and the acknowledged or “ack” balance is the balance after processing any in-flight transactions known to the Factom node responding to the API call. The factoid address balance will be returned in factoshis (a factoshi is 10^8 factoids) not factoids(FCT) and the entry credit balance will be returned in entry credits.

* If walletd and factomd are not both running this call will not work.

* If factomd is not loaded up all the way to last saved block it will return: “result”:{“Factomd Error”:“Factomd is not fully booted, please wait and try again.”}

* If an address is not in the correct format the call will return: “result”:{“Factomd Error”:”There was an error decoding an address”}

* If an address does not have a public and private address known to the wallet it will not be included in the balance.

* "fctaccountbalances" are the total of all factoid account balances returned in factoshis.

* "ecaccountbalances" are the total of all entry credit account balances returned in entry credits.

*/
    pub fn wallet_balances(self)-> impl Future<Item=Response, Error=FetchError>{
        self.walletd_api_call("wallet-balances", HashMap::new())
    } 
}

// Transactions function different search options available
pub enum SearchBy{
    Range(u32, u32),
    Txid(&'static str),
    Address(&'static str)
}