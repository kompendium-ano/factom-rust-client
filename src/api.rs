use super::*;

impl Factomd{
    pub fn ablock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("ablock-by-height")
                                .parameters(params)
                                .to_json())
    }

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
    
    pub fn admin_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("admin-block")
                                .parameters(params)
                                .to_json())
    }

    pub fn chain_head(self, chainid: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("chainid".to_string(), json!(chainid));
        self.api_call(ApiRequest::method("chain-head")
                                .parameters(params)
                                .to_json())
    }

    pub fn commit_chain(self, message: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("message".to_string(), json!(message));
        self.api_call(ApiRequest::method("commit-chain")
                                .parameters(params)
                                .to_json())
    }

    pub fn commit_entry(self, message: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("message".to_string(), json!(message));
        self.api_call(ApiRequest::method("commit-entry")
                                .parameters(params)
                                .to_json())
    }

    pub fn current_minute(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("current-minute")
                                .to_json())
    }


    pub fn dblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("dblock-by-height")
                                .parameters(params)
                                .to_json())
    }


    pub fn directory_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("directory-block")
                                .parameters(params)
                                .to_json())
    }

    pub fn directory_block_head(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("directory-block-head")
                                .to_json())
    }

    pub fn ecblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("ecblock-by-height")
                                .parameters(params)
                                .to_json())
    }


    pub fn entry(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("entry")
                                .parameters(params)
                                .to_json())
    }

    pub fn entry_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("entry-block")
                                .parameters(params)
                                .to_json())
    }

    pub fn entry_credit_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("entry-credit-balance")
                                .parameters(params)
                                .to_json())
    }

    pub fn entry_credit_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("entrycredit-block")
                                .parameters(params)
                                .to_json())
    }

    pub fn entry_credit_rate(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("entry-credit-rate")
                                .to_json())
    }

    pub fn factoid_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("factoid-balance")
                                .parameters(params)
                                .to_json())
    }

    pub fn factoid_block(self, keymr: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("keymr".to_string(), json!(keymr));
        self.api_call(ApiRequest::method("factoid-block")
                                .parameters(params)
                                .to_json())
    }

    pub fn factoid_submit(self, transaction: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("transaction".to_string(), json!(transaction));
        self.api_call(ApiRequest::method("factoid-submit")
                                .parameters(params)
                                .to_json())
    }


    pub fn fblock_by_height(self, height: u32)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("height".to_string(), json!(height));
        self.api_call(ApiRequest::method("fblock-by-height")
                                .parameters(params)
                                .to_json())
    }


    pub fn heights(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("heights")
                                .to_json())
    }

    pub fn multiple_ec_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("addresses".to_string(), json!(addresses));
        self.api_call(ApiRequest::method("multiple-ec-balances")
                                .parameters(params)
                                .to_json())
    }

    pub fn multiple_fct_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("addresses".to_string(), json!(addresses));
        self.api_call(ApiRequest::method("multiple-fct-balances")
                                .parameters(params)
                                .to_json())
    }

    pub fn pending_entries(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("pending-entries")
                                .to_json())
    }

    pub fn pending_transactions(self, address: Option<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        if let Some(add) = address {
            params.insert("address".to_string(), json!(add));
        }
        self.api_call(ApiRequest::method("pending-transactions")
                                .parameters(params)
                                .to_json())
    }

    pub fn properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("properties")
                                .to_json())
    }

    pub fn raw_data(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("raw-data")
                                .parameters(params)
                                .to_json())
    }

    pub fn receipt(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("receipt")
                                .parameters(params)
                                .to_json())
    }

    pub fn reveal_chain(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("entry".to_string(), json!(entry));
        self.api_call(ApiRequest::method("reveal-chain")
                                .parameters(params)
                                .to_json())
    }

    pub fn reveal_entry(self, entry: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("entry".to_string(), json!(entry));
        self.api_call(ApiRequest::method("reveal-entry")
                                .parameters(params)
                                .to_json())
    }


    pub fn transaction(self, hash: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("hash".to_string(), json!(hash));
        self.api_call(ApiRequest::method("transaction")
                                .parameters(params)
                                .to_json())
    }
}

impl Walletd{



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


    pub fn add_fee(self, txname: &str, address: &str)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(txname));
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("add-fee")
                                    .parameters(params)
                                    .to_json())
    }


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

    pub fn address(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("address")
                                .parameters(params)
                                .to_json())
    }
    
    pub fn all_addresses(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("all-addresses")
                                .to_json())
    }

    pub fn compose_chain(self, extids: Vec<&str>, content: &str, ecpub: &str)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        let chain = json!({
            "firstentry": {
                "extids": extids,
                "content": content
            }
        });
        params.insert("chain".to_string(), chain);
        params.insert("ecpub".to_string(), json!(ecpub));
        self.api_call(ApiRequest::method("compose-chain")
                                    .parameters(params)
                                    .to_json())
    }

    pub fn compose_entry(self, chainid: &str, extids: Vec<&str>, content: &str, ecpub: &str)
                                                -> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        let entry = json!({
        "chainid": chainid,
        "extids": extids,
        "content": content
        });
        params.insert("entry".to_string(), entry);
        params.insert("ecpub".to_string(), json!(ecpub));
        self.api_call(ApiRequest::method("compose-entry")
                                    .parameters(params)
                                    .to_json())
    }

    pub fn compose_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("compose-transaction")
                                    .parameters(params)
                                    .to_json())
    }


    pub fn delete_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("delete-transaction")
                                    .parameters(params)
                                    .to_json())
    }

    pub fn generate_factoid_address(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("generate-factoid-address")
                                .to_json())
    }

    pub fn get_height(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("get-height")
                                .to_json())
    }


    pub fn import_addresses(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        let mut secrets: HashMap<&str, &str> = HashMap::new();
        for address in addresses{
            secrets.insert("secret", address);
        }
        params.insert("addresses".to_string(), json!(secrets));
        self.api_call(ApiRequest::method("import-addresses")
                                    .parameters(params)
                                    .to_json())
    }

    pub fn new_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("new-transaction")
                                    .parameters(params)
                                    .to_json())
    }

    pub fn properties(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("properties")
                                .to_json())
    }


    pub fn sign_transaction(self, tx_name: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        self.api_call(ApiRequest::method("sign-transaction")
                                    .parameters(params)
                                    .to_json())
    }

    
    pub fn sub_fee(self, tx_name: &str, address: &str)-> impl Future<Item=Response, Error=FetchError>{
        let mut params = HashMap::new();
        params.insert("tx-name".to_string(), json!(tx_name));
        params.insert("address".to_string(), json!(address));
        self.api_call(ApiRequest::method("sub-fee")
                                    .parameters(params)
                                    .to_json())
    }

    pub fn tmp_transactions(self)-> impl Future<Item=Response, Error=FetchError>{
        self.api_call(ApiRequest::method("tmp-transactions")
                                .to_json())
    } 

    pub fn transactions<T>(self, filter: SearchBy )-> impl Future<Item=Response, Error=FetchError>{
         
        let mut params = HashMap::new();

        match filter {
            SearchBy::Txid(txid) => {
                                params.insert("txid".to_string(), json!(txid));
                                }
            SearchBy::Address(address) => {
                                params.insert("address".to_string(), json!(address));
                                }
            SearchBy::Range((start, end)) => {
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



}

// Transactions function different search options available
pub enum SearchBy{
    Range((u32, u32)),
    Txid(&'static str),
    Address(&'static str)
}