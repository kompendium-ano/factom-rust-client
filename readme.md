# Factom Rust API Client

[![Crates.io](https://img.shields.io/crates/v/factom.svg)](https://crates.io/crates/factom)
[![Released API docs](https://docs.rs/factom/badge.svg)](https://docs.rs/factom)
[![Build Status](https://travis-ci.com/Kompendium-llc/Factom-Client.svg?branch=master)](https://travis-ci.com/Kompendium-llc/Factom-Client)
[![dependency status](https://deps.rs/crate/factom/2.0.0/status.svg)](https://deps.rs/crate/factom/2.0.0)

## Installation

Add to cargo.toml:
```toml
[dependencies]
factom = "2.0.0"
```

## Quickstart
```rust
// Get current block height
#[tokio::main]
async fn main() {
  let client = Factom::open_node();
  let response = factomd::heights(&client).await.unwrap();
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
See the [testing readme]() for instructions on running the test suite.

## Benchmarking
A criterion benching harness is provided, if gnuplot is installed it will also automatically 
generate plots for any benchmarks run, they can be found in `<LIBRARY>/target/criterion/report/index.html` folder. This 
can be used to test the performance of factomd, factom-walletd, network connections 
and the library functions themselves. See the benches folder readme for more information.

## Fuzzing
A fuzzing suite is provided using the rust implementation of 
[American Fuzzy Lop](http://lcamtuf.coredump.cx/afl/), with appropriate setup 
this can be used to fuzz both the Factom rust library along with a simulated 
Factom network and factom-walletd. See the Fuzz folder [readme] for more information.

## Client modules
The Factom rust client API is separated into logical modules. Links to their respective 
functions are below. Full documentation is hosted at https://docs.rs/factom/

----

### Address

Relating to Address functions

* [address](ggg-address)
* [all-addresses](ggg-all-addresses)
* [remove-address](ggg-remove-address)

----

### Api

The main api client module holds Factom struct from which requests are constructed.

----
### Balance

For balance related functions.

* [entry-credit-balance](ggg-entry-credit-balance)
* [factoid-balance](ggg-factoid-balance)
* [multiple-ec-balances](ggg-multiple-ec-balances)
* [multiple-fct-balances](ggg-multiple-fct-balances)

----

### Block

For functions dealing with block data queries.

* [ablock_by_height](ggg-ablock_by_height)
* [admin_block](ggg-admin_block)
* [anchors](ggg-anchors)
* [dblock_by_height](ggg-dblock_by_height)
* [directory_block](ggg-directory_block)
* [directory_block_head](ggg-directory_block_head)
* [ecblock_by_height](ggg-ecblock_by_height)
* [entry_block](ggg-entry_block)
* [entry_credit_block](ggg-entry_credit_block)
* [factoid_block](ggg-factoid_block)
* [fblock_by_height](ggg-fblock_by_height)

----

### Chain

For functions handling chain data.

* [chain_head](ggg-chain_head)
* [commit_chain](ggg-commit_chain)
* [reveal_chain](ggg-reveal_chain)

----

### Compose

Functions that compose transactions, entries and identities.

* [compose_chain](ggg-compose_chain)
* [compose_entry](ggg-compose_entry)
* [compose_transaction](ggg-compose_transaction)
* [compose_id_attribute<T>](ggg-compose_id_attribute<T>)
* [compose_id_attribute_endorsement](ggg-compose_id_attribute_endorsement)
* [compose_id_chain](ggg-compose_id_chain)
* [compose_id_key_replacement](ggg-compose_id_key_replacement)

----

### Constants

Static constants for use within the library.

* [WALLETD_DEFAULT](ggg-WALLETD_DEFAULT)
* [FACTOMD_DEFAULT](ggg-FACTOMD_DEFAULT)
* [OPENNODE_URI](ggg-OPENNODE_URI)
* [DEV_OPENNODE_URI](ggg-DEV_OPENNODE_URI)
* [DEBUG](ggg-DEBUG)
* [API_VERSION](ggg-API_VERSION)
* [JSONRPC ](ggg-JSONRPC )
* [ID](ggg-ID)

----

### Debug

Factomd debug functions. Tests exist but are disabled for this module and require running a local factomd node.

* [holding_queue](ggg-holding_queue)
* [network_info](ggg-network_info)
* [predictive_fer](ggg-predictive_fer)
* [audit_servers](ggg-audit_servers)
* [federated_servers](ggg-federated_servers)
* [configuration](ggg-configuration)
* [process_list](ggg-process_list)
* [authorities](ggg-authorities)
* [reload_configuration](ggg-reload_configuration)
* [drop_rate](ggg-drop_rate)
* [set_drop_rate](ggg-set_drop_rate)
* [delay](ggg-delay)
* [set_delay](ggg-set_delay)
* [summary](ggg-summary)
* [messages](ggg-messages)

----

### Entry
For querying entires.

* [commit_entry](ggg-commit_entry)
* [entry](ggg-entry)
* [raw_data](ggg-raw_data)
* [pending_entries](ggg-pending_entries)
* [reveal_entry](ggg-reveal_entry)

----

### Factomd

General functions relating to factomd

* [current_minute](ggg-current_minute)
* [diagnostics](ggg-diagnostics)
* [entry_credit_rate](ggg-entry_credit_rate)
* [heights](ggg-heights)
* [properties](ggg-properties)
* [receipt](ggg-receipt)
* [send_raw_message](ggg-send_raw_message)

----

### Generate

Functions for generating addresses or identities.

* [generate_ec_address](ggg-generate_ec_address)
* [generate_factoid_address](ggg-generate_factoid_address)
* [generate_identity_key](ggg-generate_identity_key)

----

### Identity

Relating to identity functions.

* [all_id_keys](ggg-all_id_keys)
* [active_id_keys](ggg-active_id_keys)
* [remove_id_key](ggg-remove_id_key)
* [id_key](ggg-id_key)

----

### Import

For importing addresses or identities

* [import_addresses](ggg-import_addresses)
* [import_identity_keys](ggg-import_identity_keys)
* [import_koinify](ggg-import_koinify)

----

### Requests

Request handling functions intrinsic to the factom struct

* [factomd_call](ggg-factomd_call)
* [walletd_call](ggg-walletd_call)
* [debug_call](ggg-debug_call)

---

### Responses

Response handling functions to parse json responses into objects

* [ack](ggg-ack)
* [factoid_submit](ggg-factoid_submit)
* [transaction](ggg-transaction)
* [pending_transactions](ggg-pending_transactions)
* [add_ec_output](ggg-add_ec_output)
* [add_fee](ggg-add_fee)
* [add_input](ggg-add_input)
* [add_output](ggg-add_output)
* [delete_transaction](ggg-delete_transaction)
* [new_transaction](ggg-new_transaction)
* [sign_transaction](ggg-sign_transaction)
* [sub_fee](ggg-sub_fee)
* [tmp_transactions](ggg-tmp_transactions)
* [transactions](ggg-transactions)


---

### Tx

Functions relating to transactions

* [ack](ggg-ack)
* [factoid_submit](ggg-factoid_submit)
* [transaction](ggg-transaction)
* [pending_transactions](ggg-pending_transactions)
* [add_ec_output](ggg-add_ec_output)
* [add_fee](ggg-add_fee)
* [add_input](ggg-add_input)
* [add_output](ggg-add_output)
* [delete_transaction](ggg-delete_transaction)
* [new_transaction](ggg-new_transaction)
* [sign_transaction](ggg-sign_transaction)
* [sub_fee](ggg-sub_fee)
* [tmp_transactions](ggg-tmp_transactions)
* [transactions](ggg-transactions)

### Walletd

General utility functions relating to factom-walletd

* [wallet_backup](ggg-wallet_backup)
* [wallet_balances](ggg-wallet_balances)
* [unlock_wallet](ggg-unlock_wallet)
* [wallet_height](ggg-wallet_height)
* [wallet_properties](ggg-wallet_properties)
<!-- * [sign_data](ggg-sign_data) -->
