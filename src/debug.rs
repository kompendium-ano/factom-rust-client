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
  pub async fn holding_queue(self)
    -> Result<ApiResponse<HoldingQueue>>
  {
    let req =  ApiRequest::new("holding-queue");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn network_info(self)
    -> Result<ApiResponse<NetworkInfo>>
  {
    let req =  ApiRequest::new("network-info");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn predictive_fer(self)
    -> Result<ApiResponse<PredictiveFER>>
  {
    let req =  ApiRequest::new("predictive-fer");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn audit_servers(self)
    -> Result<ApiResponse<AuditServers>>
  {
    let req =  ApiRequest::new("audit-servers");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn federated_servers(self)
    -> Result<ApiResponse<FederatedServers>>
  {
    let req =  ApiRequest::new("federated-servers");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn configuration(self)
    -> Result<ApiResponse<Configuration>>
  {
    let req =  ApiRequest::new("configuration");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn process_list(self)
    -> Result<ApiResponse<ProcessList>>
  {
    let req =  ApiRequest::new("process-list");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn authorities(self)
    -> Result<ApiResponse<Authorities>>
  {
    let req =  ApiRequest::new("authorities");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn reload_configuration(self)
    -> Result<ApiResponse<Configuration>>
  {
    let req =  ApiRequest::new("reload-configuration");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn drop_rate(self)
    -> Result<ApiResponse<DropRate>>
  {
    let req =  ApiRequest::new("drop-rate");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn set_drop_rate(
    self, 
    drop_rate: usize
  )-> Result<ApiResponse<DropRate>>
  {
    let mut req =  ApiRequest::new("set-drop-rate");
    req.params.insert("DropRate".to_string(), json!(drop_rate)); 
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn delay(self)
    -> Result<ApiResponse<Delay>>{
    let req =  ApiRequest::new("delay");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn set_delay(
    self, 
    delay: usize
  )-> Result<ApiResponse<Delay>>
  {
    let mut req =  ApiRequest::new("set-delay");
    req.params.insert("Delay".to_string(), json!(delay)); 
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn summary(self)
    -> Result<ApiResponse<Summary>>{ 
    let req =  ApiRequest::new("summary");
    let response = self.debug_call(req).await;
    parse(response).await
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
  pub async fn messages(self)
    -> Result<ApiResponse<Messages>>
  { 
    let req =  ApiRequest::new("messages");
    let response = self.debug_call(req).await;
    parse(response).await
  }
}

/// holding-queue function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoldingQueue {
  #[serde(rename = "NetworkNumber")]
  messages: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkInfo {
  #[serde(rename = "NetworkNumber")]
  network_number: i64,
  #[serde(rename = "NetworkName")]
  network_name: String,
  #[serde(rename = "NetworkID")]
  network_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictiveFER {
    #[serde(rename = "PredictiveFER")]
    predictive_fer: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditServers {
    #[serde(rename = "AuditServers")]
    audit_servers: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedServers {
    #[serde(rename = "FederatedServers")]
    federated_servers: Vec<FederatedServer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedServer {
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
pub struct Configuration {
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
pub struct App {
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
pub struct Peer {
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
pub struct Log {
    #[serde(rename = "LogPath")]
    log_path: String,
    #[serde(rename = "LogLevel")]
    log_level: String,
    #[serde(rename = "ConsoleLogLevel")]
    console_log_level: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
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
pub struct Walletd {
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
pub struct ProcessList {
    #[serde(rename = "ProcessList")]
    process_list: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorities {
    #[serde(rename = "Authorities")]
    authorities: Vec<Authority>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authority {
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
pub struct DropRate {
    #[serde(rename = "DropRate")]
    drop_rate: i64,
}

/// delay and set-delay functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delay {
    #[serde(rename = "Delay")]
    delay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "Summary")]
    summary: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Messages {
    #[serde(rename = "Messages")]
    messages: Vec<String>,
}

