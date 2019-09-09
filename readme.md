# Factom API Client

[![Crates.io](https://img.shields.io/crates/v/factom.svg)](https://crates.io/crates/factom)
[![Build Status](https://travis-ci.com/MitchellBerry/Factom-Client.svg?branch=master)](https://travis-ci.com/MitchellBerry/Factom-Client)
[![dependency status](https://deps.rs/crate/factom/1.0.2/status.svg)](https://deps.rs/crate/factom/1.0.1)

Asynchronous rust client for the Factom API. Full documentation is available [here](https://docs.rs/factom/1.0.1/factom/). Serialisation is handled by serde and the futures runtime is handled by tokio.

## Installation

Add to cargo.toml:
```toml
[dependencies]
factom = "1.0.2"
```

## Quickstart

```rust
use factom::*;

// Create new api handler for factomd/walletd on the localhost on ports 8088 and 8089 respectively
let api = Factom::new();

// Methods return a future to be spawned onto a runtime or synchronously fetched
let request = api.properties();

// Fetch is a blocking helper method to get the result of a future
// Http or Json errors raise here. 
// API errors such as invalid method or parameters will passed along in the response to be handled later
let response = fetch(request).expect("Unable to fetch request");

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
// For https calls use Factom::from_https_host()
let api = Factom::from_host("192.168.27.42");
let request = api.heights();
let response = fetch(request).unwrap();
dbg!(response);
```

#### Different hosts or ports for factomd and walletd
```rust
// Factomd open node and walletd locally on port 3003. Id is included in the json-rpc call.
let api = Factom{
  uri: "https://api.factomd.net/v2",
  wallet_uri: "http://localhost:3003/v2",
  id: 0
};
let request = api.wallet_balances();
let response = fetch(request).unwrap();
dbg!(response);

```

##### Synchronous call using Tokio block_on and custom json-rpc id
```rust
// Create Tokio runtime
let mut runtime = Runtime::new().expect("Unable to create Tokio Runtime"); 
let api = Factom::new();

let entryhash = "6ecd7c6c40d0e9dbb52457343e083d4306c5b4cd2d6e623ba67cf9d18b39faa7";

// Add custom id
let entry_query = api.entry(entryhash)
                        .set_id(42);

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


## Testing 

A custom database (~700kb) is located in /tests/env/ and needs to moved into the ~/.factom folder. 
If factomd/walletd are not run locally, you will need to modify the HOST variable in tests/mod.rs, see the tests [readme](/tests/readme.md) for more information.
