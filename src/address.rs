//! Relating to Address functions
use super::*;

/// Retrieve the public and private parts of a Factoid or Entry Credit address 
///stored in the wallet.
///
///# Example
///```
/// use factom::*;
/// 
///#[tokio::main]
///async fn main() {
///  let client = Factom::new();
///  let my_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
///  let query = address::address(&client, my_address);
///  let response = query.await.expect("Fetching query");
///  assert_eq!(response.result.public, my_address);
/// } 
/// ```
pub async fn address(
  api: &Factom, 
  address: &str
)-> Result<ApiResponse<Address>>
{
  let mut req =  ApiRequest::new("address");
  req.params.insert("address".to_string(), json!(address));
  let response = walletd_call(api, req).await;
  parse(response).await
}

///Retrieve all of the Factoid and Entry Credit addresses stored in the wallet.
///
///# Example
///```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///  let client = Factom::new();
///  let my_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
///  let query = address::all_addresses(&client);
///  let response = query.await.unwrap();
///  // Iterate through all returned addresses for my_address 
///  assert!(response
///           .result
///           .addresses
///           .iter()
///           .any(|address| address.public == my_address));
/// }
/// ``` 
pub async fn all_addresses(api: &Factom)
  -> Result<ApiResponse<AllAddresses>> 
{
  let req =  ApiRequest::new("all-addresses");
  let response = requests::walletd_call(api, req).await;
  parse(response).await
}

/// Be careful using this function! Ensure that you have backups of important keys 
/// before removing them. Given a factoid or entry-credit address, this command 
/// deletes the corresponding key pair from the wallet. Once executed, the user will 
/// no longer be able to retrieve the private key or make transactions with the 
/// address from this wallet. If the wallet is encrypted, it must be unlocked prior 
/// to using this command.
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let new_address = generate::factoid_address(&client)
///                          .await
///                          .expect("Fetching query");
///   let rm_address = address::remove_address(&client, &new_address.result.public)
///                             .await
///                             .expect("Fetching query");
///   assert!(&rm_address.result.success);
///  }
/// ```
pub async fn remove_address(
  api: &Factom, 
  address: &str
)-> Result<ApiResponse<RemoveAddress>>
{
  let mut req =  ApiRequest::new("remove-address");
  req.params.insert("address".to_string(), json!(address));
  let response = requests::walletd_call(api, req).await;
  parse(response).await
}

/// address function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
  pub public: String,
  pub secret: String
}

/// all-addresses function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllAddresses {
  pub addresses: Vec<Address>
}

/// remove-address function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveAddress {
  pub success: bool
}



