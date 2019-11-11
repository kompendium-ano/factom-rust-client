/// Default factom-walletd location
pub const WALLETD_DEFAULT: &str = "http://localhost:8089/v2";
/// Default factomd location
pub const FACTOMD_DEFAULT: &str = "http://localhost:8088/v2";
/// Public factomd mainnet open node. For more information visit https://factomd.net
pub const OPENNODE_URI: &str = "https://api.factomd.net/v2";
/// Public factomd testnet open node
pub const DEV_OPENNODE_URI: &str = "https://dev.factomd.net/v2";
/// Factomd debug functions
pub const DEBUG_DEFAULT: &str = "http://localhost:8088/debug";
/// Factom api versioning
pub const API_VERSION: u8 = 2;
/// JSON-RPC versioning
pub const JSONRPC : &str = "2.0";
/// Default JSON-RPC ID
pub const ID: usize = 0;

/// Factoid RCD address prefix
pub const RCD_PREFIX: [u8; 1] = [0x1];
/// Entry Credit public key prefix
pub const EC_PUBLIC_KEY_PREFIX: [u8; 2] = [89, 42];
/// Entry Credit secret key prefix
pub const EC_SECRET_KEY_PREFIX: [u8; 2] = [93, 182];
/// Factoid public key prefix
pub const FCT_PUBLIC_KEY_PREFIX: [u8; 2] = [95, 177];
/// Factoid private key prefix
pub const FCT_SECRET_KEY_PREFIX: [u8; 2] = [100, 120];
/// Valid address prefixes
pub const VALID_PREFIXES: [[u8; 2]; 4] = [
    EC_PUBLIC_KEY_PREFIX, 
    EC_SECRET_KEY_PREFIX, 
    FCT_PUBLIC_KEY_PREFIX, 
    FCT_SECRET_KEY_PREFIX
];
/// Valid pub prefixes
pub const VALID_PUB_PREFIXES: [&str; 2] = ["FA", "EC"];
/// Valid sec prefixes
pub const VALID_SEC_PREFIXES: [&str; 2] = ["Fs", "Es"];
/// Valid FCT prefixes
pub const VALID_FCT_PREFIXES: [&str; 2] = ["FA", "Fs"];
/// Valid EC prefixes
pub const VALID_EC_PREFIXES: [&str; 2] = ["EC", "Es"];