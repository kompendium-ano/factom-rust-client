//! Contains all api methods which query block data
use super::*;

/// Anchortype is a required argument in the anchors function
pub enum AnchorType {
    Hash(String),
    Height(usize),
}

/// Retrieve administrative blocks for any given height.
///
/// The admin block contains data related to the identities within the factom
/// system and the decisions the system makes as it builds the block chain. The
/// abentries’ (admin block entries) in the JSON response can be of various types,
/// the most common is a directory block signature (DBSig). A majority of the
/// federated servers sign every directory block, meaning every block after m5
/// will contain 5 DBSigs in each admin block.
///
/// The ABEntries are detailed
/// [here](https://github.com/FactomProject/FactomDocs/blob/master/factomDataStructureDetails.md#adminid-bytes)
///
/// # Example
/// ```
/// use factom::*;  
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let height = 220000;
///   let prevbackrefhash = "dbe20345a773a593d06cce65cf68d976011063208d54927433fac3c2b10f06b2";
///   let response = block::ablock_by_height(&client, height)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.ablock.header.prevbackrefhash, prevbackrefhash);
/// }
/// ```
pub async fn ablock_by_height(
    api: &Factom,
    height: u32,
) -> Result<ApiResponse<ABlockHeightResult>> {
    let mut req = ApiRequest::new("ablock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve a specified admin block given its merkle root key.
/// # Example
/// ```
/// use factom::*;

/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();;
///   let ablock_keymr = "072334d94450296810cb647172812a5dc7ce518d29ecab411d47494d38ca4c88";
///   let prevbackrefhash = "dbe20345a773a593d06cce65cf68d976011063208d54927433fac3c2b10f06b2";  
///   let response = block::admin_block(&client, ablock_keymr)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.ablock.header.prevbackrefhash, prevbackrefhash);
/// }
/// ```
pub async fn admin_block(api: &Factom, keymr: &str) -> Result<ApiResponse<ABlockResult>> {
    let mut req = ApiRequest::new("admin-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = factomd_call(api, req).await;
    parse(response).await
}

///  Retrieve information about the directory block anchors that have been confirmed
///  on Bitcoin and Ethereum.
///
/// Parameter options: - "height" - the directory block height (integer) to request
/// anchors for - "hash" - the object’s hash (hex string) to request anchors for
/// (e.g. entry hash, entry block keymr, factoid block keymr, admin block lookup
/// hash, entry credit block header hash, or directory block keymr)
///
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let directoryblockheight = 220000;
///   let response = block::anchors(&client, block::AnchorType::Height(directoryblockheight))
///                                 .await
///                                 .expect("API Request");
///    assert_eq!(response.result.directoryblockheight, directoryblockheight);
/// }
/// ```
pub async fn anchors(api: &Factom, target: AnchorType) -> Result<ApiResponse<Anchor>> {
    let mut req = ApiRequest::new("anchors");
    match target {
        AnchorType::Hash(h) => {
            req.params.insert("hash".to_string(), json!(h));
        }
        AnchorType::Height(i) => {
            req.params.insert("height".to_string(), json!(i));
        }
    }
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve a directory block given only its height.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let height = 220000;
///   let bodymr = "a575d8f07c725cd8c523c7881121dac330a29b6d5edcf2da0b0f7d2d5cbba256";
///   let response = block::dblock_by_height(&client, height)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.dblock.header.bodymr, bodymr);
/// }

/// ```
pub async fn dblock_by_height(
    api: &Factom,
    height: u32,
) -> Result<ApiResponse<DBlockHeightResult>> {
    let mut req = ApiRequest::new("dblock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Every directory block has a KeyMR (Key Merkle Root), which can be used to
/// retrieve it. The response will contain information that can be used to
/// navigate through all transactions (entry and factoid) within that block. The
/// header of the directory block will contain information regarding the previous
/// directory block’s keyMR, directory block height, and the timestamp.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let bodymr = "0f53b422b4ecf6753c4482fe21e958ec5e565ba44bcaa5253d0227bf2789b5dd";
///   let prevblockkeymr = "86b1b7b9307b45628ef6c9f78d935f77d7bc2b156a8828be98109f72b22792f5";
///   let response = block::directory_block(&client, bodymr)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.header.prevblockkeymr, prevblockkeymr);
/// }
/// ```
pub async fn directory_block(api: &Factom, keymr: &str) -> Result<ApiResponse<DBlock>> {
    let mut req = ApiRequest::new("directory-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = factomd_call(api, req).await;
    parse(response).await
}
/// The directory block head is the last known directory block by factom, or in
/// other words, the most recently recorded block. This can be used to grab the
/// latest block and the information required to traverse the entire blockchain.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let response = block::directory_block_head(&client)
///                             .await
///                             .expect("Api Request");
///   assert!(response.success());
/// }
/// ```
pub async fn directory_block_head(api: &Factom) -> Result<ApiResponse<DBlockHead>> {
    let req = ApiRequest::new("directory-block-head");
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve the entry credit block for any given height. These blocks contain
/// entry credit transaction information.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let height = 220000;
///   let bodyhash = "c4b5a9d81028c5a87e7ff1d5ee8e1446f337ac6c57eca185a533cadeb653acee";
///   let response = block::ecblock_by_height(&client, height)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.ecblock.header.bodyhash, bodyhash);
/// }

/// ```
pub async fn ecblock_by_height(
    api: &Factom,
    height: u32,
) -> Result<ApiResponse<EBlockHeightResult>> {
    let mut req = ApiRequest::new("ecblock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve a specified entry block given its merkle root key. The entry block
/// contains 0 to many entries
/// # Example
/// ```
/// use factom::*;

/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let keymr = "45a6c1ef9e48e84dbbb6911a46262bbb24fe9d1b0123d768c9d9c63170b7b9c3";
///   let chainid = "a642a8674f46696cc47fdb6b65f9c87b2a19c5ea8123b3d2f0c13b6f33a9d5ef";
///   let response = block::entry_block(&client, keymr)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.header.chainid, chainid);
/// }

/// ```
pub async fn entry_block(api: &Factom, keymr: &str) -> Result<ApiResponse<EBlock>> {
    let mut req = ApiRequest::new("entry-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve a specified entrycredit block given its merkle root key. The numbers
/// are minute markers.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let keymr = "a04189d5a050503bddb3bebb9b9eda0d542498c01b2f7820c58b6786441d187b";
///   let bodyhash = "c4b5a9d81028c5a87e7ff1d5ee8e1446f337ac6c57eca185a533cadeb653acee";
///   let response = block::entry_credit_block(&client, keymr)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.ecblock.header.bodyhash, bodyhash);
/// }
/// ```
pub async fn entry_credit_block(api: &Factom, keymr: &str) -> Result<ApiResponse<EcBlockResult>> {
    let mut req = ApiRequest::new("entrycredit-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve a specified factoid block given its merkle root key.
/// # Example
/// ```
/// use factom::*;
///
/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let keymr = "08d74c568ada1f89930c02d57cb3269425873f1a55217957b500ad22765a83cc";
///   let bodymr = "faa064a185a2c677404dcc24e9428e781a19350fc8f21c7daf3d18b1d3f91412";
///   let response = block::factoid_block(&client, keymr)
///                             .await
///                             .expect("Request");
///    assert_eq!(response.result.fblock.bodymr, bodymr);
/// }
/// ```
pub async fn factoid_block(api: &Factom, keymr: &str) -> Result<ApiResponse<FBlockResult>> {
    let mut req = ApiRequest::new("factoid-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// Retrieve the factoid block for any given height. These blocks contain factoid transaction information.
/// # Example
/// ```
/// use factom::*;

/// #[tokio::main]
/// async fn main() {
///   let client = Factom::open_node();
///   let height = 220000;
///   let bodymr = "faa064a185a2c677404dcc24e9428e781a19350fc8f21c7daf3d18b1d3f91412";
///   let response = block::fblock_by_height(&client, height)
///                             .await
///                             .expect("Request");
///   dbg!(&response);
///   assert_eq!(response.result.fblock.bodymr, bodymr);
/// }
///```
pub async fn fblock_by_height(api: &Factom, height: u32) -> Result<ApiResponse<FBlockResult>> {
    let mut req = ApiRequest::new("fblock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = factomd_call(api, req).await;
    parse(response).await
}

/// ablock-by-height function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ABlockHeightResult {
    pub ablock: AblockHeight,
    pub rawdata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AblockHeight {
    pub header: Header,
    pub abentries: Vec<ABHeightentry>,
    pub backreferencehash: String,
    pub lookuphash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ABHeightentry {
    #[serde(default)]
    pub identityadminchainid: String,
    #[serde(default)]
    pub prevdbsig: Prevdbsig,
}

/// admin block function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ABlockResult {
    pub ablock: Ablock,
    pub rawdata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ablock {
    pub header: Header,
    pub abentries: Vec<Abentry>,
    pub backreferencehash: String,
    pub lookuphash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub prevbackrefhash: String,
    pub dbheight: usize,
    pub headerexpansionsize: usize,
    pub headerexpansionarea: String,
    pub messagecount: usize,
    pub bodysize: usize,
    pub adminchainid: String,
    pub chainid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abentry {
    pub identityadminchainid: Option<String>,
    pub prevdbsig: Option<Prevdbsig>,
    pub minutenumber: Option<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prevdbsig {
    #[serde(rename = "pub")]
    pub pub_field: String,
    pub sig: String,
}

/// anchors function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Anchor {
    pub directoryblockheight: usize,
    pub directoryblockkeymr: String,
    #[serde(default)]
    pub bitcoin: Bitcoin,
    pub ethereum: Ethereuem,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ethereuem {
    pub recordheight: usize,
    pub dbheightmax: usize,
    pub dbheightmin: usize,
    pub windowmr: String,
    pub merklebranch: Vec<MerkleBranch>,
    pub contractaddress: String,
    pub txid: String,
    pub blockhash: String,
    pub txindex: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bitcoin {
    pub transactionhash: String,
    pub blockhash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MerkleBranch {
    pub left: String,
    pub right: String,
    pub top: String,
}

/// dblock-by-height function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBlockHeightResult {
    pub dblock: DblockHeight,
    pub rawdata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DblockHeight {
    pub header: DBlockHeightHeader,
    pub dbentries: Vec<Dbentry>,
    pub dbhash: String,
    pub keymr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBlockHeightHeader {
    pub version: u8,
    pub networkid: usize,
    pub bodymr: String,
    pub prevkeymr: String,
    pub prevfullhash: String,
    pub timestamp: usize,
    pub dbheight: usize,
    pub blockcount: usize,
    pub chainid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dbentry {
    pub chainid: String,
    pub keymr: String,
}

/// directory block function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBlock {
    pub header: DBlockHeader,
    pub entryblocklist: Vec<Entryblocklist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBlockHeader {
    pub prevblockkeymr: String,
    pub sequencenumber: usize,
    pub timestamp: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entryblocklist {
    pub chainid: String,
    pub keymr: String,
}

/// directory-block-head function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBlockHead {
    pub keymr: String,
}

/// ecblock-by-height function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EBlockHeightResult {
    pub ecblock: EcBlock,
    pub rawdata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcBlock {
    pub header: ECHeightHeader,
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECHeightHeader {
    pub bodyhash: String,
    pub prevheaderhash: String,
    pub prevfullhash: String,
    pub dbheight: usize,
    pub headerexpansionarea: String,
    pub objectcount: usize,
    pub bodysize: usize,
    pub chainid: String,
    pub ecchainid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub serverindexnumber: Option<u8>,
    pub version: Option<u8>,
    pub millitime: Option<String>,
    pub entryhash: Option<String>,
    pub credits: Option<usize>,
    pub ecpubkey: Option<String>,
    pub sig: Option<String>,
    pub number: Option<usize>,
}

/// entry-block function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EBlock {
    pub header: EBlockHeader,
    pub entrylist: Vec<Entrylist>,
}

/// entry-block function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EBlockHeader {
    pub blocksequencenumber: usize,
    pub chainid: String,
    pub prevkeymr: String,
    pub timestamp: usize,
    pub dbheight: usize,
}

/// entry-block function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entrylist {
    pub entryhash: String,
    pub timestamp: usize,
}

/// entrycredit-block function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcBlockResult {
    pub ecblock: Ecblock,
    pub rawdata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ecblock {
    pub header: EcBlockHeader,
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcBlockHeader {
    pub bodyhash: String,
    pub prevheaderhash: String,
    pub prevfullhash: String,
    pub dbheight: usize,
    pub headerexpansionarea: String,
    pub objectcount: usize,
    pub bodysize: usize,
    pub chainid: String,
    pub ecchainid: String,
}

// factoid-block and fblock-by-height functions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FBlockResult {
    pub fblock: Fblock,
    pub rawdata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fblock {
    pub bodymr: String,
    pub prevkeymr: String,
    pub prevledgerkeymr: String,
    pub exchrate: usize,
    pub dbheight: usize,
    pub transactions: Vec<Transaction>,
    pub chainid: String,
    pub keymr: String,
    pub ledgerkeymr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub blockheight: usize,
    pub millitimestamp: usize,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub outecs: Vec<::serde_json::Value>,
    pub rcds: Vec<String>,
    pub sigblocks: Vec<Sigblock>,
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
pub struct Sigblock {
    pub signatures: Vec<String>,
}
