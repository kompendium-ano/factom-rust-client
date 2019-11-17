use super::*;

impl Factom {
/// Return the wallet seed and all addresses in the wallet for backup and offline 
/// storage.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///             .wallet_backup()
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn wallet_backup(self)
      -> Result<ApiResponse<WalletBackup>>
    {
    let req =  ApiRequest::new("wallet-backup");
    let response = self.walletd_call(req).await;
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
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///             .wallet_balances()
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
   pub async fn wallet_balances(self)
  -> Result<ApiResponse<WalletBalances>>
  {
    let req =  ApiRequest::new("wallet-balances");
    let response = self.walletd_call(req).await;
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
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let passphrase = "opensesame";
/// let timeout = 300;
/// let query = factom
///             .unlock_wallet(
///               passphrase,
///               timeout
///             )
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn unlock_wallet(
    self, 
    passphrase :&str, 
    timeout: usize
  ) -> Result<ApiResponse<UnlockWallet>>
  {
    let mut req =  ApiRequest::new("unlock-wallet");
    req.params.insert("passphrase".to_string(), json!(passphrase));
    req.params.insert("timeout".to_string(), json!(timeout));
    let response = self.walletd_call(req).await;
    parse(response).await
  }

/// Get the current hight of blocks that have been cached by the wallet while syncing.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///             .get_height()
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn wallet_height(self)
    -> Result<ApiResponse<Height>>
  {
    let req =  ApiRequest::new("get-height");
    let response = self.walletd_call(req).await;
    parse(response).await
  }
  
/// Retrieve current properties of factom-walletd, including the wallet and wallet 
/// API versions.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///             .properties()
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn wallet_properties(self)
    -> Result<ApiResponse<Properties>>
  {
    let req =  ApiRequest::new("properties");
    let response = self.walletd_call(req).await;
    parse(response).await
  }

///  Sign arbitrary data using a secret key stored in the wallet using ed25519 
///  signatures. signer can be a human readable Factoid Address (FA), Entry Credit 
///  Address (EC), or Identity Key (idpub). data is a base64-encoded string. 
///  Returns both the public key component and the signature as base64-encoded 
///  strings. Wallet must be unlocked prior to using this command.
/// 
/// Note: For signing large amounts of data it may be advisable to sign a hash of 
/// the data rather than the data itself.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let signer = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
/// let data = "Here be data";
/// let query = factom
///             .sign_data(
///               signer,
///               data
///             )
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn sign_data(
    self, 
    signer: &str, 
    data: &str
  )-> Result<ApiResponse<SignData>>
  {
    let mut req =  ApiRequest::new("sign-data");
    req.params.insert("signer".to_string(), json!(signer));
    req.params.insert("data".to_string(), json!(data));
    let response = self.walletd_call(req).await;
    parse(response).await
  }
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

