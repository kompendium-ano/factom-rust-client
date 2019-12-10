use super::*;

/// Return the wallet seed and all addresses in the wallet for backup and offline 
/// storage.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let response = walletd::wallet_backup(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn wallet_backup(api: &Factom)
    -> Result<ApiResponse<WalletBackup>>
  {
  let req =  ApiRequest::new("wallet-backup");
  let response = walletd_call(api, req).await;
  parse(response).await
} 

/// The wallet-balances API is used to query the acknowledged and saved balances for 
/// all addresses in the currently running factom-walletd. The saved balance is the 
/// last saved to the database and the acknowledged or “ack” balance is the balance 
/// after processing any in-flight transactions known to the Factom node responding 
/// to the API call. The factoid address balance will be returned in factoshis 
/// (a factoshi is 10^8 factoids) not factoids(FCT) and the entry credit balance 
/// will be returned in entry credits.
/// 
/// * If walletd and factomd are not both running this call will not work.
/// 
/// * If factomd is not loaded up all the way to last saved block it will 
/// return: “result”:{“Factomd Error”:“Factomd is not fully booted, please 
/// wait and try again.”}
/// 
/// * If an address is not in the correct format the call will return: 
/// “result”:{“Factomd Error”:”There was an error decoding an address”}
/// 
/// * If an address does not have a public and private address known to the wallet 
/// it will not be included in the balance.
/// 
/// * "fctaccountbalances" are the total of all factoid account balances returned 
/// in factoshis.
/// 
/// * "ecaccountbalances" are the total of all entry credit account balances 
/// returned in entry credits.
  pub async fn wallet_balances(api: &Factom)
-> Result<ApiResponse<WalletBalances>>
{
  let req =  ApiRequest::new("wallet-balances");
  let response = walletd_call(api, req).await;
  parse(response).await
} 

///  Unlocks this wallet for the amount of time specified in seconds by timeout. 
///  The maximum amount of time a wallet can be unlocked for is 230 seconds (Roughly 
///  34 Years… Give or take a decade). This command will only work on wallets that 
///  are encrypted. If successful, returns the expiration time of your access as a 
///  Unix timestamp.
/// 
/// While the wallet is locked, the only accessible RPC API commands are get-height, 
/// properties, transactions, and unlock-wallet.
pub async fn unlock_wallet(
  api: &Factom, 
  passphrase :&str, 
  timeout: usize
) -> Result<ApiResponse<UnlockWallet>>
{
  let mut req =  ApiRequest::new("unlock-wallet");
  req.params.insert("passphrase".to_string(), json!(passphrase));
  req.params.insert("timeout".to_string(), json!(timeout));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Get the current hight of blocks that have been cached by the wallet while syncing.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let response = walletd::wallet_height(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn wallet_height(api: &Factom)
  -> Result<ApiResponse<Height>>
{
  let req =  ApiRequest::new("get-height");
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Retrieve current properties of factom-walletd, including the wallet and wallet 
/// API versions.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let response = walletd::wallet_properties(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn wallet_properties(api: &Factom)
  -> Result<ApiResponse<Properties>>
{
  let req =  ApiRequest::new("properties");
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// unlock-wallet function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlockWallet {
  pub success: bool,
  pub unlockeduntil: i64,
}

/// wallet-backup function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletBackup {
  #[serde(rename = "wallet-seed")]
  pub wallet_seed: String,
  pub addresses: Vec<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
  pub public: String,
  pub secret: String,
}

/// wallet-balances function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletBalances {
  pub fctaccountbalances: Fctaccountbalances,
  pub ecaccountbalances: Ecaccountbalances,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fctaccountbalances {
  pub ack: i64,
  pub saved: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ecaccountbalances {
  pub ack: i64,
  pub saved: i64,
}


/// sign-data function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignData {
  pub pubkey: String,
  pub signature: String,
}

/// wallet-properties function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
  pub walletversion: String,
  pub walletapiversion: String,
}

/// get-height function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Height {
  pub height: i64,
}

