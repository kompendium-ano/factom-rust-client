# Factom Rust API Client

[![Crates.io](https://img.shields.io/crates/v/factom.svg)](https://crates.io/crates/factom)
[![Released API docs](https://docs.rs/factom/badge.svg)](https://docs.rs/factom)
[![Build Status](https://travis-ci.com/kompendium-llc/factom-rust-client.svg?branch=master)](https://travis-ci.com/kompendium-llc/factom-rust-client)
[![dependency status](https://deps.rs/crate/factom/2.0.2/status.svg)](https://deps.rs/crate/factom/2.0.2)
[![Discord](https://img.shields.io/discord/419201548372017163.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/mYmcQM2)
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

##### Using default settings
```rust
let client = Factom::new();
```

##### Using the open node
```rust
// factom-walletd is at `http://localhost:8089`
let client = Factom::open_node();
```

##### Using the testnet open node
```rust
// factom-walletd is located at: 'http://localhost:8089`
let client = Factom::testnet_node();
```

##### Custom factomd and factom-walletd locations
```rust
let factomd = "https://api.factomd.net/";
let factom_walletd = "http://192.168.1.42:18089";
let client = Factom::custom_node(factomd, factom_walletd)
```

##### Retrieve a Balance
```rust
let client = factom.testnet_node();
let testnet_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let response = balance::factoid_balance(&client, testnet_address).await.unwrap();
// factoid balance returns factoshis, convert
let factoids = utils::factoshis_to_fct(response.result.balance);
println!("The testnet balance of {} is {} factoids", FCT_PUB, factoids);
```

##### Get Entry Data
```rust
let client = factom.testnet_node();
let hash = "97c4e7adce9ed277b62adfb9fb7a31ca4778181e49dcdfebca967102dd424fbc";
let response = entry::entry(hash).await.unwrap();
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

## Contributing
PR's welcome. Fork the library and submit to dev branch. 
By contributing to this library you agree to it being Apache 2.0 licensed 