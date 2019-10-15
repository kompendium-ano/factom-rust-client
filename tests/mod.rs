use factom::*; 


fn test_node() -> Factom {
  Factom::testnet_open_node()
}

#[test]
fn properties(){
  let api = test_node();
  let query = api.properties();
  let response = fetch(query).expect("Unable to fetch request");
  dbg!(response);
  let a = 1;

}