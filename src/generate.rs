use super::*;

/// Create a new Entry Credit Address and store it in the wallet. If the wallet 
/// is encrypted, it must be unlocked prior to using this command.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let response = generate::ec_address(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
///   /// Remove doctest address
///   let address = response.result.public;
///   let remove = address::remove_address(&client, &address).await.unwrap();
/// }
/// ```
pub async fn ec_address(api: &Factom)
  -> Result<ApiResponse<Generate>>
{
  let req =  ApiRequest::new("generate-ec-address");
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Create a new Entry Credit Address and store it in the wallet.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let response = generate::factoid_address(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
///   /// Remove doctest address
///   let address = response.result.public;
///   let remove = address::remove_address(&client, &address).await.unwrap();
/// }
/// ```
pub async fn factoid_address(api: &Factom)
  -> Result<ApiResponse<Generate>>
{
  let req =  ApiRequest::new("generate-factoid-address");
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Creates a new identity key and adds it to the wallet. New keys are generated 
/// from the same mnemonic seed used for FCT and EC addresses. If the wallet is 
/// encrypted, it must be unlocked prior to using this command.
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let response = generate::identity_key(&client).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
///   /// Remove doctest address
///   let address = response.result.public;
///   let remove = identity::remove_id_key(&client, &address).await.unwrap();
/// }
/// ```
pub async fn identity_key(api: &Factom)
  -> Result<ApiResponse<Generate>>
  {
  let req =  ApiRequest::new("generate-identity-key");
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Deserialises from generate-ec-address, generate-fct-address and 
/// generate-identity-key
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Generate {
    pub public: String,
    pub secret: String,
}
