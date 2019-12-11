use super::*;

/// Show current holding messages in the queue.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::holding_queue(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn holding_queue(api: &Factom)
  -> Result<ApiResponse<HoldingQueue>>
{
  let req =  ApiRequest::new("holding-queue");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get information on the current network factomd is connected to (TEST, MAIN, etc)
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::network_info(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn network_info(api: &Factom)
  -> Result<ApiResponse<NetworkInfo>>
{
  let req =  ApiRequest::new("network-info");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get the predicted future entry credit rate.
/// ///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::predictive_fer(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn predictive_fer(api: &Factom)
  -> Result<ApiResponse<PredictiveFER>>
{
  let req =  ApiRequest::new("predictive-fer");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get a list of the current network audit servers along with their information.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::audit_servers(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn audit_servers(api: &Factom)
  -> Result<ApiResponse<AuditServers>>
{
  let req =  ApiRequest::new("audit-servers");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get a list of the current network federated servers along with their information.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::federated_servers(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn federated_servers(api: &Factom)
  -> Result<ApiResponse<FederatedServers>>
{
  let req =  ApiRequest::new("federated-servers");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get the current configuration state from factomd.conf.
/// 
/// NOTE: If a tag is commented out, this call will return the default value for it.
/// E.g: In the Example Response “ExchangeRate” is set to “0”. factomd.config default
///  does not have an “ExchangeRate” tag. That is why it is set to “0”.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::configuration(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn configuration(api: &Factom)
  -> Result<ApiResponse<Configuration>>
{
  let req =  ApiRequest::new("configuration");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get the process list known to the current factomd instance. 

pub async fn process_list(api: &Factom)
  -> Result<ApiResponse<ProcessList>>
{
  let req =  ApiRequest::new("process-list");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// List of authority servers in the management chain.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::authorities(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn authorities(api: &Factom)
  -> Result<ApiResponse<Authorities>>
{
  let req =  ApiRequest::new("authorities");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Causes factomd to re-read the configuration from the config file. Note: This 
/// may cause consensus problems and could be impractical even in testing.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::reload_configuration(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn reload_configuration(api: &Factom)
  -> Result<ApiResponse<Configuration>>
{
  let req =  ApiRequest::new("reload-configuration");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get the current package drop rate for network testing.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::drop_rate(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn drop_rate(api: &Factom)
  -> Result<ApiResponse<DropRate>>
{
  let req =  ApiRequest::new("drop-rate");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Change the network drop rate for testing.
pub async fn set_drop_rate(
  api: &Factom,
  drop_rate: usize
)-> Result<ApiResponse<DropRate>>
{
  let mut req =  ApiRequest::new("set-drop-rate");
  req.params.insert("DropRate".to_string(), json!(drop_rate)); 
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get the current msg delay time for network testing.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::delay(&client)
///                         .await
///                         .expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn delay(api: &Factom)
  -> Result<ApiResponse<Delay>>{
  let req =  ApiRequest::new("delay");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Set the current msg delay time for network testing.
pub async fn set_delay(
  api: &Factom,
  delay: usize
)-> Result<ApiResponse<Delay>>
{
  let mut req =  ApiRequest::new("set-delay");
  req.params.insert("Delay".to_string(), json!(delay)); 
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get the nodes summary string.
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::summary(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn summary(api: &Factom)
  -> Result<ApiResponse<Summary>>{ 
  let req =  ApiRequest::new("summary");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// Get a list of messages from the message journal 
/// (must run factomd with -journaling=true)
///
/// # Example
///
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   /// Doctest examples will only work with a local factomd node running
///   let client = Factom::new();
///   let response = debug::messages(&client).await.expect("Api Request");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn messages(api: &Factom)
  -> Result<ApiResponse<Messages>>
{ 
  let req =  ApiRequest::new("messages");
  let response = debug_call(api, req).await;
  parse(response).await
}

/// holding-queue function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoldingQueue {
  #[serde(rename = "Messages")]
  messages: Option<String>,
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
  pub predictive_fer: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditServers {
    #[serde(rename = "AuditServers")]
  pub audit_servers: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedServers {
    #[serde(rename = "FederatedServers")]
  pub federated_servers: Vec<FederatedServer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedServer {
    #[serde(rename = "ChainID")]
  pub chain_id: String,
    #[serde(rename = "Name")]
  pub name: String,
    #[serde(rename = "Online")]
  pub online: bool,
    #[serde(rename = "Replace")]
  pub replace: ::serde_json::Value,
}

/// configuration and reload-configuration functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "App")]
  pub app: App,
    #[serde(rename = "Peer")]
  pub peer: Peer,
    #[serde(rename = "Log")]
  pub log: Log,
    #[serde(rename = "Wallet")]
  pub wallet: Wallet,
    #[serde(rename = "Walletd")]
  pub walletd: Walletd,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(rename = "PortNumber")]
  pub port_number: i64,
    #[serde(rename = "HomeDir")]
  pub home_dir: String,
    #[serde(rename = "ControlPanelPort")]
  pub control_panel_port: i64,
    #[serde(rename = "ControlPanelFilesPath")]
  pub control_panel_files_path: String,
    #[serde(rename = "ControlPanelSetting")]
  pub control_panel_setting: String,
    #[serde(rename = "DBType")]
  pub dbtype: String,
    #[serde(rename = "LdbPath")]
  pub ldb_path: String,
    #[serde(rename = "BoltDBPath")]
  pub bolt_dbpath: String,
    #[serde(rename = "DataStorePath")]
  pub data_store_path: String,
    #[serde(rename = "DirectoryBlockInSeconds")]
  pub directory_block_in_seconds: i64,
    #[serde(rename = "ExportData")]
  pub export_data: bool,
    #[serde(rename = "ExportDataSubpath")]
  pub export_data_subpath: String,
    #[serde(rename = "NodeMode")]
  pub node_mode: String,
    #[serde(rename = "IdentityChainID")]
  pub identity_chain_id: String,
    #[serde(rename = "LocalServerPrivKey")]
  pub local_server_priv_key: String,
    #[serde(rename = "LocalServerPublicKey")]
  pub local_server_public_key: String,
    #[serde(rename = "ExchangeRate")]
  pub exchange_rate: i64,
    #[serde(rename = "ExchangeRateChainId")]
  pub exchange_rate_chain_id: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKey")]
  pub exchange_rate_authority_public_key: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKeyMainNet")]
  pub exchange_rate_authority_public_key_main_net: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKeyTestNet")]
  pub exchange_rate_authority_public_key_test_net: String,
    #[serde(rename = "ExchangeRateAuthorityPublicKeyLocalNet")]
  pub exchange_rate_authority_public_key_local_net: String,
    #[serde(rename = "Network")]
  pub network: String,
    #[serde(rename = "MainNetworkPort")]
  pub main_network_port: String,
    #[serde(rename = "PeersFile")]
  pub peers_file: String,
    #[serde(rename = "MainSeedURL")]
  pub main_seed_url: String,
    #[serde(rename = "MainSpecialPeers")]
  pub main_special_peers: String,
    #[serde(rename = "TestNetworkPort")]
  pub test_network_port: String,
    #[serde(rename = "TestSeedURL")]
  pub test_seed_url: String,
    #[serde(rename = "TestSpecialPeers")]
  pub test_special_peers: String,
    #[serde(rename = "LocalNetworkPort")]
  pub local_network_port: String,
    #[serde(rename = "LocalSeedURL")]
  pub local_seed_url: String,
    #[serde(rename = "LocalSpecialPeers")]
  pub local_special_peers: String,
    #[serde(rename = "CustomBootstrapIdentity")]
  pub custom_bootstrap_identity: String,
    #[serde(rename = "CustomBootstrapKey")]
  pub custom_bootstrap_key: String,
    #[serde(rename = "FactomdTlsEnabled")]
  pub factomd_tls_enabled: bool,
    #[serde(rename = "FactomdTlsPrivateKey")]
  pub factomd_tls_private_key: String,
    #[serde(rename = "FactomdTlsPublicCert")]
  pub factomd_tls_public_cert: String,
    #[serde(rename = "FactomdRpcUser")]
  pub factomd_rpc_user: String,
    #[serde(rename = "FactomdRpcPass")]
  pub factomd_rpc_pass: String,
    #[serde(rename = "ChangeAcksHeight")]
  pub change_acks_height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    #[serde(rename = "AddPeers")]
  pub add_peers: ::serde_json::Value,
    #[serde(rename = "ConnectPeers")]
  pub connect_peers: ::serde_json::Value,
    #[serde(rename = "Listeners")]
  pub listeners: ::serde_json::Value,
    #[serde(rename = "MaxPeers")]
  pub max_peers: i64,
    #[serde(rename = "BanDuration")]
  pub ban_duration: i64,
    #[serde(rename = "TestNet")]
  pub test_net: bool,
    #[serde(rename = "SimNet")]
  pub sim_net: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    #[serde(rename = "LogPath")]
  pub log_path: String,
    #[serde(rename = "LogLevel")]
  pub log_level: String,
    #[serde(rename = "ConsoleLogLevel")]
  pub console_log_level: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(rename = "Address")]
  pub address: String,
    #[serde(rename = "Port")]
  pub port: i64,
    #[serde(rename = "DataFile")]
  pub data_file: String,
    #[serde(rename = "RefreshInSeconds")]
  pub refresh_in_seconds: String,
    #[serde(rename = "BoltDBPath")]
  pub bolt_dbpath: String,
    #[serde(rename = "FactomdAddress")]
  pub factomd_address: String,
    #[serde(rename = "FactomdPort")]
  pub factomd_port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Walletd {
    #[serde(rename = "WalletRpcUser")]
  pub wallet_rpc_user: String,
    #[serde(rename = "WalletRpcPass")]
  pub wallet_rpc_pass: String,
    #[serde(rename = "WalletTlsEnabled")]
  pub wallet_tls_enabled: bool,
    #[serde(rename = "WalletTlsPrivateKey")]
  pub wallet_tls_private_key: String,
    #[serde(rename = "WalletTlsPublicCert")]
  pub wallet_tls_public_cert: String,
    #[serde(rename = "FactomdLocation")]
  pub factomd_location: String,
    #[serde(rename = "WalletdLocation")]
  pub walletd_location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessList {
    #[serde(rename = "ProcessList")]
  pub process_list: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorities {
    #[serde(rename = "Authorities")]
  pub authorities: Vec<Authority>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authority {
    #[serde(rename = "AuthorityChainID")]
  pub authority_chain_id: String,
    #[serde(rename = "ManagementChainID")]
  pub management_chain_id: String,
    #[serde(rename = "MatryoshkaHash")]
  pub matryoshka_hash: String,
    #[serde(rename = "SigningKey")]
  pub signing_key: String,
    #[serde(rename = "Status")]
  pub status: i64,
    #[serde(rename = "AnchorKeys")]
  pub anchor_keys: ::serde_json::Value,
    #[serde(rename = "KeyHistory")]
  pub key_history: ::serde_json::Value,
}

/// drop-rate and set-drop-rate functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropRate {
    #[serde(rename = "DropRate")]
  pub drop_rate: i64,
}

/// delay and set-delay functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delay {
    #[serde(rename = "Delay")]
  pub delay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "Summary")]
  pub summary: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Messages {
    #[serde(rename = "Messages")]
  pub messages: Vec<String>,
}

