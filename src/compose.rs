//! Functions that compose transactions, entries and identities.
use super::*;
use factomd::str_to_hex;
use std::collections::HashMap;

/// This method, compose-chain, will return the appropriate API calls to create a
/// chain in factom. You must first call the commit-chain, then the reveal-chain
/// API calls. To be safe, wait a few seconds after calling commit.
///
/// Note: The firstentry fields are automatically hex encoded for the server to
/// process.
pub async fn compose_chain(
    api: &Factom,
    extids: Vec<&str>,
    content: &str,
    ecpub: &str,
) -> Result<ApiResponse<Compose>> {
    let mut req = ApiRequest::new("compose-chain");
    let hex_content = str_to_hex(content);
    let mut hex_extids = Vec::new();
    for extid in extids {
        hex_extids.push(str_to_hex(extid));
    }
    let chain = json!({
      "firstentry": {
        "extids": hex_extids,
        "content": hex_content
      }
    });
    req.params.insert("chain".to_string(), chain);
    req.params.insert("ecpub".to_string(), json!(ecpub));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// This method, compose-entry, will return the appropriate API calls to create an
/// entry in factom. You must first call the commit-entry, then the reveal-entry
/// API calls. To be safe, wait a few seconds after calling commit.
///
/// Note: The entry fields are automatically hex encoded for the server to process.
pub async fn compose_entry(
    api: &Factom,
    chainid: &str,
    extids: Vec<&str>,
    content: &str,
    ecpub: &str,
) -> Result<ApiResponse<Compose>> {
    let mut req = ApiRequest::new("compose-entry");
    let mut hex_extids = Vec::new();
    for extid in extids {
        hex_extids.push(str_to_hex(extid));
    }
    let entry = json!({
    "chainid": chainid,
    "extids": hex_extids,
    "content": str_to_hex(content)
    });
    req.params.insert("entry".to_string(), entry);
    req.params.insert("ecpub".to_string(), json!(ecpub));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// Compose transaction marshals the transaction into a hex encoded string. The
/// string can be inputted into the factomd API factoid-submit to be sent to
/// the network.
///
/// See the examples folder for a demonstration of the full workflow.
pub async fn compose_transaction(api: &Factom, tx_name: &str) -> Result<ApiResponse<ComposeTx>> {
    let mut req = ApiRequest::new("compose-transaction");
    req.params.insert("tx-name".to_string(), json!(tx_name));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// This request allows one identity to state an attribute about another identity
/// and publish that entry to any existing chain. An attribute is a set of generic
/// key:value pairs that can be assigned to an identity and is flexible enough to
/// accommodate many different use-cases. In the example request, an identity is
/// giving itself an attribute describing its current email address, and writing
/// that entry to its own identity chain. Each attribute must be in the format of
/// {"key":KEY, "value":VALUE} where KEY and VALUE can be of any valid JSON type.
/// Each attribute you wish to assign must be put into an array, even if it is
/// just a single key/value pair.
/// For example: [{"key":"worksAt", "value":"Factom Inc."}, {"key":"isNice",
/// "value":true}] would be a valid attributes array to use as a parameter. If the
/// wallet is encrypted, it must be unlocked prior to using this command.
///
/// The entry will be constructed based on the information you included in the request.
///
/// receiver-chainid - the Chain ID for the identity being assigned the attribute
///
/// destination-chainid - the Chain ID where the attribute entry will be written.
/// Could be any existing chain, dream big.
///
/// attributes - the array of attributes that you are assigning to the receiver
///
/// signerkey - the public identity key being used to sign the entry. Must be
/// stored in the wallet already and should be a currently valid key for the
/// signer identity.
///
/// signer-chainid - the Identity Chain of the signing party (who is giving the
/// attribute)
///
/// The response you receive is similar to the compose-entry response. You must
/// first call the commit-entry, then the reveal-entry API calls. To be safe,
/// wait a few seconds after calling commit.
pub async fn compose_id_attribute<T>(
    api: &Factom,
    receiver_chain: &str,
    destination_chain: &str,
    attributes: Vec<(T, T)>,
    signer_key: &str,
    signer_chainid: &str,
    ecpub: &str,
    force: bool,
) -> Result<ApiResponse<Compose>>
where
    T: Serialize,
{
    let mut req = ApiRequest::new("compose-identity-attribute");
    let mut attr_list = vec![HashMap::new()];
    req.params
        .insert("receiver-chainid".to_string(), json!(receiver_chain));
    req.params
        .insert("destination-chainid".to_string(), json!(destination_chain));
    req.params
        .insert("signerkey".to_string(), json!(signer_key));
    req.params
        .insert("signer-chainid".to_string(), json!(signer_chainid));
    req.params.insert("ecpub".to_string(), json!(ecpub));
    req.params.insert("force".to_string(), json!(force));
    for attr in attributes {
        let mut map = HashMap::new();
        map.insert("key".to_string(), attr.0);
        map.insert("value".to_string(), attr.1);
        attr_list.push(map);
    }
    req.params
        .insert("attributes".to_string(), json!(attr_list));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// This method helps you endorse an attribute that has already been registered on
/// the Factom blockchain. To do this, you’ll need to create a structured entry on
/// to the Identity chain. The compose-identity-attribute-endorsement method will
/// return the API calls needed to create that entry. If the wallet is encrypted,
/// it must be unlocked prior to using this command.
///
/// An “endorsement” is a statement that one identity agrees with, recognizes, or
/// co-signs an attribute that has been given to another identity. In the example
/// request, we created an endorsement entry that points to the email attribute
/// entry we gave in the previous section. Depending on the application context
/// this could mean different things. One such use-case would be to reconfirm that
/// the attribute is still valid and that email address is still correct at present.
/// Just like attributes, endorsements are very generic so that they can be used
/// in a variety of ways.
///
/// The entry will be constructed based on the information you included in the
/// request:
///
/// destination-chainid - the Chain ID where the attribute entry will be written.
/// Could be any existing chain, dream big.
///
/// entry-hash - the entry hash of the attribute that will be endorsed
///
/// signerkey - the public identity key being used to sign the entry. Must be
/// stored in the wallet already and should be a currently valid key for the
/// signer identity.
///
/// signer-chainid - the Identity Chain of the signing party (who is endorsing the
/// attribute located at entry-hash)
///
/// The response you receive is similar to the compose-entry response. You must
/// first call the commit-entry, then the reveal-entry API calls. To be safe,
/// wait a few seconds after calling commit.
pub async fn compose_id_attribute_endorsement(
    api: &Factom,
    destination_chain: &str,
    entry_hash: &str,
    signer_key: &str,
    signer_chainid: &str,
    ecpub: &str,
    force: bool,
) -> Result<ApiResponse<Compose>> {
    let mut req = ApiRequest::new("compose-identity-attribute-endorsement");
    req.params
        .insert("destination-chainid".to_string(), json!(destination_chain));
    req.params
        .insert("entry-hash".to_string(), json!(entry_hash));
    req.params
        .insert("signerkey".to_string(), json!(signer_key));
    req.params
        .insert("signer-chainid".to_string(), json!(signer_chainid));
    req.params.insert("ecpub".to_string(), json!(ecpub));
    req.params.insert("force".to_string(), json!(force));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// The compose-identity-chain method will return the appropriate API calls to
/// create an identity chain in factom. The chain will be constructed based on the
/// name and public keys that you send in the request. The response you receive is
/// similar to the compose-chain response. You must first call the commit-chain,
/// then the reveal-chain API calls. To be safe, wait a few seconds after calling
/// commit. If the wallet is encrypted, it must be unlocked prior to using this
/// command.
///
/// The chain to be created will contain the identity name as the ExtIDs of the
/// first entry (a.k.a. the Chain Name) and also a JSON object in the Content
/// field signifying the identity version and initial valid keys. This first set
/// of keys are listed in order of decreasing priority. The first key is a
/// “master” key of sorts that should be kept as securely as possible and used
/// infrequently — it is the only key that can transfer complete ownership of an
/// identity. The last key in the array is the lowest priority, meaning that it
/// can be kept in less secure locations and used more frequently. A higher
/// priority key can always just replace a lower priority key that was
/// compromised or simply lost. For more information on key replacements, see the
/// compose-identity-key-replacement section.
pub async fn compose_id_chain(
    api: &Factom,
    name: Vec<&str>,
    pubkeys: Vec<&str>,
    ecpub: &str,
    force: bool,
) -> Result<ApiResponse<Compose>> {
    let mut req = ApiRequest::new("compose-identity-chain");
    req.params.insert("name".to_string(), json!(name));
    req.params.insert("pubkeys".to_string(), json!(pubkeys));
    req.params.insert("ecpub".to_string(), json!(ecpub));
    req.params.insert("force".to_string(), json!(force));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// Replacing one of an identity’s keys is done by adding a structured entry onto
/// the identity’s chain. This will need to be done if you feel that a key was
/// compromised or has been in use for too long. The compose-identity-key-replacement
/// method will return the API calls needed to create the replacement entry. The
/// response you receive is similar to the compose-entry response. You must first
/// call the commit-entry, then the reveal-entry API calls. To be safe, wait a few
/// seconds after calling commit. If the wallet is encrypted, it must be unlocked
/// prior to using this command.
///
/// The entry will be constructed based on the information you included in the
/// request.
///
/// * chainid - the ChainID of the identity chain in question
///
/// * oldkey - the public identity key for the level to be replaced
///
/// * newkey - the public identity key that will be replacing oldkey
///
/// * signerkey - the public identity key that will sign the entry and authorize the
/// replacement. This key must be stored in the wallet already and must be of the
/// same or higher priority than the oldkey in the context of the given Identity Chain.
pub async fn compose_id_key_replacement(
    api: &Factom,
    chain_id: &str,
    old_key: &str,
    new_key: &str,
    signer_key: &str,
    ecpub: &str,
    force: bool,
) -> Result<ApiResponse<Compose>> {
    let mut req = ApiRequest::new("compose-identity-key-replacement");
    req.params.insert("chainid".to_string(), json!(chain_id));
    req.params.insert("oldkey".to_string(), json!(old_key));
    req.params.insert("newkey".to_string(), json!(new_key));
    req.params
        .insert("signerkey".to_string(), json!(signer_key));
    req.params.insert("ecpub".to_string(), json!(ecpub));
    req.params.insert("force".to_string(), json!(force));
    let response = walletd_call(api, req).await;
    parse(response).await
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compose {
    pub commit: Commit,
    pub reveal: Reveal,
}

/// Struct for deserialising the results of: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub jsonrpc: String,
    pub id: i64,
    pub params: CommitParams,
    pub method: String,
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitParams {
    pub message: String,
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reveal {
    pub jsonrpc: String,
    pub id: i64,
    pub params: RevealParams,
    pub method: String,
}

/// Struct for deserialising the results of the functions: compose-chain
/// compose-entry
/// compose-identity-attribute
/// compose-identity-attribute-endorsement
/// compose-identity-chain
/// compose-identity-key-replacementt
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevealParams {
    pub entry: String,
}

/// compose-transaction function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComposeTx {
    pub jsonrpc: String,
    pub id: i64,
    pub params: TxParams,
    pub method: String,
}

/// compose-transaction function
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxParams {
    pub transaction: String,
}
