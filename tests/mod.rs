use ::factom::{Factom, requests::fetch};
// use std::iter;
// use rand::{Rng, thread_rng};
// use rand::distributions::Alphanumeric;


// Hopefully it doesn't need to be said but please don't use these addresses as 
// they are publicly known! They should be funded on testnet for further usage,
// as needed. Please raise an issue if they aren't or top them up at:
// https://faucet.factoid.org/
const FCT_PUB: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
const FCT_PRIV: &str = "Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK";
const EC_PUB: &str = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
// const EC_PRIV: &str = "Es3LS7zYa9DSzZuUC14HDpMinehmzz61JG1XFY62rX5pVDenH8Pk";
const ABLOCK_KEYMR: &str = "cda746980e7b562d213a871f85a3caf54320a4284b2762961edf62ddf63ef131";
const ABLOCK_HEIGHT: u32 = 105098;
const DBLOCK_KEYMR: &str = "90b344dabe065bcb38c90948cde8ab388e22364257f811da7e7a7e8102efc33f";
// const DBLOCK_HEIGHT: u32 = 218665;
const ECBLOCK_KEYMR: &str = "06a56a6917ff26d118b67117908a58832ad741433a3047ce4ec45f03754aeed6";
const ECBLOCK_BODYHASH: &str = "c8b7120687c6bc465a5f0fb0517c9bbaf27ee733238aef3f4b79b772a9590030";
const FBLOCK_KEYMR: &str = "32bf997a124dd31c897cca5d92648f8a1fa18626ff396a89cb34a59aaa47c5b3";
const FBLOCK_HEIGHT: u32 = 218668;
const FBLOCK_BODYMR: &str = "9886c838cd8eddfaaf809a1425d71c49539e86223d1e624dfd0d65fdc1aa8674";
//pnet chain
const CHAIN_ID: &str = "a642a8674f46696cc47fdb6b65f9c87b2a19c5ea8123b3d2f0c13b6f33a9d5ef";
const CHAIN_HEAD: &str = "a1e08aca2848999b0c28eaaddf13d34109f4c18bd46fb15b02e50566ad91a38f";
const ENTRY_HASH: &str = "716526c3279184bca11fc453fa9c2ab2f4488a03c821ee107664c9052f01d733";

// Address module
#[test]
fn address(){
  let api = Factom::new();
  let query = api.address(FCT_PUB);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.secret, FCT_PRIV);
}

#[test]
fn all_addresses(){
  let api = Factom::new();
  let query = api.all_addresses();
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  let address_present = response
                        .result
                        .addresses
                        .iter()
                        .find(|&x| x.public == FCT_PUB);
  assert!(address_present.is_some());
}

#[test]
fn rm_address(){
  let api = Factom::new();
  let query = api.clone().generate_factoid_address();
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  let address = response.result.public;
  let rm_query = api.remove_address(&address);
  let rm_response = fetch(rm_query).expect("Fetching Query");
  dbg!(&rm_response);
  assert!(rm_response.result.success);
}

// Balance module
#[test]
fn balance(){
  let api = Factom::testnet_node();
  let query = api.factoid_balance(FCT_PUB);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn ec_balance(){
  let api = Factom::testnet_node();
  let query = api.entry_credit_balance(EC_PUB);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn multiple_balances(){
  let api = Factom::testnet_node();
  let query = api.multiple_fct_balances(vec!(FCT_PUB));
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn multiple_ec_balances(){
  let api = Factom::testnet_node();
  let query = api.multiple_ec_balances(vec!(EC_PUB));
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

// Block module
#[test]
fn ablock_height(){
  let api = Factom::testnet_node();
  let query = api.ablock_by_height(ABLOCK_HEIGHT as u32);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.ablock.header.dbheight, ABLOCK_HEIGHT as usize);
}

#[test]
fn admin_block(){
  let api = Factom::open_node();
  let query = api.admin_block(ABLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.ablock.header.dbheight, ABLOCK_HEIGHT as usize);
}

/// Anchors json response returns either boolean false or hashmap, implementation
/// currently is unable to handle a false return
#[test]
fn anchors(){
  let api = Factom::open_node();
  let query = api.anchors(factom::block::AnchorType::Height(ABLOCK_HEIGHT as usize));
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
}

#[test]
fn dblock_height(){
  let api = Factom::testnet_node();
  let query = api.dblock_by_height(ABLOCK_HEIGHT);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.dblock.header);
  assert_eq!(response.result.dblock.header.dbheight, ABLOCK_HEIGHT as usize);
}

#[test]
fn directory_block(){
  let api = Factom::open_node();
  let query = api.directory_block(DBLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.header);
  assert_eq!(response.result.header.timestamp, 1573694640);
}

#[test]
fn directory_block_head(){
  let api = Factom::open_node();
  let query = api.directory_block_head();
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn ecblock_height(){
  let api = Factom::open_node();
  let query = api.ecblock_by_height(218668);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.ecblock.header);
  assert_eq!(response.result.ecblock.header.bodyhash, ECBLOCK_BODYHASH);
}

#[test]
fn ec_block(){
  let api = Factom::open_node();
  let query = api.entry_credit_block(ECBLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.ecblock.header);
  assert_eq!(response.result.ecblock.header.bodyhash, ECBLOCK_BODYHASH);
}

#[test]
fn fct_block(){
  let api = Factom::open_node();
  let query = api.factoid_block(FBLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.fblock);
  assert_eq!(response.result.fblock.bodymr, FBLOCK_BODYMR);
}

#[test]
fn fctblock_height(){
  let api = Factom::open_node();
  let query = api.fblock_by_height(FBLOCK_HEIGHT);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.fblock);
  assert_eq!(response.result.fblock.bodymr, FBLOCK_BODYMR);
}

#[test]
fn chain_head(){
  let api = Factom::open_node();
  let query = api.chain_head(CHAIN_ID);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.chainhead, CHAIN_HEAD);
}

// TODO:
// #[test]
// fn commit_chain

// #[test]
// fn reveal_chain


// Debug Module
// Cloudflare errors using open node, these can be tested with a local factomd
// node
#[test]
fn holding_queue(){
  let api = Factom::open_node();
  let query = api.holding_queue();
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  // assert_eq!(response.result.chainhead, CHAIN_HEAD);
}


// Entry Module
#[test]
fn entry(){
  let api = Factom::open_node();
  let query = api.entry(ENTRY_HASH);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.chainid, CHAIN_ID);
}

// // const HOST: &str ="localhost";
// const EC_ADDRESS: &str = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
// const FCT_PUB_ADDRESS: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
// const FCT_PUB_ADDRESS2: &str = "FA3Jto7SVF4o9VBNcgWWZ74ReSmi5x1aAweXf11mM2RrAfsHsNAq";
// const TXID: &str = "21fc64855771f2ee12da2a85b1aa0108007ed3a566425f3eaec7c8c7d2db6c6d";
// const CHAINID: &str = "9dec48601fba6ddb4bcea12066ba0f2b2467f89c788c5a243eb253c3de0f815b";
// const ENTRYHASH: &str = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
// const DBLOCK_KEYMR: &str = "5b372f4622c682c984dc922983d0c769db33c376d107c74e8023446029592011";
// const ABLOCK_KEYMR: &str = "9f9b2d68e7f018a272e9331765ac8d353c7f58c6f18685405b5286353b58daee";
// const FBLOCK_KEYMR: &str = "aaaf4db6c1f5b716df0d63dcf9605f599d9e41eb635d8ba3e9ddfbe697ec426c";
// const EBLOCK_KEYMR: &str = "1df118c1293858d1111762d6a0df92b12231c72deb14b53bfffc09b867db1f3b";
// const ECBLOCK_KEYMR: &str = "9b9e5b6b17f2e2d3d8405ea5fc227f6bf61fcc8c2422b36b11a7fce97018521";

// fn random_string(len: usize)-> String {
//   let mut rng = thread_rng();
//   iter::repeat(())
//       .map(|()| rng.sample(Alphanumeric))
//       .take(len)
//       .collect()
// }

// fn factom()-> Factom{
//   Factom::new()
// }

// fn error_check(response: Response){
//   let result = response.result;
//   if let Outcome::error(err) = &result{
//     panic!("{:?}", err)
//   }
// }

// // Will fail if not using local factomd
// #[test]
// fn factom_new(){
//   let api = factom();
//   let query = api.properties();
//   let response = fetch(query).expect("Unable to fetch request");
//   error_check(response);
// }

// // Daemon
// #[test]
// fn ablock_by_height() {
//   let query = factom()
//         .ablock_by_height(2)
//         .map(|response| response)
//         .map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn ack() {
//   let hash = ENTRYHASH;
//   let tx_type = "f";
//   let query = factom()
//                   .ack(hash, tx_type, None)
//                   .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn admin_block() {
//   let keymr = ABLOCK_KEYMR;
//   let query = factom()
//               .admin_block(keymr)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn anchors() {
//   let block = 100;
//   let query = factom()
//               .anchors(block::AnchorType::Height(block))
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn chain_head() {
//   let query = factom()
//               .chain_head(CHAINID)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn current_minute() {
//   let query = factom()
//               .current_minute()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn diagnostics() {
//   let query = factom()
//               .diagnostics()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn dblock_by_height() {
//   let query = factom()
//               .dblock_by_height(2)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn directory_block() {
//   let keymr = DBLOCK_KEYMR;
//   let query = factom()
//               .directory_block(keymr)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn directory_block_head() {
//   let query = factom()
//               .directory_block_head()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn ecblock_by_height() {
//   let query = factom()
//               .ecblock_by_height(2)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }


// #[test]
// fn entry() {
//   let hash = ENTRYHASH;
//   let query = factom().entry(hash)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn entryblock() {
//   let keymr = EBLOCK_KEYMR;
//   let query = factom()
//               .entry_block(keymr)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn entry_credit_balance() {
//   let address = EC_ADDRESS;
//   let query = factom()
//               .entry_credit_balance(address)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn entry_credit_block() {
//   let keymr = ECBLOCK_KEYMR;
//   let query = factom()
//               .entry_credit_block(keymr)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn entry_credit_rate() {
//   let query = factom()
//               .entry_credit_rate()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn factoid_balance() {
//   let address = FCT_PUB_ADDRESS;
//   let query = factom()
//               .factoid_balance(address)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn factoid_block() {
//   let keymr = FBLOCK_KEYMR;
//   let query = factom()
//               .factoid_block(keymr)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn factoid_submit() {
//   let tx = "0201565d109233010100b0a0e100646f3e8750c550e4582eca5047546ffef89c13a175985e320232bacac81cc428afd7c200ce7b98bfdae90f942bc1fe88c3dd44d8f4c81f4eeb88a5602da05abc82ffdb5301718b5edd2914acc2e4677f336c1a32736e5e9bde13663e6413894f57ec272e28dc1908f98b79df30005a99df3c5caf362722e56eb0e394d20d61d34ff66c079afad1d09eee21dcd4ddaafbb65aacea4d5c1afcd086377d77172f15b3aa32250a";
//   let query = factom()
//               .factoid_submit(tx)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn fblock_by_height() {
//   let query = factom()
//               .fblock_by_height(1)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn heights() {
//   let query = factom().heights()
//               .map(|response| response).map_err(|err| err);
//   let result = fetch(query);
//   let response = result.unwrap();
//   error_check(response);   
// }

// #[test]
// fn multiple_ec_balances() {
//   let addresses: Vec<&str> = vec![EC_ADDRESS];
//   let query = factom().multiple_ec_balances(addresses)
//               .map(|response| response).map_err(|err| err);
//   let result = fetch(query);
//   let response = result.unwrap();
//   error_check(response);   
// }

// #[test]
// fn multiple_fct_balances() {
//   let addresses: Vec<&str> = vec![FCT_PUB_ADDRESS];
//   let query = factom().multiple_fct_balances(addresses)
//               .map(|response| response).map_err(|err| err);
//   let result = fetch(query);
//   let response = result.unwrap();
//   error_check(response);   
// }

// #[test]
// fn pending_entries() {
//   let query = factom().pending_entries()
//               .map(|response| response).map_err(|err| err);
//   let result = fetch(query);
//   let response = result.unwrap();
//   error_check(response);   
// }

// #[test]
// fn pending_transactions() {
//   let query = factom().pending_transactions(None)
//               .map(|response| response).map_err(|err| err);
//   let result = fetch(query);
//   let response = result.unwrap();
//   error_check(response);   
// }

// #[test]
// fn factomd_properties() {
//   let query = factom().properties()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);   
// }

// #[test]
// fn raw_data() {
//   let hash = ENTRYHASH;
//   let query = factom().raw_data(hash)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);   
// }

// #[test]
// fn receipt() {
//   let hash = ENTRYHASH;
//   let query = factom().receipt(hash)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn transaction() {
//   let hash = TXID;
//   let query = factom().transaction(hash)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// // Walletd
// #[test]
// fn address() {
//   let query = factom()
//               .address(FCT_PUB_ADDRESS)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn all_addresses() {
//   let query = factom()
//                 .all_addresses()
//                 .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn compose_chain() {
//   let extids = vec!("Cargo Test", "test harness");
//   let content = "Here be the content";
//   let query = factom()
//         .compose_chain(extids, content, EC_ADDRESS)
//         .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn compose_entry() {
//   let extids = vec!("entry testing");
//   let content = "Even more content";
//   let query = factom()
//               .compose_entry(CHAINID ,extids, content, EC_ADDRESS)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn full_transaction(){
//   let txname = random_string(8);
//   let new_response = fetch(factom().new_transaction(&txname)
//                         .map(|res| res)
//                         .map_err(|err| err)).unwrap();
//   error_check(new_response);

//   let input_response = fetch(factom()
//                         .add_input(&txname, FCT_PUB_ADDRESS, 2_000_000)
//                         .map(|res| res)
//                         .map_err(|err| err))
//                         .unwrap();
//   error_check(input_response);

//   let output_response = fetch(factom()
//                     .add_output(&txname, FCT_PUB_ADDRESS2, 2_000_000)
//                     .map(|res| res)
//                     .map_err(|err| err))
//                     .unwrap();
//   error_check(output_response);

//   let subfee_response = fetch(factom()
//                               .sub_fee(&txname, FCT_PUB_ADDRESS2)
//                               .map(|res| res)
//                               .map_err(|err| err))
//                               .unwrap();

//   error_check(subfee_response);

//   let sign_response = fetch(factom()
//                             .sign_transaction(&txname)
//                             .map(|res| res)
//                             .map_err(|err| err))
//                             .unwrap();
//   error_check(sign_response);

//   let query = factom()
//                 .compose_transaction(&txname)
//                 .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  

// }

// #[test]
// fn delete_transaction() {
//   let txname = random_string(8);
//   fetch(factom().new_transaction(&txname)
//                 .map(|res| res)
//                 .map_err(|err| err)).unwrap();
//   let query = factom()
//               .delete_transaction(&txname)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn generate_ec_address() {
//   let query = factom()
//               .generate_ec_address()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn generate_factoid_address() {
//   let query = factom()
//               .generate_factoid_address()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn get_height() {
//   let query = factom()
//               .heights()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn import_addresses() {
//   let addresses = vec!(FCT_PRIV_ADDRESS);
//   let query = factom()
//               .import_addresses(addresses)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn new_transaction() {
//   let txname = random_string(8);
//   let query = factom()
//               .new_transaction(&txname)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn walletd_properties() {
//   let query = factom()
//               .properties()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }


// #[test]
// fn tmp_transactions() {
//   let query = factom()
//               .tmp_transactions()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn transactions_range() {
//   let tx = tx::SearchBy::Range(1,2);
//   let query = factom()
//               .transactions(tx)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn transactions_address() {
//   let tx = tx::SearchBy::Address(FCT_PUB_ADDRESS);
//   let query = factom()
//               .transactions(tx)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn transactions_txid() {
//   let tx = tx::SearchBy::Txid(TXID);
//   let query = factom()
//               .transactions(tx)
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn wallet_backup() {
//   let query = factom()
//               .wallet_backup()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }

// #[test]
// fn wallet_balances() {
//   let query = factom()
//               .wallet_balances()
//               .map(|response| response).map_err(|err| err);
//   let response = fetch(query).unwrap();
//   error_check(response);  
// }