use super::*;

impl Factom {

/**
Show current holding messages in the queue.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .holding_queue()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn holding_queue(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("holding-queue", HashMap::new())
  }

/**
Get information on the current network factomd is connected to (TEST, MAIN, etc)


#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .network_info()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn network_info(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("network-info", HashMap::new())
  }

/**
Get the predicted future entry credit rate.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .predictive_fer()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn predictive_fer(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("predictive-fer", HashMap::new())
  }

/**
Get a list of the current network audit servers along with their information.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .audit_servers()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn audit_servers(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("audit-servers", HashMap::new())
  }

/**
Get a list of the current network federated servers along with their information.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .federated_servers()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn federated_servers(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("federated-servers", HashMap::new())
  }

/**
Get the current configuration state from factomd.conf.

NOTE: If a tag is commented out, this call will return the default value for it.
E.g: In the Example Response “ExchangeRate” is set to “0”. factomd.config default
 does not have an “ExchangeRate” tag. That is why it is set to “0”.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .configuration()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn configuration(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("configuration", HashMap::new())
  }

/**

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .process_list()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn process_list(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("process-list", HashMap::new())
  }

/**
List of authority servers in the management chain.
Get the process list known to the current factomd instance.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .authorities()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn authorities(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("authorities", HashMap::new())
  }

/**
Causes factomd to re-read the configuration from the config file. Note: This 
may cause consensus problems and could be impractical even in testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .reload_configuration()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn reload_configuration(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("reload-configuration", HashMap::new())
  }

/**
Get the current package drop rate for network testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .drop_rate()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn drop_rate(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("drop-rate", HashMap::new())
  }

/**
Change the network drop rate for testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .set_drop_rate(10)
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn set_drop_rate(self, drop_rate: usize)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("DropRate".to_string(), json!(drop_rate)); 
    self.debug_call("set-drop-rate", params)
  }

/**
Get the current msg delay time for network testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .delay()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn delay(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("delay", HashMap::new())
  }

/**
Set the current msg delay time for network testing.
#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .set_delay(10)
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn set_delay(self, delay: usize)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("Delay".to_string(), json!(delay)); 
    self.debug_call("set-delay", params)
  }

  /**
Get the nodes summary string.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .summary()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn summary(self)-> impl Future<Item=Response, Error=FetchError>{ 
    self.debug_call("summary", HashMap::new())
  }

  /**
Show current holding messages in the queue.
#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .messages()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn messages(self)-> impl Future<Item=Response, Error=FetchError>{ 
    self.debug_call("messages", HashMap::new())
  }
}

/// holding-queue function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HoldingQueue {
  #[serde(rename = "NetworkNumber")]
  messages: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NetworkInfo {
  #[serde(rename = "NetworkNumber")]
  network_number: i64,
  #[serde(rename = "NetworkName")]
  network_name: String,
  #[serde(rename = "NetworkID")]
  network_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PredictiveFER {
    #[serde(rename = "PredictiveFER")]
    predictive_fer: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AuditServers {
    #[serde(rename = "AuditServers")]
    audit_servers: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FederatedServers {
    #[serde(rename = "FederatedServers")]
    federated_servers: Vec<FederatedServer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FederatedServer {
    #[serde(rename = "ChainID")]
    chain_id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Online")]
    online: bool,
    #[serde(rename = "Replace")]
    replace: ::serde_json::Value,
}

/// configuration and reload-configuration functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Configuration {
    #[serde(rename = "App")]
    app: App,
    #[serde(rename = "Peer")]
    peer: Peer,
    #[serde(rename = "Log")]
    log: Log,
    #[serde(rename = "Wallet")]
    wallet: Wallet,
    #[serde(rename = "Walletd")]
    walletd: Walletd,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct App {
    #[serde(rename = "PortNumber")]
    port_number: i64,
    #[serde(rename = "HomeDir")]
    home_dir: String,
    #[serde(rename = "ControlPanelPort")]
    control_panel_port: i64,
    #[serde(rename = "ControlPanelFilesPath")]
    control_panel_files_path: String,
    #[serde(rename = "ControlPanelSetting")]
    control_panel_setting: String,
    #[serde(rename = "DBType")]
    dbtype: String,
    #[serde(rename = "LdbPath")]
    ldb_path: String,
    #[serde(rename = "BoltDBPath")]
    bolt_dbpath: String,
    #[serde(rename = "DataStorePath")]
    data_store_path: String,
    #[serde(rename = "DirectoryBlockInSeconds")]
    directory_block_in_seconds: i64,
    #[serde(rename = "ExportData")]
    export_data: bool,
    #[serde(rename = "ExportDataSubpath")]
    export_data_subpath: String,
    #[serde(rename = "NodeMode")]
    node_mode: String,
    #[serde(rename = "IdentityChainID")]
    identity_chain_id: String,
    #[serde(rename = "LocalServerPrivKey")]
    local_server_priv_key: String,
    #[serde(rename = "LocalServerPublicKey")]
    local_server_public_key: String,
    #[serde(rename = "ExchangeRate")]
    exchange_rate: i64,
    #[serde(rename = "ExchangeRateChainId")]
    exchange_rate_chain_id: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKey")]
    exchange_rate_authority_public_key: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKeyMainNet")]
    exchange_rate_authority_public_key_main_net: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKeyTestNet")]
    exchange_rate_authority_public_key_test_net: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKeyLocalNet")]
    exchange_rate_authority_public_key_local_net: String,
    #[serde(rename = "Network")]
    network: String,
    #[serde(rename = "MainNetworkPort")]
    main_network_port: String,
    #[serde(rename = "PeersFile")]
    peers_file: String,
    #[serde(rename = "MainSeedURL")]
    main_seed_url: String,
    #[serde(rename = "MainSpecialPeers")]
    main_special_peers: String,
    #[serde(rename = "TestNetworkPort")]
    test_network_port: String,
    #[serde(rename = "TestSeedURL")]
    test_seed_url: String,
    #[serde(rename = "TestSpecialPeers")]
    test_special_peers: String,
    #[serde(rename = "LocalNetworkPort")]
    local_network_port: String,
    #[serde(rename = "LocalSeedURL")]
    local_seed_url: String,
    #[serde(rename = "LocalSpecialPeers")]
    local_special_peers: String,
    #[serde(rename = "CustomBootstrapIdentity")]
    custom_bootstrap_identity: String,
    #[serde(rename = "CustomBootstrapKey")]
    custom_bootstrap_key: String,
    #[serde(rename = "FactomdTlsEnabled")]
    factomd_tls_enabled: bool,
    #[serde(rename = "FactomdTlsPrivateKey")]
    factomd_tls_private_key: String,
    #[serde(rename = "FactomdTlsPublicCert")]
    factomd_tls_public_cert: String,
    #[serde(rename = "FactomdRpcUser")]
    factomd_rpc_user: String,
    #[serde(rename = "FactomdRpcPass")]
    factomd_rpc_pass: String,
    #[serde(rename = "ChangeAcksHeight")]
    change_acks_height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Peer {
    #[serde(rename = "AddPeers")]
    add_peers: ::serde_json::Value,
    #[serde(rename = "ConnectPeers")]
    connect_peers: ::serde_json::Value,
    #[serde(rename = "Listeners")]
    listeners: ::serde_json::Value,
    #[serde(rename = "MaxPeers")]
    max_peers: i64,
    #[serde(rename = "BanDuration")]
    ban_duration: i64,
    #[serde(rename = "TestNet")]
    test_net: bool,
    #[serde(rename = "SimNet")]
    sim_net: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Log {
    #[serde(rename = "LogPath")]
    log_path: String,
    #[serde(rename = "LogLevel")]
    log_level: String,
    #[serde(rename = "ConsoleLogLevel")]
    console_log_level: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Wallet {
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Port")]
    port: i64,
    #[serde(rename = "DataFile")]
    data_file: String,
    #[serde(rename = "RefreshInSeconds")]
    refresh_in_seconds: String,
    #[serde(rename = "BoltDBPath")]
    bolt_dbpath: String,
    #[serde(rename = "FactomdAddress")]
    factomd_address: String,
    #[serde(rename = "FactomdPort")]
    factomd_port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Walletd {
    #[serde(rename = "WalletRpcUser")]
    wallet_rpc_user: String,
    #[serde(rename = "WalletRpcPass")]
    wallet_rpc_pass: String,
    #[serde(rename = "WalletTlsEnabled")]
    wallet_tls_enabled: bool,
    #[serde(rename = "WalletTlsPrivateKey")]
    wallet_tls_private_key: String,
    #[serde(rename = "WalletTlsPublicCert")]
    wallet_tls_public_cert: String,
    #[serde(rename = "FactomdLocation")]
    factomd_location: String,
    #[serde(rename = "WalletdLocation")]
    walletd_location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProcessList {
    #[serde(rename = "ProcessList")]
    process_list: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Authorities {
    #[serde(rename = "Authorities")]
    authorities: Vec<Authority>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Authority {
    #[serde(rename = "AuthorityChainID")]
    authority_chain_id: String,
    #[serde(rename = "ManagementChainID")]
    management_chain_id: String,
    #[serde(rename = "MatryoshkaHash")]
    matryoshka_hash: String,
    #[serde(rename = "SigningKey")]
    signing_key: String,
    #[serde(rename = "Status")]
    status: i64,
    #[serde(rename = "AnchorKeys")]
    anchor_keys: ::serde_json::Value,
    #[serde(rename = "KeyHistory")]
    key_history: ::serde_json::Value,
}

/// drop-rate and set-drop-rate functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DropRate {
    #[serde(rename = "DropRate")]
    drop_rate: i64,
}

/// delay and set-delay functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Delay {
    #[serde(rename = "Delay")]
    delay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Summary {
    #[serde(rename = "Summary")]
    summary: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Messages {
    #[serde(rename = "Messages")]
    messages: Vec<String>,
}

