use factom::{
    compose::compose_entry,
    entry::{commit_entry, reveal_entry},
    Factom,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::iter;

const EC_PUB: &str = "EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK";
const CHAINID: &str = "72a2fa10b81a8bffde58ea206254f0eaa7928e9e09a4144efb3ba0bb7be26d52";

// This example is for demonstration purposes. There is a helper function
// utils::create_entry which vastly simplifies this process
#[tokio::main]
async fn main() {
    println!(
        "This example assumes you have gone through the create_chain example\
  first"
    );
    let client = Factom::new();
    let rand_ext_id = &random_string(12);
    let ext_ids = vec!["Api Client", "Test Chain", rand_ext_id];
    let content = random_string(32);

    let compose_query = compose_entry(&client, CHAINID, ext_ids, &content, EC_PUB);
    let compose_response = compose_query.await.expect("Fetching Query");
    dbg!(&compose_response);

    let commit = compose_response.result.commit.params.message;
    let commit_query = commit_entry(&client, &commit);
    let commit_response = commit_query.await.expect("Fetching Query");
    dbg!(&commit_response);

    let reveal = compose_response.result.reveal.params.entry;
    let reveal_query = reveal_entry(&client, &reveal);
    let reveal_response = reveal_query.await.expect("Fetching Query");
    dbg!(&reveal_response);
}

fn random_string(len: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect()
}
