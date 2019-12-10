use super::*;
use std::collections::HashMap;

/// This api call is used to find the status of a transaction, whether it be a 
/// factoid, reveal entry, or commit entry. When using this, you must specify the 
/// type of the transaction by giving the chainid field 1 of 3 values:
/// 
/// * f for factoid transactions
/// * c for entry credit transactions (commit entry/chain)
/// * <ChainID> for reveal entry/chain
/// 
/// The status types returned are as follows:0
/// 
/// * “Unknown” : Not found anywhere
/// * “NotConfirmed” : Found on local node, but not in network (Holding Map)
/// * “TransactionACK” : Found in network, but not written to the blockchain 
/// yet (ProcessList)
/// * “DBlockConfirmed” : Found in Blockchain
/// 
/// You may also provide the full marshaled transaction, instead of a hash, and it 
/// will be hashed for you.
/// 
/// The responses vary based on the type:
/// 
/// ### Entries
/// Requesting an entry requires you to specify if the hash you provide is a commit 
/// or an entry hash. The chainid field is used to specify this. If you are 
/// searching for a commit, put c as the chainid field, otherwise, put the chainid 
/// that the entry belongs too.
/// 
/// For commit/reveal acks, the response has 2 sections, one for the commit, one 
/// for the reveal. If you provide the entryhash and chainid, both will be 
/// filled (if found). If you only provide the commit txid and c as the chainid, 
/// then only the commitdata is guaranteed to come back with data. The committxid
///  and entryhash fields correspond to the commitdata and entrydata objects.
/// 
/// ### Factoid Transactions
/// 
/// The hash field for a factoid transaction is equivalent to txid. To indicate 
/// the hash is a factoid transaction, put f in the chainid field and the 
/// txid in the hash field.
/// 
/// The response will look different than entry related ack calls.
/// 
/// ### Extra notes:
/// 
/// Why c? It is short for 
/// 000000000000000000000000000000000000000000000000000000000000000c,
/// which is the chainid for all entry credit blocks. All commits are placed in the 
/// entry credit block (assuming they are valid and are properly paid for)
/// 
/// Why f? It is short for 
/// 000000000000000000000000000000000000000000000000000000000000000f, 
/// which is the chainid for all factoid blocks. All factoid transactions are placed 
/// in the factoid (assuming they are valid)
/// 
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let hash = "e96cca381bf25f6dd4dfdf9f7009ff84ee6edaa3f47f9ccf06d2787482438f4b";
///   let chainid = "f9164cd66af9d5773b4523a510b5eefb9a5e626480feeb6671ef2d17510ca300";
///   let committxid = "4876ffeb8f95b72911b4a5115dc8a9fbb89d874db2263a75a9062f37bbbf1fa7";
///   let response = tx::ec_ack(&client, hash, chainid, None).await.unwrap();
///   dbg!(&response);
///   assert_eq!(response.result.committxid, committxid);
/// }
/// ```
pub async fn ec_ack(
  api: &Factom, 
  hash: &str, 
  chainid: &str, 
  full_transaction: Option<&str>
) -> Result<ApiResponse<EntryAck>> 
{
  let mut req =  ApiRequest::new("ack");
  req.params.insert("hash".to_string(), json!(hash));
  req.params.insert("chainid".to_string(), json!(chainid));
  if let Some(tx) = full_transaction{
    req.params.insert("fulltransaction".to_string(), json!(tx));
  }
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// See documentation for ec_ack
pub async fn fct_ack(
  api: &Factom, 
  hash: &str, 
  full_transaction: Option<&str>
) -> Result<ApiResponse<FactoidAck>> 
{
  let mut req =  ApiRequest::new("ack");
  req.params.insert("hash".to_string(), json!(hash));
  req.params.insert("chainid".to_string(), json!("f"));
  if let Some(tx) = full_transaction{
    req.params.insert("fulltransaction".to_string(), json!(tx));
  }
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Submit a factoid transaction. The transaction hex encoded string is 
/// documented here: 
/// [Github Documentation](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#factoid-transaction)
/// 
/// The factoid-submit API takes a specifically formatted message encoded in hex 
/// that includes signatures. If you have a factom-walletd instance running, you 
/// can construct this factoid-submit API call with compose-transaction which 
/// takes easier to construct arguments.
///
/// See the examples folder for an example of the full workflow
pub async fn factoid_submit(
  api: &Factom, 
  transaction: &str
)-> Result<ApiResponse<FctSubmit>>
{
  let mut req =  ApiRequest::new("factoid-submit");
  req.params.insert("transaction".to_string(), json!(transaction));
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Retrieve details of a factoid transaction using a transaction’s hash 
/// (or corresponding transaction id).
/// 
/// Note that information regarding the
/// 
/// * directory block height,
/// * directory block keymr, and
/// * transaction block keymr
/// 
/// are also included.
/// 
/// The "blockheight” parameter in the response will always be 0 when using this 
/// call, refer to “includedindirectoryblockheight” if you need the height.
/// 
/// ### Notes 
/// 
/// This call will also accept an entry hash as input, in which case the returned 
/// data concerns the entry. The returned fields and their format are shown in the 
/// 2nd Example Response at right.
/// 
/// If the input hash is non-existent, the returned fields will be as follows:
/// 
/// * “includedintransactionblock”:“”
/// * “includedindirectoryblock”:“”
/// * “includedindirectoryblockheight”:-1
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let tx_hash ="a740ac489821399eac070cf3ba681bc4cb78058a5fedee5e407762aa3d1de158";
///   let response = tx::transaction(&client, tx_hash)
///                             .await
///                             .unwrap();
///   dbg!(&response);
///   assert_eq!(response.result.includedindirectoryblockheight, 220000);
/// }
/// ```
pub async fn transaction(
  api: &Factom, 
  hash: &str
)-> Result<ApiResponse<Transaction>>
{
  let mut req =  ApiRequest::new("transaction");
  req.params.insert("hash".to_string(), json!(hash));
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// Returns an array of factoid transactions that have not yet been recorded in the 
/// blockchain, but are known to the system.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let response = tx::pending_transactions(&client, None)
///                             .await
///                             .unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn pending_transactions(
  api: &Factom, 
  address: Option<&str>
)-> Result<ApiResponse<Vec<PendingTx>>>
{
  let mut req =  ApiRequest::new("pending-transactions");
  if let Some(add) = address {
    req.params.insert("address".to_string(), json!(add));
  }
  let response = factomd_call(api, req).await;
  parse(response).await
}

/// When adding entry credit outputs, the amount given is in factoshis, not entry credits. This means math is required to determine the correct amount of factoshis to pay to get X EC.
/// 
/// (ECRate * ECTotalOutput)
/// 
/// In our case, the rate is 1000, meaning 1000 entry credits per factoid. We added 10 entry credits, so we need 1,000 * 10 = 10,000 factoshis
/// 
/// To get the ECRate search in the search bar above for “entry-credit-rate”
/// 
/// Add ec output is a part of sending a transaction to see a full example check the 
/// examples folder.
pub async fn add_ec_output(
  api: &Factom, 
  txname: &str, 
  address: &str, 
  amount: u64
) -> Result<ApiResponse<NewTx>>
{
  let mut req =  ApiRequest::new("add-ec-output");
  req.params.insert("tx-name".to_string(), json!(txname));
  req.params.insert("address".to_string(), json!(address));
  req.params.insert("amount".to_string(), json!(amount));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Addfee is a shortcut and safeguard for adding the required additional factoshis to covert the fee. The fee is displayed in the returned transaction after each step, but addfee should be used instead of manually adding the additional input. This will help to prevent overpaying.
/// 
/// Addfee will complain if your inputs and outputs do not match up. For example, in the steps above we added the inputs first. This was done intentionally to show a case of overpaying. Obviously, no one wants to overpay for a transaction, so addfee has returned an error and the message: ‘Inputs and outputs don’t add up’. This is because we have 2,000,000,000 factoshis as input and only 1,000,000,000 + 10,000 as output. Let’s correct the input by doing 'add-input’, and putting 1000010000 as the amount for the address. It will overwrite the previous input.
/// 
/// Run the addfee again, and the feepaid and feerequired will match up
/// 
/// # Example
/// 
/// Add fee is a part of sending a transaction to see a full example check the 
/// examples folder.
pub async fn add_fee(
  api: &Factom, 
  txname: &str, 
  address: &str
) -> Result<ApiResponse<Tx>>
{
  let mut req =  ApiRequest::new("add-fee");
  req.params.insert("tx-name".to_string(), json!(txname));
  req.params.insert("address".to_string(), json!(address));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Adds an input to the transaction from the given address. The public address is 
/// given, and the wallet must have the private key associated with the address to 
/// successfully sign the transaction.
/// 
/// The input is measured in factoshis, so to send ten factoids, you must input 
/// 1,000,000,000 factoshis (without commas in JSON)
/// 
/// # Example
/// 
/// Add input is used in the process of sending a transaction, to see the full 
/// process as an example check the examples folder.
pub async fn add_input(
  api: &Factom, 
  txname: &str, 
  address: &str, 
  amount: u64
) -> Result<ApiResponse<Tx>>
{
  let mut req =  ApiRequest::new("add-input");
  req.params.insert("tx-name".to_string(), json!(txname));
  req.params.insert("address".to_string(), json!(address));
  req.params.insert("amount".to_string(), json!(amount));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Adds a factoid address output to the transaction. Keep in mind the output is 
/// done in factoshis. 1 factoid is 1,000,000,000 factoshis.
/// 
/// So to send ten factoids, you must send 1,000,000,000 factoshis 
/// (no commas in JSON).
/// 
/// #Example
/// 
/// Add Output is used in the transaction process, the full process and an example of 
/// this function being used can be found in the examples folder
pub async fn add_output(
  api: &Factom, 
  txname: &str, 
  address: &str, 
  amount: u64
) -> Result<ApiResponse<Tx>>
{
  let mut req =  ApiRequest::new("add-output");
  req.params.insert("tx-name".to_string(), json!(txname));
  req.params.insert("address".to_string(), json!(address));
  req.params.insert("amount".to_string(), json!(amount));
  let response = walletd_call(api, req).await;
  parse(response).await
}
/// Deletes a working transaction in the wallet. The full transaction will be 
/// returned, and then deleted.
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let tx_name = "test-tx";
///   tx::new_transaction(&client, tx_name).await.unwrap();
///   let response = tx::delete_transaction(&client,tx_name).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn delete_transaction(
  api: &Factom, 
  tx_name: &str
)-> Result<ApiResponse<DeleteTx>>
{
  let mut req =  ApiRequest::new("delete-transaction");
  req.params.insert("tx-name".to_string(), json!(tx_name));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// This will create a new transaction. The txid is in flux until the final 
/// transaction is signed. Until then, it should not be used or recorded.
/// 
/// When dealing with transactions all factoids are represented in factoshis. 
/// 1 factoid is 1e8 factoshis, meaning you can never send anything less than 0 to 
/// a transaction (0.5).
/// # Example
/// ```
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let tx_name = "test-tx";
///   tx::new_transaction(&client, tx_name).await.unwrap();
///   let response = tx::delete_transaction(&client,tx_name).await.unwrap();
///   dbg!(&response);
///   assert!(response.success());
/// }
/// ```
pub async fn new_transaction(
  api: &Factom, 
  tx_name: &str
)-> Result<ApiResponse<NewTx>>
{
  let mut req =  ApiRequest::new("new-transaction");
  req.params.insert("tx-name".to_string(), json!(tx_name));
  let response = walletd_call(api, req).await;
  parse(response).await
}
/// Signs the transaction. It is now ready to be executed.
/// 
/// sign_transaction is used in the transaction process, the full process can be 
/// found in the examples folder
pub async fn sign_transaction(
  api: &Factom, 
  tx_name: &str
)-> Result<ApiResponse<Tx>>
{
  let mut req =  ApiRequest::new("sign-transaction");
  req.params.insert("tx-name".to_string(), json!(tx_name));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// When paying from a transaction, you can also make the receiving transaction 
/// pay for it. Using sub fee, you can use the receiving address in the parameters, 
/// and the fee will be deducted from their output amount.
/// 
/// This allows a wallet to send all it’s factoids, by making the input and output 
/// the remaining balance, then using sub fee on the output address.
/// 
/// #Example
/// 
/// sub_fee is used in the transaction process, the full process and an example of 
/// this function being used can be found in the examples folder
pub async fn sub_fee(
  api: &Factom, 
  tx_name: &str, 
  address: &str
)-> Result<ApiResponse<Tx>>
{
  let mut req =  ApiRequest::new("sub-fee");
  req.params.insert("tx-name".to_string(), json!(tx_name));
  req.params.insert("address".to_string(), json!(address));
  let response = walletd_call(api, req).await;
  parse(response).await
}

/// Lists all the current working transactions in the wallet. These are transactions 
/// that are not yet sent.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///             .tmp_transactions()
///             .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
pub async fn tmp_transactions(api: &Factom)
  -> Result<ApiResponse<TmpTransactions>>
{
  let req =  ApiRequest::new("tmp-transactions");
  let response = walletd_call(api, req).await;
  parse(response).await
} 

/// There are a few ways to search for a transaction
/// 
/// ### Using a Range
/// This will retrieve all transactions within a given block height range.
/// 
/// ### By TxID
/// This will retrieve a transaction by the given TxID. This call is the fastest 
/// way to retrieve a transaction, but it will not display the height of the 
/// transaction. If a height is in the response, it will be 0. To retrieve the 
/// height of a transaction, use the 'By Address’ method
/// 
/// This call in the backend actually pushes the request to factomd. For a more 
/// informative response, it is advised to use the factomd transaction method
/// 
/// ### By Address
/// Retrieves all transactions that involve a particular address.
pub async fn transactions(
  api: &Factom, 
  filter: SearchBy
)-> Result<ApiResponse<Transactions>>
{
  let mut req =  ApiRequest::new("transactions");
  match filter {
    SearchBy::Txid(txid) => {
              req.params.insert("txid".to_string(), json!(txid));
              }
    SearchBy::Address(address) => {
              req.params.insert("address".to_string(), json!(address));
              }
    SearchBy::Range(start, end) => {
              let mut range = HashMap::new();
              range.insert("start", json!(start));
              range.insert("end", json!(end));
              req.params.insert("range".to_string(),json!(range));
              }
  };      
  let response = walletd_call(api, req).await;
  parse(response).await
} 

/// Search options for the transactions function
/// * Range(usize, usize)
/// * Txid(&str)
/// * Address(&str)
pub enum SearchBy{
  Range(usize, usize),
  Txid(&'static str),
  Address(&'static str)
}

/// factoid-submit function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FctSubmit {
  pub message: String,
  pub txid: String,
}

/// transaction function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
  #[serde(default)]
  pub factoidtransaction: Factoidtransaction,
  pub includedintransactionblock: String,
  pub includedindirectoryblock: String,
  pub includedindirectoryblockheight: isize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Factoidtransaction {
  pub millitimestamp: usize,
  pub inputs: Option<Vec<Input>>,
  pub outputs: Option<Vec<Output>>,
  pub outecs: Option<Vec<::serde_json::Value>>,
  pub rcds: Vec<String>,
  pub sigblocks: Vec<Sigblock>,
  pub blockheight: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
  pub amount: usize,
  pub address: String,
  pub useraddress: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub amount: usize,
  pub address: String,
  pub useraddress: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcOutput {
  pub amount: usize,
  pub address: String,
  pub useraddress: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sigblock {
  pub signatures: Vec<String>,
}

/// pending-transactions function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingTxs {
  pub pendingtx: Vec<PendingTx>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingTx {
  pub transactionid: String,
  pub status: String,
  pub inputs: Option<Vec<Input>>,
  pub outputs: Option<Vec<Output>>,
  pub ecoutputs: Option<Vec<EcOutput>>,
  pub fees: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAck {
  pub committxid: String,
  pub entryhash: String,
  pub commitdata: Commitdata,
  pub entrydata: Entrydata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactoidAck {
    txid: String,
    transactiondate: i64,
    transactiondatestring: String,
    blockdate: i64,
    blockdatestring: String,
    status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commitdata {
  pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entrydata {
  pub status: String,
}

/// new-transaction and add-ec-output functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewTx {
  pub feesrequired: usize,
  pub signed: bool,
  pub name: String,
  pub timestamp: i64,
  pub totalecoutputs: i64,
  pub totalinputs: i64,
  pub totaloutputs: i64,
  pub inputs: Option<Vec<TxInput>>,
  pub outputs: Option<Vec<TxOutput>>,
  pub ecoutputs: Option<Vec<Ecoutput>>,
  pub txid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxInput {
  pub address: String,
  pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxOutput {
  pub address: String,
  pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ecoutput {
  pub address: String,
  pub amount: i64,
}

/// add-input, add-output, add-fee, sub-fee, sign-transaction functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx {
  pub feespaid: i64,
  pub feesrequired: i64,
  pub signed: bool,
  pub name: String,
  pub timestamp: i64,
  pub totalecoutputs: i64,
  pub totalinputs: i64,
  pub totaloutputs: i64,
  pub inputs: Vec<TxInput>,
  pub outputs: Vec<TxOutput>,
  pub ecoutputs: Vec<Ecoutput>,
  pub txid: String,
}

/// delete-transaction function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteTx {
  pub signed: bool,
  pub name: String,
  pub timestamp: i64,
  pub totalecoutputs: i64,
  pub totalinputs: i64,
  pub totaloutputs: i64,
  pub inputs: ::serde_json::Value,
  pub outputs: ::serde_json::Value,
  pub ecoutputs: ::serde_json::Value,
}

/// tmp-transactions function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmpTransactions {
  pub transactions: Vec<TmpTransaction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmpTransaction {
  #[serde(rename = "tx-name")]
  pub tx_name: String,
  pub txid: String,
  pub totalinputs: i64,
  pub totaloutputs: i64,
  pub totalecoutputs: i64,
}

/// transactions function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transactions {
  pub transactions: Vec<Txs>,
}

/// Individual transactions from the transactions function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Txs {
  pub blockheight: i64,
  pub feespaid: i64,
  pub signed: bool,
  pub timestamp: i64,
  pub totalecoutputs: i64,
  pub totalinputs: i64,
  pub totaloutputs: i64,
  pub inputs: Vec<Input>,
  pub outputs: ::serde_json::Value,
  pub ecoutputs: Vec<Ecoutput>,
  pub txid: String,
}

