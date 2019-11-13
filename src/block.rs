//! The Block module contains all api methods which query block data

use super::*;

/// Anchortype is a required argument in the anchors function 
pub enum AnchorType {
  Hash(String),
  Height(usize)
}

impl Factom {
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
/// let factom = Factom::new();
/// let query = factom.ablock_by_height(2)
///               .map(|response| response)
///               .map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());
/// ```
  pub async fn ablock_by_height(self, height: u32)
    -> Result<ApiResponse<ABlockHeightResult>>
  {
    let mut req =  ApiRequest::new("ablock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = self.factomd_call(req).await;
    parse(response).await
  }


/// Retrieve a specified admin block given its merkle root key.
/// # Example
/// ```
/// use factom::*;
/// 
/// let keymr = "9f9b2d68e7f018a272e9331765ac8d353c7f58c6f18685405b5286353b58daee";
/// let factom = Factom::new();
/// let query = factom
///               .admin_block(keymr)
///               .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn admin_block(self, keymr: &str)
    -> Result<ApiResponse<ABlockResult>>
  {
    let mut req =  ApiRequest::new("admin-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = self.factomd_call(req).await;
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
/// let height = Anchor::height(200);
/// let factom = Factom::new();
/// let query = factom
///               .anchor(height)
///               .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn anchors(self, target: AnchorType)
    -> Result<ApiResponse<Anchor>>
  {
    let mut req =  ApiRequest::new("anchors");
    match target {
      AnchorType::Hash(h) => {req.params.insert("hash".to_string(), json!(h));},
      AnchorType::Height(i) => {req.params.insert("height".to_string(), json!(i));}
    }
    let response = self.factomd_call(req).await;
    parse(response).await
  }


/// Retrieve a directory block given only its height.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///       .dblock_by_height(2)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
pub async fn dblock_by_height(
    self, 
    height: u32
  )-> Result<ApiResponse<DBlockHeightResult>>
  {
    let mut req =  ApiRequest::new("dblock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = self.factomd_call(req).await;
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
/// let keymr = "5b372f4622c682c984dc922983d0c769db33c376d107c74e8023446029592011";
/// let factom = Factom::new();
/// let query = factom
///       .directory_block(keymr)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());    
/// ```
  pub async fn directory_block(self, keymr: &str)
    -> Result<ApiResponse<DBlock>>
  {
    let mut req =  ApiRequest::new("directory-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = self.factomd_call(req).await;
    parse(response).await
  }
/// The directory block head is the last known directory block by factom, or in 
/// other words, the most recently recorded block. This can be used to grab the 
/// latest block and the information required to traverse the entire blockchain. 
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///       .directory_block_head()
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
   pub async fn directory_block_head(self)
    -> Result<ApiResponse<DBlockHead>>
  {
    let req =  ApiRequest::new("directory-block-head");
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/// Retrieve the entry credit block for any given height. These blocks contain 
/// entry credit transaction information.
/// # Example
/// ```
/// use factom::*;
/// 
/// let factom = Factom::new();
/// let query = factom
///       .ecblock_by_height(2)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn ecblock_by_height(self, height: u32)
    -> Result<ApiResponse<EBlockHeightResult>>
  {
    let mut req =  ApiRequest::new("ecblock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/// Retrieve a specified entry block given its merkle root key. The entry block 
/// contains 0 to many entries
/// # Example
/// ```
/// use factom::*;
/// 
/// let keymr = "1df118c1293858d1111762d6a0df92b12231c72deb14b53bfffc09b867db1f3b";
/// let factom = Factom::new();
/// let query = factom
///       .entry_block(keymr)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// 
/// ```
  pub async fn entry_block(self, keymr: &str)
    -> Result<ApiResponse<EBlock>>
  {
    let mut req =  ApiRequest::new("entry-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/// Retrieve a specified entrycredit block given its merkle root key. The numbers 
/// are minute markers.
/// # Example
/// ```
/// use factom::*;
/// 
/// let keymr = "9b9e5b67b17f2e2d3d8405ea5fc227f6bf61fcc8c2422b36b11a7fce97018521";
/// let factom = Factom::new();
/// let query = factom
///       .entry_credit_block(keymr)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn entry_credit_block(self, keymr: &str)
    -> Result<ApiResponse<EcBlockResult>>
  {
    let mut req =  ApiRequest::new("entrycredit-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = self.factomd_call(req).await;
    parse(response).await
  }

/// Retrieve a specified factoid block given its merkle root key.
/// # Example
/// ```
/// use factom::*;
/// 
/// let keymr = "aaaf4db6c1f5b716df0d63dcf9605f599d9e41eb635d8ba3e9ddfbe697ec426c";
/// let factom = Factom::new();
/// let query = factom
///       .factoid_block(keymr)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
/// ```
  pub async fn factoid_block(self, keymr: &str)
  -> Result<ApiResponse<FBlockResult>>
  {
    let mut req =  ApiRequest::new("factoid-block");
    req.params.insert("keymr".to_string(), json!(keymr));
    let response = self.factomd_call(req).await;
    parse(response).await
  }
  
/// Retrieve the factoid block for any given height. These blocks contain factoid transaction information.
/// # Example
/// ```
/// use factom::*;
/// let factom = Factom::new();
/// let query = factom
///       .fblock_by_height(1)
///       .map(|response| response).map_err(|err| err);
/// let response = fetch(query).unwrap();
/// assert!(response.success());  
///```
  pub async fn fblock_by_height(self, height: u32)
    -> Result<ApiResponse<FBlockResult>>
  {
    let mut req =  ApiRequest::new("fblock-by-height");
    req.params.insert("height".to_string(), json!(height));
    let response = self.factomd_call(req).await;
    parse(response).await
  }
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
  pub identityadminchainid: String,
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
  pub ethereum: Ethereuem
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
  pub txindex: usize
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bitcoin {
  pub transactionhash: String,
  pub blockhash: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MerkleBranch {
  pub left: String,
  pub right: String,
  pub top: String
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
  pub keymr: String
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
