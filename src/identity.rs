//! Relating to identity functions.
use super ::*;

/// Returns all of the identity key pairs that are currently stored in the wallet. 
/// If the wallet is encrypted, it must be unlocked prior to using this command.
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let response = identity::all_id_keys(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn all_id_keys(api: &Factom)
  -> Result<ApiResponse<IdKeys>>
{
  let req =  ApiRequest::new("all-identity-keys");
  let response = walletd_call(api, req).await;
  parse(response).await
}
/// This command will return an identity’s set of public keys (in order of 
/// decreasing priority) that were active at a specific block, or at the most 
/// recent height if the "height" parameter is not included. This is useful for 
/// validating entries containing identity signatures (e.g. on identity attributes 
/// and endorsements), allowing you to tell if a given signature was created with 
/// a key that was valid at the time that the entry was published. Time is 
/// measured in directory blocks.
/// 
/// As an example, let’s say the identity at chain-id 
/// 3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae signs an entry 
/// using their level-3 key idpub2GU1Pcax2PibH8hHZg58fKRiSJKQWQkWYkpmt7VH1jCXBgqp9w, 
/// and publishes it to the blockchain at height 163420 and then replaces that key 
/// one block later at height 163421. Even though the key is no longer valid at the 
/// highest block height, we can tell that it was valid at the time that the 
/// signature was created, so we can still trust that the entry is authentic. 
/// However, if someone then published another entry signed with the key that was 
/// just replaced, we will be able to tell that the signer key is no longer valid 
/// and that the entry shouldn’t be trusted.
/// 
/// If the wallet is encrypted, it must be unlocked prior to using this command.
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let chainid = "3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae";
///   let height = 163419;
///   let response = identity::active_id_keys(&client, chainid, Some(height)).await.unwrap();
///   dbg!(&response);
/// }
/// ```
pub async fn active_id_keys(
  api: &Factom,
  chain_id: &str,
  height: Option<usize>
)-> Result<ApiResponse<ActiveIdKeys>>
{
  let mut req =  ApiRequest::new("active-identity-keys");
  req.params.insert("chainid".to_string(), json!(chain_id));
  match height {
    Some(height) => req.params.insert("height".to_string(), json!(height)),
    None => None
  };
  let response = walletd_call(api, req).await;
  parse(response).await
}

///  **Be careful using this function! Ensure that you have backups of important keys 
///  before removing them.** Given an identity public key, this command deletes the 
///  corresponding identity key pair from the wallet. Once executed, the user will 
///  no longer be able to retrieve that key pair or sign attributes/endorsements 
///  with the key pair from this wallet. If the wallet is encrypted, it must be 
///  unlocked prior to using this command.
///  # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let response = generate::identity_key(&client).await.unwrap();
///   let address = response.result.public;
///   let remove_response = identity::remove_id_key(&client, &address).await.unwrap();
///   dbg!(&remove_response);
///   assert!(remove_response.result.success);
/// }
/// ```
pub async fn remove_id_key(
  api: &Factom,
  public: &str
)-> Result<ApiResponse<RemoveIdKey>>
{
  let mut req =  ApiRequest::new("remove-identity-key");
  req.params.insert("public".to_string(), json!(public));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Given an identity public key as input, this command will respond with the 
/// corresponding public/private key pair from the wallet. If the desired identity 
/// key isn’t currently stored in the wallet, an error is returned to indicate this. 
/// If the wallet is encrypted, it must be unlocked prior to using this command.
///  # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   /// Generate key
///   let gen_response = generate::identity_key(&client).await.unwrap();
///   let pub_id = &gen_response.result.public;
///   let priv_id = &gen_response.result.secret;
///   /// Get key from wallet
///   let id_response = identity::id_key(&client, pub_id).await.unwrap();
///   assert_eq!(&id_response.result.secret, priv_id);
///   /// Remove key
///   let remove_response = identity::remove_id_key(&client, &pub_id).await.unwrap();
/// }
/// ```
pub async fn id_key(
  api: &Factom,
  public: &str
)-> Result<ApiResponse<Key>>
{
  let mut req =  ApiRequest::new("identity-key");
  req.params.insert("public".to_string(), json!(public));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// all-identity-keys function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdKeys {
  pub keys: Option<Vec<Key>>,
}

/// identity-key function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
  pub public: String,
  pub secret: String,
}

/// active-identity-keys function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveIdKeys {
  pub chainid: String,
  pub height: i64,
  pub keys: Vec<String>,
}

/// remove-id-key function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveIdKey {
  pub success: bool,
}