use factom::*;

// Transactions chain
const PNET_CHAIN: &str = "cffce0f409ebba4ed236d49d89c70e4bd1f1367d86402a3363366683265a242d";

// const PNET_TX_KEYMR: &str = "9993c521d578c4aedc125e011ce424aa317de85e080e9a49c76b8d51f8140209";

const PNET_TX_KEYMR: &str = "9ef4cce0a5d480db47513c1ff28bfb812f3e46361dfd0686c136047d992df97c";
// OPR Chain
//const PNET_CHAIN: &str = "a642a8674f46696cc47fdb6b65f9c87b2a19c5ea8123b3d2f0c13b6f33a9d5ef";

// This example traverses the pegnet conversion chain searching for 
// transactions with the provided address, it will then list all by
// date, time and volume of each conversion then export it into a csv format 
// suitable for consumption by accounting programs.
#[tokio::main]
async fn main() {
  let api = Factom::open_node();
  let chain_head_query = api.chain_head(PNET_CHAIN);
  // let query = api.entry_block(PNET_TX_KEYMR);
  let response = chain_head_query.await.expect("Fetching query");
  dbg!(&response);

  let query = api.entry_block(&response.result.chainhead);
  let eblock_response = query.await.expect("Fetching query");
  dbg!(eblock_response);
}