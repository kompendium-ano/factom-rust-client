# Factom API Client

[![Crates.io](https://img.shields.io/crates/v/factom.svg)](https://crates.io/crates/factom)
[![Build Status](https://travis-ci.com/MitchellBerry/factom-client.svg?branch=master)](https://travis-ci.com/MitchellBerry/factom-client)
[![dependency status](https://deps.rs/crate/factom/1.0.1/status.svg)](https://deps.rs/crate/factom/1.0.1)

Asynchronous rust client for the Factom API.

## Installation
----
Add to cargo.toml:
```toml
[dependencies]
factom = 1.0.1
```

## Quickstart
----
```rust
use factom::{Factom, fetch};

let api = Factom::new();
let query = api.properties();
let response = fetch(query).expect("Error occurred fetching query");
dbg!(response);
/*
Response {
    jsonrpc: "2.0",
    id: 0,
    result: result(
        Object(
            {
                "factomdapiversion": String(
                    "2.0"
                ),
                "factomdversion": String(
                    "6.1.0"
                )
            }
        )
    )
}
*/
```

## Usage

```rust
use factom::*;
```

##### Synchronous call from remote host
```rust
let api = Factom::from_host("192.168.27.42");
let query = api.dblock_by_height(8);
// Fetch is a synchronous call that retreives a result from the query future.
let response = fetch(query).unwrap();
```

##### Asynchronous requests using futures
```rust

let api = Factom::new()
let heights = api.heights()
                    .map(|response| {
                        let leaderheight = Ok(response)["leaderheight"].as_u64();
                        let dblock = Ok(response)["dblock"].as_u64();
                        if leaderheight == dblock {
                            dbg!(response.result);
                        }
                        else {
                            println!("Factomd still syncing");
                        })
                    .map_err(|err| dbg!(err));

let properties = api.properties()
                        .map(|response| dbg!(response))
                        .map_err(|err| err);

rt:run(lazy(|| {
                rt::spawn(heights);
                rt::spawn(properties);
        });

```