## Client modules
The Factom rust client API is separated into logical modules. Links to their respective 
functions are below. Full documentation is hosted at https://docs.rs/factom/

----

### [Address](https://docs.rs/factom/2.0.2/factom/address/index.html)

Relating to Address functions

* address
* all-addresses
* remove-address

----

### [Api](https://docs.rs/factom/2.0.2/factom/api/index.html)

The main api client module holds Factom struct from which requests are constructed.

----
### [Balance](https://docs.rs/factom/2.0.2/factom/balance/index.html)

For balance related functions.

* entry-credit-balance
* factoid-balance
* multiple-ec-balances
* multiple-fct-balances

----

### [Block](https://docs.rs/factom/2.0.2/factom/block/index.html)

For functions dealing with block data queries.

* ablock_by_height
* admin_block
* anchors
* dblock_by_height
* directory_block
* directory_block_head
* ecblock_by_height
* entry_block
* entry_credit_block
* factoid_block
* fblock_by_height

----

### [Chain](https://docs.rs/factom/2.0.2/factom/chain/index.html)

For functions handling chain data.

* chain_head
* commit_chain
* reveal_chain

----

### [Compose](https://docs.rs/factom/2.0.2/factom/compose/index.html)

Functions that compose transactions, entries and identities.

* compose_chain
* compose_entry
* compose_transaction
* compose_id_attribute<T>
* compose_id_attribute_endorsement
* compose_id_chain
* compose_id_key_replacement

----

### [Constants](https://docs.rs/factom/2.0.2/factom/constants/index.html)

Static constants for use within the library.

* WALLETD_DEFAULT
* FACTOMD_DEFAULT
* OPENNODE_URI
* DEV_OPENNODE_URI
* DEBUG
* API_VERSION
* JSONRPC 
* ID

----

### [Debug](https://docs.rs/factom/2.0.2/factom/debug/index.html)

Factomd debug functions. Tests exist but are disabled for this module and require running a local factomd node.

* holding_queue
* network_info
* predictive_fer
* audit_servers
* federated_servers
* configuration
* process_list
* authorities
* reload_configuration
* drop_rate
* set_drop_rate
* delay
* set_delay
* summary
* messages

----

### [Entry](https://docs.rs/factom/2.0.2/factom/entry/index.html)
For querying entires.

* commit_entry
* entry
* raw_data
* pending_entries
* reveal_entry

----

### [Factomd](https://docs.rs/factom/2.0.2/factom/factomd/index.html)

General functions relating to factomd

* current_minute
* diagnostics
* entry_credit_rate
* heights
* properties
* receipt
* send_raw_message

----

### [Generate](https://docs.rs/factom/2.0.2/factom/generate/index.html)

Functions for generating addresses or identities.

* generate_ec_address
* generate_factoid_address
* generate_identity_key

----

### [Identity](https://docs.rs/factom/2.0.2/factom/identity/index.html)

Relating to identity functions.

* all_id_keys
* active_id_keys
* remove_id_key
* id_key

----

### [Import](https://docs.rs/factom/2.0.2/factom/import/index.html)

For importing addresses or identities

* import_addresses
* import_identity_keys
* import_koinify

----

### [Requests](https://docs.rs/factom/2.0.2/factom/requests/index.html)

Request handling functions intrinsic to the factom struct

* factomd_call
* walletd_call
* debug_call

---

### [Responses](https://docs.rs/factom/2.0.2/factom/responses/index.html)

Response handling functions to parse json responses into objects

---

### [Tx](https://docs.rs/factom/2.0.2/factom/tx/index.html)

Functions relating to transactions

* ack
* factoid_submit
* transaction
* pending_transactions
* add_ec_output
* add_fee
* add_input
* add_output
* delete_transaction
* new_transaction
* sign_transaction
* sub_fee
* tmp_transactions
* transactions

### [Walletd](https://docs.rs/factom/2.0.2/factom/walletd/index.html)

General utility functions relating to factom-walletd

* wallet_backup
* wallet_balances
* unlock_wallet
* wallet_height
* wallet_properties


### [Utils](https://docs.rs/factom/2.0.2/factom/utils/index.html)