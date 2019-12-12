//!Static constants for use with the library.
/// Default factom-walletd location
pub const WALLETD_DEFAULT: &str = "http://localhost:8089";
/// Default factomd location
pub const FACTOMD_DEFAULT: &str = "http://localhost:8088";
/// Public factomd mainnet open node. For more information visit https://factomd.net
pub const OPENNODE_URI: &str = "https://api.factomd.net";
/// Public factomd testnet open node
pub const DEV_OPENNODE_URI: &str = "https://dev.factomd.net";
/// Factomd debug functions
pub const DEBUG: &str = "debug";
/// Factom api versioning
pub const API_VERSION: &str = "v2";
/// JSON-RPC versioning
pub const JSONRPC: &str = "2.0";
/// Default JSON-RPC ID
pub const ID: usize = 0;
/// Regex for matching Factoid addresses
pub const FCT_REGEX: &str = "^FA[123][1-9A-HJ-NP-Za-km-z]{49}";
/// Regex for matching Entry Credit Addresses
pub const EC_REGEX: &str = "^EC[123][1-9A-HJ-NP-Za-km-z]{49}";
/// Null key merkle root
pub const NULL_KEYMR: &str = "0000000000000000000000000000000000000000000000000000000000000000";
