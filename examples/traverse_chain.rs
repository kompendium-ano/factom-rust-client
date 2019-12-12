use factom::{Factom, chain::chain_head, block::entry_block};

// Transactions chain
const PNET_TX_CHAIN: &str = "cffce0f409ebba4ed236d49d89c70e4bd1f1367d86402a3363366683265a242d";

// This example is for demonstration purposes. There is a helper function
// utils::traverse_chain which vastly simplifies this process
#[tokio::main]
async fn main() {
  let api = Factom::open_node();
  let chainhead_response = chain_head(&api, PNET_TX_CHAIN)
                            .await
                            .expect("Fetching query");
  let mut keymr = chainhead_response.result.chainhead;
  let mut entries = Vec::new(); 
  // It's a large chain, lets only traverse the last 5 blocks
  for _ in 0..5u8 {
    let response = entry_block(&api, &keymr)
                            .await
                            .expect("Fetching query");
    keymr = response.result.header.prevkeymr;
    let entrylist = response.result.entrylist;
    for entry in entrylist {
      entries.push(entry.entryhash);
    }
  }
  dbg!(entries);
}
