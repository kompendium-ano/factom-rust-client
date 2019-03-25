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
use factom::*;

let api = Factom::new();
let response = fetch(api.properties()).expect("Unable to fetch query");
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
----
## Usage

```rust
use factom::*;
```

##### Simple synchronous call from remote host
```rust
let api = Factom::from_host("192.168.27.42");
let height_query = api.heights();
let response = fetch(height_query).unwrap();
dbg!(response)
```

##### Synchronous call using Tokio block_on
```rust
// Create Tokio runtime
let mut runtime = Runtime::new().expect("Unable to create Tokio Runtime"); 
let api = Factom::new();

let entryhash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let entry_query = api.entry(entryhash);

// block_on waits for future to return result
let response = runtime.block_on(entry_query).unwrap();
dbg!(response);

// Shutdown runtime once idle
shutdown(runtime);
```

##### Asynchronous requests using futures
```rust
let mut runtime = Runtime::new().unwrap(); 
let api = Factom::new();

// Closure to parse heights response
let height_handler = |response: Response| {
    // Parse Result
    let result = response.get_result().expect("Error fetching request");
    // Extract heights
    let leader = &result["leaderheight"];
    let dblockheight = &result["directoryblockheight"];
    // Compare
    if leader == dblockheight{
        println!("Factomd fully synced at height: {}", leader);
    }
    else {
        println!("Not synced");
    }
};

// Main heights request
let heights_query = api.heights()
                        // Handle successful execution of future
                        .map(height_handler)
                        // Print FetchError info if it occurs
                        .map_err(|err| {dbg!(err);});

// Closure to print entry content
let entry_handler = |res: Response| {
    let result = res.get_result().unwrap();
    dbg!(&result["content"]);
};

// Main entry request
let entryhash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";
let entry_query = api.entry(entryhash)
                        .map(entry_handler)
                        .map_err(|err| {dbg!(err);});

// Spawn queries into current runtime
runtime.spawn(heights_query);
runtime.spawn(entry_query);

shutdown(runtime);
```

##### Passing messages between async tasks
```rust
use futures::sync::oneshot;

let mut runtime = Runtime::new().unwrap();
let api = Factom::from_host("192.168.121.132");

// Oneshot used for passing single values, use mpsc to pass streams.
let (tx, rx) = oneshot::channel::<Response>();

// Heights query transmits the response data
let heights = api.heights()
                    .map(|res| {
                        tx.send(res);
                    })
                    .map_err(|err| ());

// Spawn into current runtime
runtime.spawn(heights);

// Reciever prints out response
runtime.spawn(rx.and_then(|res| {
    println!("response recieved: {:?}", res);
    Ok(())
    })
    .map_err(|e| {println!("error: {}", e);})
);

shutdown(runtime);

```
----

## Testing 

Setup test environment first, if factomd/walletd are not run locally modify the HOST variable in tests/mod.rs, see the [readme](/tests/readme.md) for more information.
```bash
cargo test
```
----
## License
```
   Copyright 2018 Mitchell Berry

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```