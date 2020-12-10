# Factom Rust API Client

[![Crates.io](https://img.shields.io/crates/v/factom.svg)](https://crates.io/crates/factom)
[![Build Status](https://travis-ci.com/kompendium-llc/factom-rust-client.svg?branch=master)](https://travis-ci.com/kompendium-llc/factom-rust-client)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Installation

Add to cargo.toml:
```toml
[dependencies]
factom = "^2"
```

## Quickstart
```rust
use factom::*;
// Get current block height from open node
#[tokio::main]
async fn main() {
  //Create a re-usable client
  let client = Factom::open_node();
  // Call the heights function, handle the request result
  let response = factomd::heights(&client).await.expect("Network Request");
  assert!(response.result.leaderheight > 0);
}
```
## Usage
See the examples folder for common workflows, or use `cargo run --example` to
view all those you can run.

##### Configuration
```rust
// Default settings
// factomd: http://localhost:8088
// factom-walletd: http://localhost:8089
let client = Factom::new();

// Using open node
// factomd: https://api.factomd.net/v2
let client = Factom::open_node();

// Using the tesnet open node
// factomd: https://dev.factomd.net/v2
let client = Factom::testnet_node();

// Using custom node locations
let factomd = "https://api.factomd.net/";
let factom_walletd = "http://192.168.1.42:18089";
let client = Factom::custom_node(factomd, factom_walletd)
```

##### Retrieve a Balance
```rust
let client = factom::testnet_node();
let testnet_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let response = balance::factoid_balance(&client, testnet_address).await.unwrap();
// factoid balance returns factoshis, convert
let factoids = utils::factoshis_to_fct(response.result.balance);
println!("The testnet balance of {} is {} factoids", FCT_PUB, factoids);
```

##### Get Entry Data
```rust
let client = factom::testnet_node();
let hash = "97c4e7adce9ed277b62adfb9fb7a31ca4778181e49dcdfebca967102dd424fbc";
let response = entry::entry(hash).await.unwrap();
dbg!(response);
```

##### Traverse a Chain
```rust
 let client = Factom::open_node();
 let chain = "843dbee7a49a9b9510d399759fbce24b1f700268c94508085abce352d70ed1f6";
 // traverse_chain is a utility that returns Vec<Entry>, the number of blocks to
 // parse can be specified, to traverse the entire chain use a depth of 0, here
 // only 1 block,the chainhead itself, will be retrieved.
 let response = utils::traverse_chain(&client, chain, 1).await;
 dbg!(response);
 ```

## Runtime
This library re-exports the tokio runtime and executor by default, to disable this
and use a different runtime modify your `cargo.toml` with a feature flag:
```toml
[dependencies]
factom = {version="^2", default-features=false}
```

## Testing
Most of the functions are covered by the test modules along with all the documentation examples.
Beware that running `cargo test` with nocapture will produce a huge amount of output.
For many of the tests to pass you will need to be running factom-walletd, any
test transactions or addresses are cleaned up afterward.

See the [tests readme](https://github.com/kompendium-llc/factom-rust-client/tree/master/tests) for instructions.

## Benchmarking
A criterion benching harness is provided, if gnuplot is installed it will also automatically
generate plots for any benchmarks run, they can be found at `<LIBRARY>/target/criterion/report/index.html`. This
can be used to test the performance of factomd, factom-walletd, network connections
and the library functions themselves.

## Fuzzing
A fuzzing suite is provided using the rust implementation of
[American Fuzzy Lop](http://lcamtuf.coredump.cx/afl/), with appropriate setup
this can be used to fuzz both the Factom rust library along with a simulated
Factom network and factom-walletd. See the
[fuzz folder readme](https://github.com/kompendium-llc/factom-rust-client/tree/master/fuzz)
for more information.

## Learn
- [Accessing the Factom blockchain from different programming languages](https://medium.com/kompendium-developments/accessing-factom-blockchain-from-different-programming-languages-7f09030efe16)
- [Building simple blockchain game with Factom](https://medium.com/kompendium-developments/accessing-factom-blockchain-from-different-programming-languages-7f09030efe16)

## Contributions

The Library developed by Kompendium, LLC in partnership with [Mitchell Berry](https://github.com/MitchellBerry), [Sergey Bushnyak](https://sigrlami.eu), [Kelecorix, Inc](https://kelecorix.com) and for the good of the Factom community.

If you're an active user or find it useful we strongly encourage you to support our efforts and ensure long maintenance by contributing a small donation to one of the following cryptocurrency addresses:

- BTC: 39oVXpsgsyW8ZgzsnX3sV7HLdtXWfT96qN
- ETH: 0x9cDBA6bb44772259B3A3fb89cf233A147a720f34
- FCT: FA38cwer93mmPw1HxjScLmK1yF9iJTu5P87T2vdkbuLovm2YXyss
