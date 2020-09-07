//! For balance related functions.
use super::*;

/// Return its current balance for a specific entry credit address.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::testnet_node();
///   let ec_address = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
///   let response = balance::multiple_ec_balances(&client, vec![ec_address])
///                             .await
///                             .expect("Fetching query");
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn entry_credit_balance(api: &Factom, address: &str) -> Result<ApiResponse<Balance>> {
    let mut req = ApiRequest::new("entry-credit-balance");
    req.params.insert("address".to_string(), json!(address));
    let response = factomd_call(api, req).await;
    parse(response).await
}

///  This call returns the number of Factoshis (Factoids *10^-8) that are
/// currently available at the address specified.
///
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
///async fn main() {
///  let client = Factom::testnet_node();
///  let fct_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
///  let response = balance::factoid_balance(&client, fct_address)
///                            .await
///                            .expect("Fetching query");
///  assert_eq!(response.error.code, 0);
/// }
/// ```
pub async fn factoid_balance(api: &Factom, address: &str) -> Result<ApiResponse<Balance>> {
    let mut req = ApiRequest::new("factoid-balance");
    req.params.insert("address".to_string(), json!(address));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// The multiple-ec-balances API is used to query the acknowledged and saved
/// balances for a list of entry credit addresses.
///
/// * currentheight is the current height that factomd was loading.
/// * lastsavedheight is the height last saved to the database.
///
/// * In balances it returns "ack", "saved" and "err".
///   * ack is the balance after processing any in-flight transactions known to
///   the Factom node responding to the API call
///   * saved is the last saved to the database
///   * err is just used to display any error that might have happened during the
///   request. If it is empty that means there was no error.
///
/// * If the syntax of the parameters is off e.g. missing a quote, a comma, or a
/// square bracket, it will return: `{“jsonrpc”:“2.0”,“id”:null,“error”:
/// {“code”:-32600,“message”:“Invalid Request”}}`
///
/// * If the parameters are labeled incorrectly the call will return:
/// `{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed
/// in, expected addresses”}`
///
/// * If factomd is not loaded up all the way to the last saved block it will
/// return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,
/// “err”:“Not fully booted”}]}`
///
/// * If the list of addresses contains an incorrectly formatted address the call
/// will return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,
/// “saved”:0,“err”:“Error decoding address”}]}`
///
/// * If an address in the list is valid but has never been part of a transaction
/// the call will return: `“balances”:[{“ack”:0,“saved”:0,“err”:“Address has not
/// had a transaction”}]`
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::testnet_node();
///   let ec_address = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
///   let response = balance::multiple_ec_balances(&client, vec![ec_address])
///                             .await
///                             .expect("Fetching query");
///   assert!(response.success());
/// }
/// ```
pub async fn multiple_ec_balances(
    api: &Factom,
    addresses: Vec<&str>,
) -> Result<ApiResponse<Balances>> {
    let mut req = ApiRequest::new("multiple-ec-balances");
    req.params.insert("addresses".to_string(), json!(addresses));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// The multiple-fct-balances API is used to query the acknowledged and saved
/// balances in factoshis (a factoshi is 10^8 factoids) not factoids(FCT) for a
/// list of FCT addresses.
///
/// * currentheight is the current height that factomd was loading.
/// * lastsavedheight is the height last saved to the database.
///
/// * In balances it returns "ack", "saved" and "err".
///   * ack is the balance after processing any in-flight transactions known to
///   the Factom node responding to the API call
///   * saved is the last saved to the database
///   * err is just used to display any error that might have happened during the
///   request. If it is "" that means there was no error.
///
/// * If the syntax of the parameters is off e.g. missing a quote, a comma, or a
/// square bracket, it will return: `{”jsonrpc”:“2.0”,“id”:null,“error”:
/// {“code”:-32600,“message”:“Invalid Request”}}`
///
/// * If the parameters are labeled incorrectly the call will return: `
/// {“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed in, expected 'addresses’”}`
///
/// * If factomd is not loaded up all the way to the last saved block it will
/// return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:
/// [{“ack”:0,“saved”:0,“err”:“Not fully booted”}]}`
///
/// * If the list of addresses contains an incorrectly formatted address the call
/// will return: `{“currentheight”:0,“lastsavedheight”:0,
/// “balances”:[{“ack”:0,“saved”:0,“err”:“Error decoding address”}]}`
///
/// * If an address in the list is valid but has never been part of a transaction
/// it will return: `“balances”:[{“ack”:0,“saved”:0,“err”:“Address has not had a
/// transaction”}]`
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::testnet_node();
///   let fct_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
///   let response = balance::multiple_fct_balances(&client, vec![fct_address])
///                             .await
///                             .expect("Fetching query");
///   assert!(response.success());
/// }
/// ```
pub async fn multiple_fct_balances(
    api: &Factom,
    addresses: Vec<&str>,
) -> Result<ApiResponse<Balances>> {
    let mut req = ApiRequest::new("multiple-fct-balances");
    req.params.insert("addresses".to_string(), json!(addresses));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// entry-credit-balance and factoid-balance functions
#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    pub balance: usize,
}

/// Struct for deserialising multiple-fct-balances and multiple-ec-balances
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipleBalances {
    pub currentheight: usize,
    pub lastsavedheight: usize,
    pub balances: Vec<Balances>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balances {
    #[serde(default)]
    pub ack: usize,
    #[serde(default)]
    pub saved: usize,
    #[serde(default)]
    pub err: String,
}
