use factom::*;

const FCT_PUB: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";

fn main() {
  let api = Factom::testnet_node();
  let query = api.factoid_balance(FCT_PUB);
  let response = fetch(query).expect("Fetching query");
  if response.is_err() {
    println!("Error: {}\nCode: {}", response.error.message, response.error.code);
  }
  else {
    println!("The balance of {} is {} factoids", FCT_PUB, &response.result.balance);
  }
}