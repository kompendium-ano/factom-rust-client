//! For functions handling chain data.
use super::*;

/// Return the keymr of the head of the chain for a chain ID (the unique hash 
/// created when the chain was created).
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let chainid= "a642a8674f46696cc47fdb6b65f9c87b2a19c5ea8123b3d2f0c13b6f33a9d5ef";
///   let response = chain::chain_head(&client, chainid)
///                         .await
///                         .unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn chain_head(client: &Factom, chainid: &str)
  -> Result<ApiResponse<ChainHead>>
{
  let mut req =  ApiRequest::new("chain-head");
  req.params.insert("chainid".to_string(), json!(chainid));
  let response = requests::factomd_call(client, req).await;
  parse(response).await
}

/// Send a Chain Commit Message to factomd to create a new Chain. 
/// The commit chain hex encoded string is documented here: 
/// [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry-commit)
/// 
/// The commit-chain API takes a specifically formated message encoded in hex that 
/// includes signatures. If you have a factom-walletd instance running, you can 
/// construct this commit-chain API call with compose-chain which takes easier 
/// to construct arguments.
/// 
/// The compose-chain api call has two api calls in it’s response: 
/// commit-chain and reveal-chain. To successfully create a chain, the reveal-chain 
/// must be called after the commit-chain.
/// 
/// Notes:
/// It is possible to be unable to send a commit, if the commit already exists
///  (if you try to send it twice). This is a mechanism to prevent you from double 
///  spending. If you encounter this error, just skip to the reveal-chain. The 
///  error format can be found here: repeated-commit
///
/// See the examples folder for the full process flow linked together.
pub async fn commit_chain(api: &Factom, message: &str)
  -> Result<ApiResponse<CommitChain>>
{
  let mut req =  ApiRequest::new("commit-chain");
  req.params.insert("message".to_string(), json!(message));
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Reveal the First Entry in a Chain to factomd after the Commit to complete the 
/// Chain creation. The reveal-chain hex encoded string is documented here: 
/// [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#entry)
/// 
/// The reveal-chain API takes a specifically formatted message encoded in hex that 
/// includes signatures. If you have a factom-walletd instance running, you can 
/// construct this reveal-chain API call with compose-chain which takes easier to 
/// construct arguments.
/// 
/// The compose-chain api call has two api calls in its response: commit-chain and 
/// reveal-chain. To successfully create a chain, the reveal-chain must be called 
/// after the commit-chain.
///
/// See the examples folder for the full process flow linked together.
pub async fn reveal_chain(api: &Factom, entry: &str)
  -> Result<ApiResponse<RevealChain>>
{
  let mut req =  ApiRequest::new("reveal-chain");
  req.params.insert("entry".to_string(), json!(entry));
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// chain-head function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainHead {
  pub chainhead: String,
  pub chaininprocesslist: bool,
}

/// commit-chain function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitChain {
  pub message: String,
  pub txid: String,
  pub entryhash: String,
  #[serde(alias = "chainidhash")]
  pub chainid: String,
}

/// reveal-chain function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevealChain {
  pub message: String,
  pub entryhash: String,
  #[serde(alias = "chainidhash")]
  pub chainid: String,
}