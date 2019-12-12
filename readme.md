# Factom Rust API Client

[![Crates.io](https://img.shields.io/crates/v/factom.svg)](https://crates.io/crates/factom)
[![Released API docs](https://docs.rs/factom/badge.svg)](https://docs.rs/factom)
[![Build Status](https://travis-ci.com/Kompendium-llc/Factom-Client.svg?branch=master)](https://travis-ci.com/Kompendium-llc/Factom-Client)
[![dependency status](https://deps.rs/crate/factom/2.0.0/status.svg)](https://deps.rs/crate/factom/2.0.0)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Installation

Add to cargo.toml:
```toml
[dependencies]
factom = "2.0.0"
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
view all those you can run. Eg. `cargo run --example get_balance`

## Runtime
This library re-exports the tokio runtime and executor by default, to disable this
and use a different runtime modify your `cargo.toml` with a feature flag:
```toml
[dependencies]
factom = {version="2.0.0", features="no-runtime"}
```

## Testing
Most of the functions are covered by the test modules along with all the documentation examples. 
Beware that running `cargo test` with nocapture will produce a huge amount of output. 
For many of the tests to pass you will need to be running factom-walletd, any 
test transactions or addresses are cleaned up afterward. 

See the testing readme for instructions.

## Benchmarking
A criterion benching harness is provided, if gnuplot is installed it will also automatically 
generate plots for any benchmarks run, they can be found at `<LIBRARY>/target/criterion/report/index.html`. This 
can be used to test the performance of factomd, factom-walletd, network connections 
and the library functions themselves. See the benches folder readme for more information.

## Fuzzing
A fuzzing suite is provided using the rust implementation of 
[American Fuzzy Lop](http://lcamtuf.coredump.cx/afl/), with appropriate setup 
this can be used to fuzz both the Factom rust library along with a simulated 
Factom network and factom-walletd. See the fuzz folder readme for more information.

## Contributing
PR's welcome. Fork the library and submit to dev branch. 
By contributing to this library you agree to it being Apache 2.0 licensed 

