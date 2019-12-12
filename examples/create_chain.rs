use factom::{
  Factom,
  compose::compose_chain, 
  chain::{commit_chain, reveal_chain} 
};

const EC_PUB: &str = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";

// This example is for demonstration purposes. There is a helper function
// utils::create_chain which vastly simplifies this process
#[tokio::main]
async fn main() {
  println!("Initial Local Node Setup:\nfactomd -network=LOCAL -blktime=20\n\
  factom-cli importaddress Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK\n\
  factom-cli importaddress Es3LS7zYa9DSzZuUC14HDpMinehmzz61JG1XFY62rX5pVDenH8Pk\n\
  factom-cli buyec FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK 10000");
  let client = Factom::new();
  let ext_ids = vec!("Api Client", "Test Chain");
  let content = "Testing";
  
  let compose_query = compose_chain(&client, ext_ids, content, EC_PUB);
  let compose_response = compose_query.await.unwrap();
  dbg!(&compose_response);
  
  let commit = compose_response.result.commit.params.message;
  let commit_query = commit_chain(&client, &commit);
  let commit_response = commit_query.await.unwrap();
  dbg!(&commit_response);

  let reveal = compose_response.result.reveal.params.entry;
  let reveal_query = reveal_chain(&client, &reveal);
  let reveal_response = reveal_query.await.unwrap();
  dbg!(&reveal_response);
}
