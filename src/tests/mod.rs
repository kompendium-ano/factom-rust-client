use super::*; 
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
 

//const EC_TXID: &str = "f04279998823bfa545dc9feb7bd2de77f57a2e226686309b19c4c721bf7cfcc2";

const EC_ADDRESS: &str = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
const FCT_PRIV_ADDRESS: &str = "Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK";
const FCT_PUB_ADDRESS: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
const FCT_PUB_ADDRESS2: &str = "FA3Jto7SVF4o9VBNcgWWZ74ReSmi5x1aAweXf11mM2RrAfsHsNAq";
const TXID: &str = "21fc64855771f2ee12da2a85b1aa0108007ed3a566425f3eaec7c8c7d2db6c6d";
const CHAINID: &str = "9dec48601fba6ddb4bcea12066ba0f2b2467f89c788c5a243eb253c3de0f815b";
const ENTRYHASH: &str = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
const DBLOCK_KEYMR: &str = "5b372f4622c682c984dc922983d0c769db33c376d107c74e8023446029592011";
const ABLOCK_KEYMR: &str = "9f9b2d68e7f018a272e9331765ac8d353c7f58c6f18685405b5286353b58daee";
const FBLOCK_KEYMR: &str = "aaaf4db6c1f5b716df0d63dcf9605f599d9e41eb635d8ba3e9ddfbe697ec426c";
const EBLOCK_KEYMR: &str = "1df118c1293858d1111762d6a0df92b12231c72deb14b53bfffc09b867db1f3b";
const ECBLOCK_KEYMR: &str = "9b9e5b67b17f2e2d3d8405ea5fc227f6bf61fcc8c2422b36b11a7fce97018521";

const HOST: &str ="192.168.121.131";



fn random_string(len: usize)-> String {
    let mut rng = thread_rng();
    iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(len)
            .collect()
}



fn walletd()-> Walletd{
    let mut walletd = Walletd::new();
    walletd.host(HOST);
    walletd
}

fn factomd()-> Factomd{
    let mut factomd = Factomd::new();
    factomd.host(HOST);
    factomd
}



fn get_result<F, R, E>(fut: F)-> Result<R, E>
    where
        F: Send + 'static + Future<Item = R, Error = E>,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a tokio runtime");
        runtime.block_on(fut)
    }

fn error_check(response: Response){
    let result = response.result;
    if let Outcome::error(err) = &result{
        // Prints json responses using `cargo test -- --nocapture`
        // dbg!(&result);
        panic!("{:?}", err)
    }
}

// Daemon

#[test]
fn ablock_by_height() {
    let query = factomd()
                .ablock_by_height(2)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn ack() {
    let hash = ENTRYHASH;
    let tx_type = "f";
    let query = factomd()
                .ack(hash, tx_type, None)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn admin_block() {
    let keymr = ABLOCK_KEYMR;
    let query = factomd()
                .admin_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn chain_head() {
    let chainid = CHAINID;
    let query = factomd()
                .chain_head(chainid)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn current_minute() {
    let query = factomd()
                .current_minute()
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn dblock_by_height() {
    let query = factomd()
                .dblock_by_height(2)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn directory_block() {
    let keymr = DBLOCK_KEYMR;
    let query = factomd()
                .directory_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn directory_block_head() {
    let query = factomd()
                .directory_block_head()
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn ecblock_by_height() {
    let query = factomd()
                .ecblock_by_height(2)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}


#[test]
fn entry() {
    let hash = ENTRYHASH;
    let query = factomd().entry(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entryblock() {
    let keymr = EBLOCK_KEYMR;
    let query = factomd()
                .entry_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entry_credit_balance() {
    let address = EC_ADDRESS;
    let query = factomd()
                .entry_credit_balance(address)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entry_credit_block() {
    let keymr = ECBLOCK_KEYMR;
    let query = factomd()
                .entry_credit_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entry_credit_rate() {
    let query = factomd()
                .entry_credit_rate()
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn factoid_balance() {
    let address = FCT_PUB_ADDRESS;
    let query = factomd()
                .factoid_balance(address)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn factoid_block() {
    let keymr = FBLOCK_KEYMR;
    let query = factomd()
                .factoid_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn factoid_submit() {
    let tx = "0201565d109233010100b0a0e100646f3e8750c550e4582eca5047546ffef89c13a175985e320232bacac81cc428afd7c200ce7b98bfdae90f942bc1fe88c3dd44d8f4c81f4eeb88a5602da05abc82ffdb5301718b5edd2914acc2e4677f336c1a32736e5e9bde13663e6413894f57ec272e28dc1908f98b79df30005a99df3c5caf362722e56eb0e394d20d61d34ff66c079afad1d09eee21dcd4ddaafbb65aacea4d5c1afcd086377d77172f15b3aa32250a";
    let query = factomd()
                .factoid_submit(tx)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn fblock_by_height() {
    let query = factomd()
                .fblock_by_height(1)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn heights() {
    let query = factomd().heights()
                            .map(|response| response).map_err(|err| err);
    let result = get_result(query);
    let response = result.unwrap();
    error_check(response);   
}


#[test]
fn multiple_ec_balances() {
    let addresses: Vec<&str> = vec![EC_ADDRESS];
    let query = factomd().multiple_ec_balances(addresses)
                            .map(|response| response).map_err(|err| err);
    let result = get_result(query);
    let response = result.unwrap();
    error_check(response);   
}


#[test]
fn multiple_fct_balances() {
    let addresses: Vec<&str> = vec![FCT_PUB_ADDRESS];
    let query = factomd().multiple_fct_balances(addresses)
                            .map(|response| response).map_err(|err| err);
    let result = get_result(query);
    let response = result.unwrap();
    error_check(response);   
}

#[test]
fn pending_entries() {
    let query = factomd().pending_entries()
                            .map(|response| response).map_err(|err| err);
    let result = get_result(query);
    let response = result.unwrap();
    error_check(response);   
}


#[test]
fn pending_transactions() {
    let query = factomd().pending_transactions(None)
                            .map(|response| response).map_err(|err| err);
    let result = get_result(query);
    let response = result.unwrap();
    error_check(response);   
}


#[test]
fn factomd_properties() {
    let query = factomd().properties()
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);   
}

#[test]
fn raw_data() {
    let hash = ENTRYHASH;
    let query = factomd().raw_data(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);   
}

#[test]
fn receipt() {
    let hash = ENTRYHASH;
    let query = factomd().receipt(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn transaction() {
    let hash = TXID;
    let query = factomd().transaction(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

