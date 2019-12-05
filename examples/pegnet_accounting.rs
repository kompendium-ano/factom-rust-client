use factom::{Factom, chain::chain_head, block::entry_block};

// Transactions chain
const PNET_TX_CHAIN: &str = "cffce0f409ebba4ed236d49d89c70e4bd1f1367d86402a3363366683265a242d";

// const PNET_TX_KEYMR: &str = "9ef4cce0a5d480db47513c1ff28bfb812f3e46361dfd0686c136047d992df97c";
// OPR Chain
// const PNET_CHAIN: &str = "a642a8674f46696cc47fdb6b65f9c87b2a19c5ea8123b3d2f0c13b6f33a9d5ef";

// This example traverses the pegnet conversion chain searching for 
// transactions with the provided address.
#[tokio::main]
async fn main() {
  let api = Factom::open_node();
  let chain_head_query = chain_head(&api, PNET_TX_CHAIN);
  let response = chain_head_query.await.expect("Fetching query");
  dbg!(&response);

  let query = entry_block(&api, &response.result.chainhead);
  let eblock_response = query.await.expect("Fetching query");
  dbg!(eblock_response);
}
