use super::*; 





// Daemon

fn factomd()-> Factomd{
    let mut factomd = Factomd::new();
    factomd.port(443);
    factomd.https();
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
    if let Outcome::error(err) = result{
        panic!("{:?}", err)
    }
}

#[test]
fn ablock_by_height() {
    let query = factomd()
                .ablock_by_height(14460)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn ack() {
    let hash = "ca541bbf22bd1e4cb1c16d08694bc1a6a1db4a8ad2cc0452db704966c8de7b6f";
    let tx_type = "f";
    let query = factomd()
                .ack(hash, tx_type, None)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn admin_block() {
    let keymr = "bb36d9a75ab887c21a365cc3464806e769f88b50326a55bf97e4b7190975bf65";
    let query = factomd()
                .admin_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn chain_head() {
    let chainid = "fbf1bb7ffa4ec0bbb0f7dc18cbeb47514102c2eb38fd1f985be3254156b28677";
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
                .dblock_by_height(14460)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn directory_block() {
    let keymr = "ba82854fae10bf41c95a1f97368a8ae7876d4f957246fc773c0b2f700db1c6b0";
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
                .ecblock_by_height(14460)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}


#[test]
fn entry() {
    let hash = "885acfbdfa2a7b51e6c7562384929984dbcfae9a6d0e489b7a7593c26e429738";
    let query = factomd().entry(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entryblock() {
    let keymr = "9b56d2fc48759d74d8430c87f792146951acd85486cec7bad3f4e5cb3a3e6008";
    let query = factomd()
                .entry_block(keymr)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entry_credit_balance() {
    let address = "EC3MAHiZyfuEb5fZP2fSp2gXMv8WemhQEUFXyQ2f2HjSkYx7xY1S";
    let query = factomd()
                .entry_credit_balance(address)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn entry_credit_block() {
    let keymr = "2050b16701f29238d6b99bcf3fb0ca55d6d884139601f06691fc370cda659d60";
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
    let address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
    let query = factomd()
                .factoid_balance(address)
                .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn factoid_block() {
    let keymr = "1df843ee64f4b139047617a2df1007ea4470fabd097ddf87acabc39813f71480";
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
                .fblock_by_height(14460)
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
    let addresses: Vec<&str> = vec!["EC293AbTn3VScgC2m86xTh2kFKAMNnkgoLdXgywpPa66Jacom5ya"];
    let query = factomd().multiple_ec_balances(addresses)
                            .map(|response| response).map_err(|err| err);
    let result = get_result(query);
    let response = result.unwrap();
    error_check(response);   
}


#[test]
fn multiple_fct_balances() {
    let addresses: Vec<&str> = vec!["FA3uMAv9htC5y5u3ayzxNQKZNDpgrJVf49kJSKdVNxcYoNBbSLXc"];
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
fn properties() {
    let query = factomd().properties()
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);   
}

#[test]
fn raw_data() {
    let hash = "8cd3ba08e4bc9e581050f075fae9a4d97e56f6bd5fe17c39e7c228c54139f359";
    let query = factomd().raw_data(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);   
}

#[test]
fn receipt() {
    let hash = "8cd3ba08e4bc9e581050f075fae9a4d97e56f6bd5fe17c39e7c228c54139f359";
    let query = factomd().receipt(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

#[test]
fn transaction() {
    let hash = "9542f6695fdf02ab8e7adfc058dbb566919f85fbd37ff7e83e8bb459f6f8310c";
    let query = factomd().transaction(hash)
                            .map(|response| response).map_err(|err| err);
    let response = get_result(query).unwrap();
    error_check(response);  
}

