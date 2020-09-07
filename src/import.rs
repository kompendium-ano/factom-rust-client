//! For importing addresses or identities
use super::*;
use std::collections::HashMap;

/// Import Factoid and/or Entry Credit address secret keys into the wallet.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let priv_key = "Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK";
///   let response = import::import_addresses(&client, vec![priv_key]).await.unwrap();
///   dbg!(&response);
///   assert!(response.success())
/// }
/// ```
pub async fn import_addresses(
    api: &Factom,
    addresses: Vec<&str>,
) -> Result<ApiResponse<Addresses>> {
    let mut req = ApiRequest::new("import-addresses");
    let mut secrets: Vec<HashMap<&str, &str>> = Vec::new();
    for address in addresses {
        let mut tmp = HashMap::new();
        tmp.insert("secret", address);
        secrets.push(tmp);
    }
    req.params.insert("addresses".to_string(), json!(secrets));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// Allows a user to add one or more identity keys to the wallet. Using the secret
/// keys as input, the command will return the corresponding key pairs that were
/// imported. If the wallet is encrypted, it must be unlocked prior to using this
/// command.
///
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let priv_id_key = "idsec2rWrfNTD1x9HPPesA3fz8dmMNZdjmSBULHx8VTXE1J4D9icmAK";
///   let response = import::import_identity_keys(&client, vec![priv_id_key])
///                             .await
///                             .unwrap();
///   dbg!(&response);
///   assert!(response.success())
/// }
/// ```
pub async fn import_identity_keys(api: &Factom, keys: Vec<&str>) -> Result<ApiResponse<Keys>> {
    let mut req = ApiRequest::new("import-identity-keys");
    let mut secrets: Vec<HashMap<&str, &str>> = Vec::new();
    for address in keys {
        let mut tmp = HashMap::new();
        tmp.insert("secret", address);
        secrets.push(tmp);
    }
    req.params.insert("keys".to_string(), json!(secrets));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// Import a Koinify crowd sale address into the wallet. In our examples we used
/// the word “yellow” twelve times, note that in your case the master passphrase
/// will be different. If the wallet is encrypted, it must be unlocked prior to
/// using this command.
///
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::new();
///   let koinify_phrase = "yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow yellow";
///   let response = import::import_koinify(&client, koinify_phrase)
///                             .await
///                             .unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn import_koinify(api: &Factom, phrase: &str) -> Result<ApiResponse<Address>> {
    let mut req = ApiRequest::new("import-koinify");
    req.params.insert("words".to_string(), json!(phrase));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// import-addresses function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Addresses {
    pub addresses: Vec<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub public: String,
    pub secret: String,
}

/// import-identity-keys function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keys {
    pub keys: Vec<Key>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub public: String,
    pub secret: String,
}
