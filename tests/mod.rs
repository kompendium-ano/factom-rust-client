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

// #[test]
// fn holding_queue(){
//   let api = Factom::open_node();
//   let query = api.holding_queue();
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   // assert_eq!(response.result.chainhead, CHAIN_HEAD);
// }


// Entry Module
#[test]
fn entry(){
  let api = Factom::open_node();
  let query = api.entry(ENTRY_HASH);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.chainid, CHAIN_ID);
}