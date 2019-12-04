use factom::*;

const FCT_PUB: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";

#[tokio::main]
async fn main() {
  let api = Factom::testnet_node();
  let query = api.factoid_balance(FCT_PUB);
  let response = query.await.expect("Fetching query");
  if response.is_err() {
    println!("Error: {}\nCode: {}", response.error.message, response.error.code);
  }
  else {
    // factoid balance returns factoshis
    let factoids = *&response.result.balance as f64 / 100_000_000f64;
    println!("The testnet balance of {} is {} factoids", FCT_PUB, factoids);
  }
}
