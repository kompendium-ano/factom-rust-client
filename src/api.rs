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


}