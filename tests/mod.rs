use::factom::*;
use std::iter;
use rand::{Rng, thread_rng, distributions::Alphanumeric};

// Hopefully it doesn't need to be said but please don't use these addresses as 
// they are publicly known! They should be funded on testnet for further usage,
// as needed. Please raise an issue if they aren't or top them up yourself at:
// https://faucet.factoid.org/
const FCT_PUB: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
const FCT_PRIV: &str = "Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK";
const EC_PUB: &str = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
// const EC_PRIV: &str = "Es3LS7zYa9DSzZuUC14HDpMinehmzz61JG1XFY62rX5pVDenH8Pk";

const ABLOCK_KEYMR: &str = "072334d94450296810cb647172812a5dc7ce518d29ecab411d47494d38ca4c88";
const ABLOCK_HEIGHT: u32 = 220000;
const DBLOCK_KEYMR: &str = "90b344dabe065bcb38c90948cde8ab388e22364257f811da7e7a7e8102efc33f";
// const DBLOCK_HEIGHT: u32 = 218665;
const ECBLOCK_KEYMR: &str = "06a56a6917ff26d118b67117908a58832ad741433a3047ce4ec45f03754aeed6";
const ECBLOCK_BODYHASH: &str = "c8b7120687c6bc465a5f0fb0517c9bbaf27ee733238aef3f4b79b772a9590030";
const FBLOCK_KEYMR: &str = "32bf997a124dd31c897cca5d92648f8a1fa18626ff396a89cb34a59aaa47c5b3";
const FBLOCK_HEIGHT: u32 = 218668;
const FBLOCK_BODYMR: &str = "9886c838cd8eddfaaf809a1425d71c49539e86223d1e624dfd0d65fdc1aa8674";

//pnet chain
const CHAIN_ID: &str = "a642a8674f46696cc47fdb6b65f9c87b2a19c5ea8123b3d2f0c13b6f33a9d5ef";
const ENTRY_HASH: &str = "716526c3279184bca11fc453fa9c2ab2f4488a03c821ee107664c9052f01d733";
const RAW_DATA_HASH: &str = "0ae2ab2cf543eed52a13a5a405bded712444cc8f8b6724a00602e1c8550a4ec2";
const RAW_DATA: &str = "000caff62ea5b5aa015c706add7b2463a5be07e1f0537617f553558090f23c7f5600420040e57283e4618f13b18c2be8d14926999331ef4ab905639a82d748634201cd85ae1c22b6186a72eee3f4ae12b8f6fa9c73a8a98b5eae238ed6133424bcef062f0e7b224150494d6574686f64223a2268747470733a2f2f706f6c6f6e6965782e636f6d2f7075626c69633f636f6d6d616e643d72657475726e4f72646572426f6f6b5c753030323663757272656e6379506169723d4254435f4e58545c753030323664657074683d34222c2252657475726e44617461223a227b5c2261736b735c223a5b5b5c22302e30303030313334315c222c343437342e37323033353739345d2c5b5c22302e30303030313334325c222c363038302e39363930373133355d2c5b5c22302e30303030313334355c222c31343831342e38353833353730375d2c5b5c22302e30303030313337385c222c38303030305d5d2c5c22626964735c223a5b5b5c22302e30303030313332375c222c363032382e303333313537355d2c5b5c22302e30303030313332365c222c3236302e34333839313430335d2c5b5c22302e30303030313332355c222c3130393931352e30363731363938315d2c5b5c22302e30303030313332335c222c31323030305d5d2c5c22697346726f7a656e5c223a5c22305c227d222c2254696d657374616d70223a313435303134373830317d";

const ID_PRIV: &str = "idsec2rWrfNTD1x9HPPesA3fz8dmMNZdjmSBULHx8VTXE1J4D9icmAK";
const ID_PUB: &str = "idpub2g25nPNZ2kf6KGTjthYdHT3nykDbwEUEPyGJ52fo55SHwtAvLA";

const KOINIFY: &str = "yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow";
const KOINIFY_PUB: &str = "FA3cih2o2tjEUsnnFR4jX1tQXPpSXFwsp3rhVp6odL5PNCHWvZV1";

const TESTNET_CHAIN: &str = "72a2fa10b81a8bffde58ea206254f0eaa7928e9e09a4144efb3ba0bb7be26d52";

const ACK_HASH: &str = "e96cca381bf25f6dd4dfdf9f7009ff84ee6edaa3f47f9ccf06d2787482438f4b";
const ACK_CHAIN: &str = "f9164cd66af9d5773b4523a510b5eefb9a5e626480feeb6671ef2d17510ca300";
const ACK_COMMITTXID: &str = "4876ffeb8f95b72911b4a5115dc8a9fbb89d874db2263a75a9062f37bbbf1fa7";

const FCT_TX_ID: &str = "774ec19ff567b202ca2702b1f3411901ccae8995df46517c134ab3560100c848";
const FCT_TX_TIMESTAMP: usize = 1575574473164;

fn random_string(len: usize)-> String {
  let mut rng = thread_rng();
  iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect()
}

// Address module
#[test]
fn address(){
  let client = Factom::new();
  let import = import::import_addresses(&client, vec!(FCT_PRIV));
  fetch(import).unwrap();
  let query = factom::address::address(&client, FCT_PUB);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.secret, FCT_PRIV);
}

#[test]
fn all_addresses(){
  let client = Factom::new();
  let import = import::import_addresses(&client, vec!(FCT_PRIV));
  fetch(import).unwrap();
  let query = address::all_addresses(&client);
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
  let client = Factom::new();
  let query = generate::factoid_address(&client);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  let address = response.result.public;
  let rm_query = address::remove_address(&client, &address);
  let rm_response = fetch(rm_query).expect("Fetching Query");
  dbg!(&rm_response);
  assert!(rm_response.result.success);
}

// Balance module
#[test]
fn balance(){
  let client = Factom::testnet_node();
  let query = balance::factoid_balance(&client, FCT_PUB);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn ec_balance(){
  let client = Factom::testnet_node();
  let query = balance::entry_credit_balance(&client, EC_PUB);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn multiple_balances(){
  let client = Factom::testnet_node();
  let query = balance::multiple_fct_balances(&client, vec!(FCT_PUB));
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn multiple_ec_balances(){
  let client = Factom::testnet_node();
  let query = balance::multiple_ec_balances(&client, vec!(EC_PUB));
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

// Block module
#[test]
fn ablock_height(){
  let client = Factom::open_node();
  let query = block::ablock_by_height(&client, ABLOCK_HEIGHT);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.ablock.header.dbheight, ABLOCK_HEIGHT as usize);
}

#[test]
fn admin_block(){
  let client = Factom::open_node();
  let query = block::admin_block(&client, ABLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.ablock.header.dbheight, ABLOCK_HEIGHT as usize);
}

#[test]
fn anchors(){
  let client = Factom::open_node();
  let query = block::anchors(
                        &client, 
                        block::AnchorType::Height(ABLOCK_HEIGHT as usize)
                      );
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
}

#[test]
fn dblock_height(){
  let client = Factom::open_node();
  let query = block::dblock_by_height(&client, ABLOCK_HEIGHT);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.dblock.header);
  assert_eq!(response.result.dblock.header.dbheight, ABLOCK_HEIGHT as usize);
}

#[test]
fn directory_block(){
  let client = Factom::open_node();
  let query = block::directory_block(&client, DBLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.header);
  assert_eq!(response.result.header.timestamp, 1573694640);
}

#[test]
fn directory_block_head(){
  let client = Factom::open_node();
  let query = block::directory_block_head(&client);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.error.code, 0);
}

#[test]
fn ecblock_height(){
  let client = Factom::open_node();
  let query = block::ecblock_by_height(&client, 218668);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.ecblock.header);
  assert_eq!(response.result.ecblock.header.bodyhash, ECBLOCK_BODYHASH);
}

#[test]
fn ec_block(){
  let client = Factom::open_node();
  let query = block::entry_credit_block(&client, ECBLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.ecblock.header);
  assert_eq!(response.result.ecblock.header.bodyhash, ECBLOCK_BODYHASH);
}

#[test]
fn fct_block(){
  let client = Factom::open_node();
  let query = block::factoid_block(&client, FBLOCK_KEYMR);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.fblock);
  assert_eq!(response.result.fblock.bodymr, FBLOCK_BODYMR);
}

#[test]
fn fctblock_height(){
  let client = Factom::open_node();
  let query = block::fblock_by_height(&client, FBLOCK_HEIGHT);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response.result.fblock);
  assert_eq!(response.result.fblock.bodymr, FBLOCK_BODYMR);
}

#[test]
fn chain_head(){
  let client = Factom::open_node();
  let query = chain::chain_head(&client, CHAIN_ID);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert!(response.success());
}

// factom-walletd needs to be run with a command to testnet open node
// > factom-walletd -s https://dev.factomd.net/
#[test]
fn create_chain(){
  let client = Factom::testnet_node();
  let rand_ext_id = &random_string(12);
  let ext_ids = vec!("Api Client", "Test Chain", rand_ext_id);
  let content = random_string(32);
  
  let compose_query = compose::compose_chain(&client, ext_ids, &content, EC_PUB);
  let compose_response = fetch(compose_query).expect("Fetching Query");
  dbg!(&compose_response);
  
  let commit = compose_response.result.commit.params.message;
  let commit_query = chain::commit_chain(&client, &commit);
  let commit_response = fetch(commit_query).expect("Fetching Query");
  dbg!(&commit_response);
  let reveal = compose_response.result.reveal.params.entry;
  let reveal_query = chain::reveal_chain(&client, &reveal);
  let reveal_response = fetch(reveal_query).expect("Fetching Query");
  dbg!(&reveal_response);
  assert!(!reveal_response.is_err());
}

#[test]
fn create_entry(){
  let client = Factom::testnet_node();
  let rand_ext_id = &random_string(12);
  let ext_ids = vec!("Api Client", "Test Entry", rand_ext_id);
  let content = random_string(32);
  
  let compose_query = compose::compose_entry(&client,
                                            TESTNET_CHAIN,
                                            ext_ids, 
                                            &content, 
                                            EC_PUB);
  let compose_response = fetch(compose_query).expect("Fetching Query");
  dbg!(&compose_response);
  assert!(!compose_response.is_err());

  let commit = compose_response.result.commit.params.message;
  let commit_query = entry::commit_entry(&client, &commit);
  let commit_response = fetch(commit_query).expect("Fetching Query");
  dbg!(&commit_response);
  assert!(!commit_response.is_err());
  
  let reveal = compose_response.result.reveal.params.entry;
  let reveal_query = entry::reveal_entry(&client, &reveal);
  let reveal_response = fetch(reveal_query).expect("Fetching Query");
  dbg!(&reveal_response);
  assert!(!reveal_response.is_err());
}

// Debug Module
// Open_node doesn't expose debug functionality, these can all be tested with a 
//local factomd  node running a debug network with a command such as factomd -network=LOCAL
// #[test]
// fn holding_queue(){
//   let client = Factom::new();
//   let query = debug::holding_queue(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn network_info(){
//   let client = Factom::new();
//   let query = debug::network_info(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn predictive_fer(){
//   let client = Factom::new();
//   let query = debug::predictive_fer(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn audit_servers(){
//   let client = Factom::new();
//   let query = debug::audit_servers(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn federated_servers(){
//   let client = Factom::new();
//   let query = debug::federated_servers(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn configuration(){
//   let client = Factom::new();
//   let query = debug::configuration(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn process_list(){
//   let client = Factom::new();
//   let query = debug::process_list(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn authorities(){
//   let client = Factom::new();
//   let query = debug::authorities(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn reload_configuration(){
//   let client = Factom::new();
//   let query = debug::reload_configuration(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn drop_rate(){
//   let client = Factom::new();
//   let query = debug::network_info(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn set_drop_rate(){
//   let client = Factom::new();
//   let query = debug::set_drop_rate(&client, 10);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn delay(){
//   let client = Factom::new();
//   let query = debug::delay(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn set_delay(){
//   let client = Factom::new();
//   let query = debug::set_delay(&client, 10);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn summary(){
//   let client = Factom::new();
//   let query = debug::summary(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Debug tests only work with a local factomd node
// #[test]
// fn messages(){
//   let client = Factom::new();
//   let query = debug::messages(&client);
//   let response = fetch(query).expect("Fetching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

// Entry Module
#[test]
fn entry(){
  let client = Factom::open_node();
  let query = entry::entry(&client, ENTRY_HASH);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert_eq!(response.result.chainid, CHAIN_ID);
}

#[test]
fn raw_data(){
  let client = Factom::open_node();
  let query = entry::raw_data(&client, RAW_DATA_HASH);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert_eq!(response.result.data, RAW_DATA);  
}

#[test]
fn pending_entries(){
  let client = Factom::open_node();
  let query = entry::pending_entries(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());  
}

// factomd module
#[test]
fn current_minute(){
  let client = Factom::open_node();
  let query = factomd::current_minute(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn diagnostics(){
  let client = Factom::open_node();
  let query = factomd::diagnostics(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn entry_credit_rate(){
  let client = Factom::open_node();
  let query = factomd::entry_credit_rate(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn heights(){
  let client = Factom::open_node();
  let query = factomd::heights(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn properties(){
  let client = Factom::open_node();
  let query = factomd::properties(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn receipt(){
  let client = Factom::open_node();
  let query = factomd::receipt(&client, ENTRY_HASH, false);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

// generate module
#[test]
fn generate_factoid_address(){
  let client = Factom::new();
  let query = generate::factoid_address(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn generate_ec_address(){
  let client = Factom::new();
  let query = generate::ec_address(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn generate_identity_key(){
  let client = Factom::new();
  let query = generate::identity_key(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

// identity module
#[test]
fn all_id_keys(){
  let client = Factom::new();
  let query = identity::all_id_keys(&client);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

// #[test]
// fn active_id_keys(){
//   let client = Factom::open_node();
//   let query = identity::active_id_keys(&client, ID_CHAIN, Some(ID_HEIGHT));
//   let response = fetch(query).expect("Fectching Query");
//   dbg!(&response);
//   assert!(response.success());
// }

#[test]
fn remove_id_key(){
  let client = Factom::new();
  let gen_query = generate::identity_key(&client);
  let response = fetch(gen_query).expect("Fectching Query");
  let key = response.result.public;
  let rm_query = identity::remove_id_key(&client, &key);
  let response = fetch(rm_query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.result.success)
}

#[test]
fn id_key(){
  let client = Factom::new();
  let gen_query = generate::identity_key(&client);
  let response = fetch(gen_query).expect("Fectching Query");
  let key = response.result.public;
  let key_query = identity::id_key(&client, &key);
  let response = fetch(key_query).expect("Fectching Query");
  dbg!(&response);
  let rm_query = identity::remove_id_key(&client, &key);
  fetch(rm_query).expect("Fectching Query");
  assert_eq!(response.result.public, key.clone());
}

// import module
#[test]
fn import_addresses(){
  let client = Factom::new();
  let query = import::import_addresses(&client, vec!(FCT_PRIV));
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
  let query = address::remove_address(&client, FCT_PUB);
  fetch(query).expect("Fectching Query");
}

#[test]
fn import_id_keys(){
  let client = Factom::new();
  let query = import::import_identity_keys(&client, vec!(ID_PRIV));
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
  let query = identity::remove_id_key(&client, ID_PUB);
  fetch(query).expect("Fectching Query");
}

#[test]
fn import_koinify(){
  let client = Factom::new();
  let query = import::import_koinify(&client, KOINIFY);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
  let query = address::remove_address(&client, KOINIFY_PUB);
  fetch(query).expect("Fectching Query");
}

// txmodule
#[test]
fn ack(){
  let client = Factom::open_node();
  let query = tx::ec_ack(&client, ACK_HASH, ACK_CHAIN, None);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert_eq!(response.result.committxid, ACK_COMMITTXID);
}

#[test]
fn transaction(){
  let client = Factom::open_node();
  let query = tx::transaction(&client, FCT_TX_ID);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert_eq!(response.result.factoidtransaction.millitimestamp, FCT_TX_TIMESTAMP);
}

#[test]
fn pending_transactions(){
  let client = Factom::open_node();
  let query = tx::pending_transactions(&client, None);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn new_and_delete_transaction(){
  let client = Factom::new();
  let tx_name = "api-client-test-tx";
  let query = tx::new_transaction(&client, tx_name);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
  let query = tx::delete_transaction(&client, tx_name);
  let response = fetch(query).expect("Fectching Query");
  dbg!(&response);
  assert!(response.success());
}

// Walletd module
#[test]
fn wallet_backup() {
  let client = Factom::new();
  let query = walletd::wallet_backup(&client);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn wallet_height() {
  let client = Factom::new();
  let query = walletd::wallet_height(&client);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert!(response.success());
}

#[test]
fn wallet_properties() {
  let client = Factom::new();
  let query = walletd::wallet_properties(&client);
  let response = fetch(query).expect("Fetching Query");
  dbg!(&response);
  assert!(response.success());
}
