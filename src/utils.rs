//! General purpose helper functions
use super::*;
use std::{time, thread};
use crate::entry::{Entry, RevealEntry}; 
use crate::chain::RevealChain;

/// Creates a chain going through the entire compose, commit, reveal workflow
/// 
/// # Example
/// 
/// ```no_run
///  use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
/// let client = Factom::new();
/// let name = vec!["Factom","Test","Identity"];
/// let pubkeys = vec!["idpub2k8zGYQUfekxehyUKeqPw6QPiJ5hkV3bbc9JBgL7GNrEiqMpQX",
///                 "idpub3fXRj21gXveTk6RKYrpJniWV2pAanQktekEt62yhJUQXyPdvwL",
///                 "idpub2GU1Pcax2PibH8hHZg58fKRiSJKQWQkWYkpmt7VH1jCXBgqp9w"];
/// let ec_pub = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
/// let response = utils::create_id_chain(&client, name, pubkeys, ec_pub).await;
/// dbg!(response);
/// }
/// ```
pub async fn create_id_chain(
  client: &Factom,
  name: Vec<&str>,
  pubkeys: Vec<&str>,
  ec_pub: &str,
) -> ApiResponse<RevealChain>
{
  let compose_query = compose::compose_id_chain(
                                &client, 
                                name, 
                                pubkeys, 
                                ec_pub,
                                false
                              );
  let compose_response = compose_query.await.expect("Composing Identity Chain");
  let commit = compose_response.result.commit.params.message;
  let commit_query = chain::commit_chain(&client, &commit);
  commit_query.await.expect("Committing Identity Chain");
  // Short pause for reveal
  thread::sleep(time::Duration::from_secs(1));
  let reveal = compose_response.result.reveal.params.entry;
  let reveal_query = chain::reveal_chain(&client, &reveal);
  let reveal_response = reveal_query.await.expect("Revealing Identity Chain");
  reveal_response
}

/// Creates an entry going through the entire compose, commit, reveal workflow
/// 
/// # Example
/// ```no_run
///  use factom::*;
///
/// #[tokio::main]
/// async fn main() {
/// let client = Factom::new();
/// let chainid = "72a2fa10b81a8bffde58ea206254f0eaa7928e9e09a4144efb3ba0bb7be26d52";
/// let ext_ids = vec!["Api Client", "Test Chain"];
/// let content = "Testing";
/// let ec_pub = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
/// let response = utils::create_entry(&client,chainid, ext_ids, content, ec_pub).await;
/// dbg!(response);
/// }
/// ```
pub async fn create_entry(
  client: &Factom,
  chainid: &str,
  ext_ids: Vec<&str>,
  content: &str,
  ec_pub: &str
) -> ApiResponse<RevealEntry> {
  let compose_query = compose::compose_entry(
                                &client, 
                                chainid, 
                                ext_ids, 
                                &content, 
                                ec_pub
                              );
  let compose_response = compose_query.await.expect("Compose Entry");
  let commit = compose_response.result.commit.params.message;
  let commit_query = entry::commit_entry(&client, &commit);
  commit_query.await.expect("Commit Entry");
  thread::sleep(time::Duration::from_millis(300));
  let reveal = compose_response.result.reveal.params.entry;
  let reveal_query = entry::reveal_entry(&client, &reveal);
  let reveal_response = reveal_query.await.expect("Reveal Entry");
  reveal_response
}

/// Creates a chain going through the entire compose, commit, reveal workflow
/// 
/// # Example
/// ```no_run
///  use factom::*;
/// #[tokio::main]
/// async fn main() {
/// let client = Factom::new();
/// let ext_ids = vec!["Api Client", "Test Chain"];
/// let content = "Testing";
/// let ec_pub = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
/// let response = utils::create_chain(&client, ext_ids, content, ec_pub).await;
/// dbg!(response);
/// }
/// ```
pub async fn create_chain(
  client: &Factom,
  ext_ids: Vec<&str>,
  content: &str,
  ec_pub: &str
) -> ApiResponse<RevealChain> {
  let compose_query = compose::compose_chain(&client, ext_ids, content, ec_pub);
  let compose_response = compose_query.await.expect("Composing Chain");
  
  let commit = compose_response.result.commit.params.message;
  let commit_query = chain::commit_chain(&client, &commit);
  commit_query.await.expect("Committing Chain");
  // Short pause for reveal
  thread::sleep(time::Duration::from_secs(1));
  let reveal = compose_response.result.reveal.params.entry;
  let reveal_query = chain::reveal_chain(&client, &reveal);
  let reveal_response = reveal_query.await.expect("Revealing Chain");
  reveal_response
}

/// Traverses a chain from the head to root returning all entries
/// Can specify a depth of blocks to go back from the chainhead
/// A depth of 0 will traverse the whole chain.
/// 
/// # Example
/// ```no_run
/// use factom::*;
/// 
/// #[tokio::main]
/// async fn main() {
/// let client = Factom::open_node();
/// let chain = "843dbee7a49a9b9510d399759fbce24b1f700268c94508085abce352d70ed1f6";
/// let response = utils::traverse_chain(&client, chain, 1).await;
/// dbg!(response);
/// }
/// ```
pub async fn traverse_chain(
  client: &Factom, 
  chainid: &str,
  depth: usize
) -> Vec<Entry> 
{
  let chainhead_response = chain::chain_head(client, chainid)
                            .await
                            .expect("Fetching ChainHead");
  let mut keymr = chainhead_response.result.chainhead;
  let mut entries = Vec::new();
  let mut blocks = 0;
  while keymr != NULL_KEYMR {
    let response = block::entry_block(&client, &keymr)
                            .await
                            .expect("Fetching EntryBlock");
    keymr = response.result.header.prevkeymr;
    let entrylist = response.result.entrylist;
    for e in entrylist {
      let response = entry::entry(&client, &e.entryhash)
                          .await
                          .expect("Fetching Entry");
      entries.push(response.result);
    }
    blocks += 1;
    if blocks == depth {
      break;
    }
  }
  entries
}

/// Converts Factoshis to Factoids
pub fn factoshis_to_fct(factoshis: usize) -> f64 {
  factoshis as f64 / 100_000_000f64
} 

/// Converts Factoids to Factoshis
pub fn fct_to_factoshis(factoids: f64) -> f64 {
  factoids * 100_000_000f64
}
