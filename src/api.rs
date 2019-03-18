use super::*;


pub fn str_to_hex(utf8: &str) -> String {
  let strs: Vec<String> = utf8.as_bytes()
                                .iter()
                                .map(|b| format!("{:02X}", b))
                                .collect();
  strs.join("")
}

impl Factomd{
/**
Retrieve administrative blocks for any given height.

The admin block contains data related to the identities within the factom system and the decisions the system makes as it builds the block chain. The ‘abentries’ (admin block entries) in the JSON response can be of various types, the most common is a directory block signature (DBSig). A majority of the federated servers sign every directory block, meaning every block after m5 will contain 5 DBSigs in each admin block.

The ABEntries are detailed here: https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#adminid-bytes
*/
    pub fn ablock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("ablock-by-height")
                                .parameters(params)
                                .to_json())
    }

/**
This api call is used to find the status of a transaction, whether it be a factoid, reveal entry, or commit entry. When using this, you must specify the type of the transaction by giving the chainid field 1 of 3 values:

    f for factoid transactions
    c for entry credit transactions (commit entry/chain)
    ################################################################ for reveal entry/chain
        Where # is the ChainID of the entry

The status types returned are as follows:

    “Unknown” : Not found anywhere
    “NotConfirmed” : Found on local node, but not in network (Holding Map)
    “TransactionACK” : Found in network, but not written to the blockchain yet (ProcessList)
    “DBlockConfirmed” : Found in Blockchain

You may also provide the full marshaled transaction, instead of a hash, and it will be hashed for you.

The responses vary based on the type:

### Entries
Requesting an entry requires you to specify if the hash you provide is a commit or an entry hash. The chainid field is used to specify this. If you are searching for a commit, put c as the chainid field, otherwise, put the chainid that the entry belongs too.

For commit/reveal acks, the response has 2 sections, one for the commit, one for the reveal. If you provide the entryhash and chainid, both will be filled (if found). If you only provide the commit txid and c as the chainid, then only the commitdata is guaranteed to come back with data. The committxid and entryhash fields correspond to the commitdata and entrydata objects.

Extra notes:
Why c? It is short for 000000000000000000000000000000000000000000000000000000000000000c, which is the chainid for all entry credit blocks. All commits are placed in the entry credit block (assuming they are valid and are properly paid for)

### Factoid Transactions

The hash field for a factoid transaction is equivalent to txid. To indicate the hash is a factoid transaction, put f in the chainid field and the txid in the hash field.

The response will look different than entry related ack calls.

##### Extra notes:
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
        self.api_call(ApiRequest::method("ack")
                                    .parameters(params)
                                    .to_json())
    }

/**
Retrieve a specified admin block given its merkle root key.
*/   
    pub fn admin_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("admin-block")
                                .parameters(params)
                                .to_json())
    }

/**
Return the keymr of the head of the chain for a chain ID (the unique hash created when the chain was created).
*/
    pub fn chain_head(self, chainid: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("chainid".to_string(), json!(chainid));
        self.api_call(ApiRequest::method("chain-head")
                                .parameters(params)
                                .to_json())
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
        self.api_call(ApiRequest::method("commit-chain")
                                .parameters(params)
                                .to_json())
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
        self.api_call(ApiRequest::method("commit-entry")
                                .parameters(params)
                                .to_json())
    }

/**
The current-minute API call returns:

    leaderheight returns the current block height.

    directoryblockheight returns the last saved height.

    minute returns the current minute number for the open entry block.

    currentblockstarttime returns the start time for the current block.

    currentminutestarttime returns the start time for the current minute.

    currenttime returns the current nodes understanding of current time.

    directoryblockinseconds returns the number of seconds per block.

    stalldetected returns if factomd thinks it has stalled.

    faulttimeout returns the number of seconds before leader node is faulted for failing to provide a necessary message.

    roundtimeout returns the number of seconds between rounds of an election during a fault.

*/
    pub fn current_minute(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("current-minute")
                                .to_json())
    }

/**
Retrieve a directory block given only its height.
*/
    pub fn dblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("dblock-by-height")
                                .parameters(params)
                                .to_json())
    }


/**
Every directory block has a KeyMR (Key Merkle Root), which can be used to retrieve it. The response will contain information that can be used to navigate through all transactions (entry and factoid) within that block. The header of the directory block will contain information regarding the previous directory block’s keyMR, directory block height, and the timestamp. 
*/
    pub fn directory_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("directory-block")
                                .parameters(params)
                                .to_json())
    }

/**
The directory block head is the last known directory block by factom, or in other words, the most recently recorded block. This can be used to grab the latest block and the information required to traverse the entire blockchain. 
*/
    pub fn directory_block_head(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("directory-block-head")
                                .to_json())
    }

/**
Retrieve the entry credit block for any given height. These blocks contain entry credit transaction information.
*/
    pub fn ecblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("ecblock-by-height")
                                .parameters(params)
                                .to_json())
    }

/**
Get an Entry from factomd specified by the Entry Hash.
*/
    pub fn entry(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("entry")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve a specified entry block given its merkle root key. The entry block contains 0 to many entries

*/
    pub fn entry_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        let json = ApiRequest::method("entry-block")
                                .parameters(params)
                                .to_json();
        self.api_call(json)
    }

/**
Return its current balance for a specific entry credit address.

*/
    pub fn entry_credit_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("entry-credit-balance")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve a specified entrycredit block given its merkle root key. The numbers are minute markers.

*/
    pub fn entry_credit_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("entrycredit-block")
                                .parameters(params)
                                .to_json())
    }

/**
Returns the number of Factoshis (Factoids *10^-8) that purchase a single Entry Credit. The minimum factoid fees are also determined by this rate, along with how complex the factoid transaction is.

*/
    pub fn entry_credit_rate(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("entry-credit-rate")
                                .to_json())
    }

/**
This call returns the number of Factoshis (Factoids *10^-8) that are currently available at the address specified.

*/
    pub fn factoid_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("factoid-balance")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve a specified factoid block given its merkle root key.

*/
    pub fn factoid_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("factoid-block")
                                .parameters(params)
                                .to_json())
    }

/**
Submit a factoid transaction. The transaction hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#factoid-transaction)

The factoid-submit API takes a specifically formatted message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this factoid-submit API call with compose-transaction which takes easier to construct arguments.

*/
    pub fn factoid_submit(self, transaction: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("transaction".to_string(), json!(transaction));
        self.api_call(ApiRequest::method("factoid-submit")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve the factoid block for any given height. These blocks contain factoid transaction information.

*/
    pub fn fblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("fblock-by-height")
                                .parameters(params)
                                .to_json())
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
        self.api_call(ApiRequest::method("heights")
                                .to_json())
    }

/**
The multiple-ec-balances API is used to query the acknowledged and saved balances for a list of entry credit addresses.

    * currentheight is the current height that factomd was loading.
    * lastsavedheight is the height last saved to the database.

    * In balances it returns "ack", "saved" and "err".
        * ack is the balance after processing any in-flight transactions known to the Factom node responding to the API call
        * saved is the last saved to the database
        * err is just used to display any error that might have happened during the request. If it is "" that means there was no error.

    * If the syntax of the parameters is off e.g. missing a quote, a comma, or a square bracket, it will return: {“jsonrpc”:“2.0”,“id”:null,“error”:{“code”:-32600,“message”:“Invalid Request”}}

    * If the parameters are labeled incorrectly the call will return: “{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed in, expected 'addresses’”}”

    * If factomd is not loaded up all the way to the last saved block it will return: {“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Not fully booted”}]}

    * If the list of addresses contains an incorrectly formatted address the call will return: {“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Error decoding address”}]}

    * If an address in the list is valid but has never been part of a transaction the call will return: “balances”:[{“ack”:0,“saved”:0,“err”:“Address has not had a transaction”}]“

*/
    pub fn multiple_ec_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("addresses".to_string(), json!(addresses));
        self.api_call(ApiRequest::method("multiple-ec-balances")
                                .parameters(params)
                                .to_json())
    }

/**
The multiple-fct-balances API is used to query the acknowledged and saved balances in factoshis (a factoshi is 10^8 factoids) not factoids(FCT) for a list of FCT addresses.

    * currentheight is the current height that factomd was loading.
    * lastsavedheight is the height last saved to the database.

    * In balances it returns "ack", "saved" and "err".
        * ack is the balance after processing any in-flight transactions known to the Factom node responding to the API call
        * saved is the last saved to the database
        * err is just used to display any error that might have happened during the request. If it is "" that means there was no error.

    * If the syntax of the parameters is off e.g. missing a quote, a comma, or a square bracket, it will return: {"jsonrpc”:“2.0”,“id”:null,“error”:{“code”:-32600,“message”:“Invalid Request”}}

    * If the parameters are labeled incorrectly the call will return: “{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed in, expected 'addresses’”}”

    * If factomd is not loaded up all the way to the last saved block it will return: {“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Not fully booted”}]}

    * If the list of addresses contains an incorrectly formatted address the call will return: {“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,“err”:“Error decoding address”}]}

    * If an address in the list is valid but has never been part of a transaction it will return: “balances”:[{“ack”:0,“saved”:0,“err”:“Address has not had a transaction”}]“

*/
    pub fn multiple_fct_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("addresses".to_string(), json!(addresses));
        self.api_call(ApiRequest::method("multiple-fct-balances")
                                .parameters(params)
                                .to_json())
    }

/**
Returns an array of the entries that have been submitted but have not been recorded into the blockchain.

*/
    pub fn pending_entries(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("pending-entries")
                                .to_json())
    }

/**
Returns an array of factoid transactions that have not yet been recorded in the blockchain, but are known to the system.

*/
    pub fn pending_transactions(self, address: Option<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        if let Some(add) = address {
            params.insert("address".to_string(), json!(add));
        }
        self.api_call(ApiRequest::method("pending-transactions")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve current properties of the Factom system, including the software and the API versions.

*/
    pub fn properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("properties")
                                .to_json())
    }

/**
Retrieve an entry or transaction in raw format, the data is a hex encoded string. 

*/
    pub fn raw_data(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("raw-data")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve a receipt providing cryptographically verifiable proof that information was recorded in the factom blockchain and that this was subsequently anchored in the bitcoin blockchain.

*/
    pub fn receipt(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("receipt")
                                .parameters(params)
                                .to_json())
    }

/**
Reveal the First Entry in a Chain to factomd after the Commit to complete the Chain creation. The reveal-chain hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)

The reveal-chain API takes a specifically formatted message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this reveal-chain API call with compose-chain which takes easier to construct arguments.

The compose-chain api call has two api calls in its response: commit-chain and reveal-chain. To successfully create a chain, the reveal-chain must be called after the commit-chain.
*/
    pub fn reveal_chain(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("entry".to_string(), json!(entry));
        self.api_call(ApiRequest::method("reveal-chain")
                                .parameters(params)
                                .to_json())
    }

/**
Reveal an Entry to factomd after the Commit to complete the Entry creation. The reveal-entry hex encoded string is documented here: [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)

The reveal-entry API takes a specifically formatted message encoded in hex that includes signatures. If you have a factom-walletd instance running, you can construct this reveal-entry API call with compose-entry which takes easier to construct arguments.

The compose-entry api call has two api calls in it’s response: commit-entry and reveal-entry. To successfully create an entry, the reveal-entry must be called after the commit-entry.
*/
    pub fn reveal_entry(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("entry".to_string(), json!(entry));
        self.api_call(ApiRequest::method("reveal-entry")
                                .parameters(params)
                                .to_json())
    }

/**
Retrieve details of a factoid transaction using a transaction’s hash (or corresponding transaction id).

Note that information regarding the

    * directory block height,
    * directory block keymr, and
    * transaction block keymr

are also included.

The "blockheight” parameter in the response will always be 0 when using this call, refer to “includedindirectoryblockheight” if you need the height.

Note: This call will also accept an entry hash as input, in which case the returned data concerns the entry. The returned fields and their format are shown in the 2nd Example Response at right.

Note: If the input hash is non-existent, the returned fields will be as follows:

    * “includedintransactionblock”:“”
    * “includedindirectoryblock”:“”
    * “includedindirectoryblockheight”:-1

*/
    pub fn transaction(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("transaction")
                                .parameters(params)
                                .to_json())
    }
}

impl Walletd{

/**

*/
    pub fn add_ec_output(self, txname: &str, address: &str, amount: u64)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(txname));
        params.insert("address".to_string(), json!(address));
        params.insert("amount".to_string(), json!(amount));
        self.api_call(ApiRequest::method("add-ec-output")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn add_fee(self, txname: &str, address: &str)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(txname));
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("add-fee")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn add_input(self, txname: &str, address: &str, amount: u64)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(txname));
        params.insert("address".to_string(), json!(address));
        params.insert("amount".to_string(), json!(amount));
        self.api_call(ApiRequest::method("add-input")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn add_output(self, txname: &str, address: &str, amount: u64)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(txname));
        params.insert("address".to_string(), json!(address));
        params.insert("amount".to_string(), json!(amount));
        self.api_call(ApiRequest::method("add-output")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn address(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("address")
                                .parameters(params)
                                .to_json())
    }

/**

*/  
    pub fn all_addresses(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("all-addresses")
                                .to_json())
    }

/**

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
        self.api_call(ApiRequest::method("compose-chain")
                                    .parameters(params)
                                    .to_json())
    }

/**

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
        self.api_call(ApiRequest::method("compose-entry")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn compose_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("compose-transaction")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn delete_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("delete-transaction")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn generate_ec_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("generate-ec-address")
                                .to_json())
    }

/**

*/
    pub fn generate_factoid_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("generate-factoid-address")
                                .to_json())
    }

/**

*/
    pub fn get_height(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("get-height")
                                .to_json())
    }

/**

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
        let json = ApiRequest::method("import-addresses")
                                    .parameters(params)
                                    .to_json();
        self.api_call(json)
    }

/**

*/
    pub fn new_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("new-transaction")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("properties")
                                .to_json())
    }

/**

*/
    pub fn sign_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("sign-transaction")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/  
    pub fn sub_fee(self, tx_name: &str, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("sub-fee")
                                    .parameters(params)
                                    .to_json())
    }

/**

*/
    pub fn tmp_transactions(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("tmp-transactions")
                                .to_json())
    } 

/**

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
        self.api_call(ApiRequest::method("transactions")
                                    .parameters(params)
                                    .to_json())
    } 

/**

*/
    pub fn wallet_backup(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("wallet-backup")
                                .to_json())
    } 

/**

*/
    pub fn wallet_balances(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("wallet-balances")
                                .to_json())
    } 
}

// Transactions function different search options available
pub enum SearchBy{
    Range(u32, u32),
    Txid(&'static str),
    Address(&'static str)
}