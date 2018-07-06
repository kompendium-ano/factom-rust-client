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


}