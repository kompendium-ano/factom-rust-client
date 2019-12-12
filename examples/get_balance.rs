use factom::{Factom, balance::factoid_balance, utils::factoshis_to_fct};

const FCT_PUB: &str = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";

#[tokio::main]
async fn main() {
  let client = Factom::testnet_node();
  let query = factoid_balance(&client, FCT_PUB);
  let response = query.await.unwrap();
  if response.is_err() {
    println!("Error: {}\nCode: {}", response.error.message, response.error.code);
  }
  else {
    // factoid balance returns factoshis
    let factoids = factoshis_to_fct(response.result.balance);
    println!("The testnet balance of {} is {} factoids", FCT_PUB, factoids);
  }
}
