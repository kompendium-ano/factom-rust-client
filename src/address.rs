use super::*;
use hex;
use bs58;
use ed25519_dalek::{PublicKey, SecretKey};
use sha2::{Digest, Sha256, Sha512};

impl Factom {
/**
Retrieve the public and private parts of a Factoid or Entry Credit address 
stored in the wallet.

# Example
```
use factom::*;
let api = Factom::testnet_open_node();
let my_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let query = factom
            .address(my_address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
*/
  pub async fn address(
    self, 
    address: &str
  )-> Result<ApiResponse<Address>>
  {
    let mut req =  ApiRequest::new("address");
    req.params.insert("address".to_string(), json!(address));
    let response = self.walletd_call(req).await;
    parse(response).await
  }

  /**
Retrieve all of the Factoid and Entry Credit addresses stored in the wallet.

# Example
```
use factom::*;
let api = Factom::testnet_open_node();
let query = factom
            .all_addresses()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```
*/  
  pub async fn all_addresses(self)
    -> Result<ApiResponse<AllAddresses>> 
  {
    let req =  ApiRequest::new("all-addresses");
    let response = self.walletd_call(req).await;
    parse(response).await
  }

/**
Be careful using this function! Ensure that you have backups of important keys 
before removing them. Given a factoid or entry-credit address, this command 
deletes the corresponding key pair from the wallet. Once executed, the user will 
no longer be able to retrieve the private key or make transactions with the 
address from this wallet. If the wallet is encrypted, it must be unlocked prior 
to using this command.

# Example
```
use factom::*;
let api = Factom::testnet_open_node();
let my_address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let query = factom
            .remove_address(my_address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());
```

*/
  pub async fn remove_address(
    self, 
    address: &str
  )-> Result<ApiResponse<RemoveAddress>>
  {
    let mut req =  ApiRequest::new("remove-address");
    req.params.insert("address".to_string(), json!(address));
    let response = self.walletd_call(req).await;
    parse(response).await
  }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Address {
  pub public: String,
  pub secret: String
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AllAddresses {
  pub addresses: Vec<Address>
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RemoveAddress {
  pub success: String
}

fn key_to_rcd_hash(key: &str) -> AddressResult<String> {
  let key = decode_key(key)?;
  let input = [RCD_PREFIX.to_vec(), key.to_vec()].concat();
  let h = sha256d(&input);
  Ok(hex::encode(h))
}

fn decode_key(key: &str) -> AddressResult<[u8; 32]> {
  let key = hex::decode(key)?;
  if key.len() != 32 {
    let err = format!("Expected key of 32 bytes, received: {} bytes", key.len());
    return Err(AddressError::Length(err));
  };

  let mut output = [0u8; 32];
  output.copy_from_slice(&key);
  Ok(output)
}

fn sha256d(input: &[u8]) -> [u8; 32] {
  let h1 = Sha256::digest(input);
  let h2 = Sha256::digest(&h1);
  
  let mut output = [0u8; 32];
  output.copy_from_slice(&h2);
  output
}

fn key_to_address(prefix: [u8; 2], key: [u8; 32]) -> String {
  let address = &[prefix.to_vec(), key.to_vec()].concat();
  let checksum = &sha256d(address)[0..4];
  bs58::encode([address, checksum].concat()).into_string()
}

/// Convert a 32 byte hex key to a public entry credit address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let key = "f8dd9d8af5d7cabd2b370be9f91bdd9021acb2eb9feaa39a78375e31f978377a";
/// let address = key_to_public_ec_address(key).unwrap();
/// assert_eq!(address, "EC3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFq".to_string());
/// ```
pub fn key_to_public_ec_address(key: &str) -> AddressResult<String> {
  let key = decode_key(key)?;
  let address = key_to_address(EC_PUBLIC_KEY_PREFIX, key);
  Ok(address)
}

/// Convert a 32 byte hex key to a secret entry credit address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let key = "f8dd9d8af5d7cabd2b370be9f91bdd9021acb2eb9feaa39a78375e31f978377a";
/// let address = key_to_secret_ec_address(key).unwrap();
/// assert_eq!(address, "Es4KG4EDfxT4bnNR7TUv45SpB8pbRnfq6hCv5dTr8hFAizV9isCr".to_string());
/// ```
pub fn key_to_secret_ec_address(key: &str) -> AddressResult<String> {
  let key = decode_key(key)?;
  let address = key_to_address(EC_SECRET_KEY_PREFIX, key);
  Ok(address)
}

/// Convert a 32 byte hex key to a secret factoid address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let key = "f8dd9d8af5d7cabd2b370be9f91bdd9021acb2eb9feaa39a78375e31f978377a";
/// let address = key_to_secret_fct_address(key).unwrap();
/// assert_eq!(address, "Fs3D7FNgvCV6P3bmFnJggAE3uQ9ykQufwzwSKgbrzQtjeiPnZW6g".to_string());
/// ```
pub fn key_to_secret_fct_address(key: &str) -> AddressResult<String> {
  let key = decode_key(key)?;
  let address = key_to_address(FCT_SECRET_KEY_PREFIX, key);
  Ok(address)
}

/// Convert a 32 byte hex key to a public factoid address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let key = "f8dd9d8af5d7cabd2b370be9f91bdd9021acb2eb9feaa39a78375e31f978377a";
/// let address = key_to_public_fct_address(key).unwrap();
/// assert_eq!(address, "FA36DMDRdHVpu6hi3CRNT2jz3yhURzb2bZ7oncosgqJTtga6ZNzY".to_string());
/// ```
pub fn key_to_public_fct_address(key: &str) -> AddressResult<String> {
  let rcd = &key_to_rcd_hash(&key)?;
  let address = rcd_to_public_fct_address(rcd)?;
  Ok(address)
}

/// Convert a 32 byte hex RCD hash to a public factoid address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let key = "93e53f898231d8de2d36cca46cea6da5e419ee27fbd84b781220668857893b74";
/// let address = rcd_to_public_fct_address(key).unwrap();
/// assert_eq!(address, "FA36DMDRdHVpu6hi3CRNT2jz3yhURzb2bZ7oncosgqJTtga6ZNzY".to_string());
/// ```
pub fn rcd_to_public_fct_address(rcd: &str) -> AddressResult<String> {
  let rcd = decode_key(rcd)?;
  let address = key_to_address(FCT_PUBLIC_KEY_PREFIX, rcd);
  Ok(address)
}

/// Convert an EC address or private FCT address into key. 
/// 
/// **Note**: cannot convert public FCT address to key. Use address_to_rcd_hash() 
/// to get public FCT address RCD hash.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_ec_addr = "EC3P1v9UpTMGjng8qbMiDcrk3VpJZGqW8bEPshKvcqRr3GCyPQna";
/// let pub_ec_key = address_to_key(pub_ec_addr).unwrap();
/// assert_eq!(pub_ec_key, "d521851f839bd859dddf8f47583e45b625fcf69d3fc6081234cd889227d6edfd".to_string());
/// 
/// let sec_ec_addr = "Es2WLtdNiTuBmA3Z7nUFPSK4AVzZFbU93LPmR7B1JK9Xjcfxt7YC";
/// let sec_ec_key = address_to_key(sec_ec_addr).unwrap();
/// assert_eq!(sec_ec_key, "0aa3bd0926fc64285abfdc1dab15861fd5c406cb6a6a966752555a738d8d99c3".to_string());
/// 
/// let sec_fct_addr = "Fs1QC5mqxhwDYRGuG7J21X6HtmKwaDhyte8HfAK2A2o6fLYF4ite";
/// let sec_fct_key = address_to_key(sec_fct_addr).unwrap();
/// assert_eq!(sec_fct_key, "0aa3bd0926fc64285abfdc1dab15861fd5c406cb6a6a966752555a738d8d99c3".to_string());
/// ```
pub fn address_to_key(addr: &str) -> AddressResult<String> {
  if !is_valid_address(addr) {
    let err = format!("Address must be valid, received: {}", addr);
    return Err(AddressError::InvalidAddress(err));
  };

  if &addr[..2] == "FA" {
    let err = format!("Address must not be public FCT address, received: {}", addr);
    return Err(AddressError::InvalidAddress(err));
  };

  let key_slice = &bs58::decode(addr).into_vec().unwrap()[2..34];
  Ok(hex::encode(key_slice))
}

/// Convert a public FCT address to an RCD hash.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_fct_addr = "FA2UCJVA3Cpp665LkWWTPaRDEYLMraQsbqgbaa5muT7PYzVhWXMU";
/// let rcd_hash = address_to_rcd_hash(pub_fct_addr).unwrap();
/// assert_eq!(rcd_hash, "421cb2fea1db8767b3d77e269e59de330178e5dd978ca942604152c33c9566b7".to_string());
/// ```
pub fn address_to_rcd_hash(addr: &str) -> AddressResult<String> {
  if !is_valid_pub_fct_address(addr) {
    let err = format!("Address must be a valid public FCT address, received: {}", addr);
    return Err(AddressError::InvalidAddress(err));
  };

  let rcd_slice = &bs58::decode(addr).into_vec().unwrap()[2..34];
  Ok(hex::encode(rcd_slice))
}

/// Get a public address from a secret address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let sec_ec_addr = "Es2WLtdNiTuBmA3Z7nUFPSK4AVzZFbU93LPmR7B1JK9Xjcfxt7YC";
/// let pub_ec_addr = secret_to_public_address(sec_ec_addr);
/// assert_eq!(pub_ec_addr.unwrap(), "EC3P1v9UpTMGjng8qbMiDcrk3VpJZGqW8bEPshKvcqRr3GCyPQna".to_string());
/// 
/// let sec_fct_addr = "Fs1QC5mqxhwDYRGuG7J21X6HtmKwaDhyte8HfAK2A2o6fLYF4ite";
/// let pub_fct_addr = secret_to_public_address(sec_fct_addr);
/// assert_eq!(pub_fct_addr.unwrap(), "FA2UCJVA3Cpp665LkWWTPaRDEYLMraQsbqgbaa5muT7PYzVhWXMU".to_string());
/// ```
pub fn secret_to_public_address(addr: &str) -> AddressResult<String> {
  if !is_valid_secret_address(addr) {
    let err = format!("{} is not a valid secret address.", addr);
    return Err(AddressError::InvalidAddress(err));
  };

  // Safe to use unwrap as address has already been valdiated.
  let secret_bytes = &bs58::decode(addr).into_vec().unwrap()[2..34];
  let secret_key = SecretKey::from_bytes(secret_bytes).unwrap();
  let public_key = PublicKey::from_secret::<Sha512>(&secret_key);
  let public_hex = hex::encode(public_key.as_bytes());

  if addr.chars().next().unwrap() == 'F' {
    Ok(key_to_public_fct_address(&public_hex)?)
  } else {
    Ok(key_to_public_ec_address(&public_hex)?)
  }
}

/// Validate any type of Factom address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_ec_addr = "EC3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFq";
/// assert!(is_valid_address(pub_ec_addr));
/// 
/// let sec_ec_addr = "Es4KG4EDfxT4bnNR7TUv45SpB8pbRnfq6hCv5dTr8hFAizV9isCr";
/// assert!(is_valid_address(sec_ec_addr));
/// 
/// let pub_fct_addr = "FA36DMDRdHVpu6hi3CRNT2jz3yhURzb2bZ7oncosgqJTtga6ZNzY";
/// assert!(is_valid_address(pub_fct_addr));
/// 
/// let sec_fct_addr = "Fs3D7FNgvCV6P3bmFnJggAE3uQ9ykQufwzwSKgbrzQtjeiPnZW6g";
/// assert!(is_valid_address(sec_fct_addr));
/// ```
pub fn is_valid_address(addr: &str) -> bool {
  let address =  match bs58::decode(addr).into_vec() {
    Ok(addr) => addr,
    Err(_) => return false
  };

  if address.len() != 38 {
    return false;
  };

  let mut prefix = [0u8; 2];
  prefix.copy_from_slice(&address[..2]);
  if !VALID_PREFIXES.contains(&prefix) {
    return false;
  };

  let checksum = &sha256d(&address[..34])[..4];
  if checksum != &address[34..38] {
    return false;
  }

  true
}

/// Validate a public Factom address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let address = "EC3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFq";
/// assert!(is_valid_public_address(address));
/// 
/// let pub_fct_addr = "FA36DMDRdHVpu6hi3CRNT2jz3yhURzb2bZ7oncosgqJTtga6ZNzY";
/// assert!(is_valid_public_address(pub_fct_addr));
/// ```
pub fn is_valid_public_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_PUB_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate a secret Factom address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let sec_ec_addr = "Es4KG4EDfxT4bnNR7TUv45SpB8pbRnfq6hCv5dTr8hFAizV9isCr";
/// assert!(is_valid_secret_address(sec_ec_addr));
/// 
/// let sec_fct_addr = "Fs3D7FNgvCV6P3bmFnJggAE3uQ9ykQufwzwSKgbrzQtjeiPnZW6g";
/// assert!(is_valid_secret_address(sec_fct_addr));
/// ```
pub fn is_valid_secret_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_SEC_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate a Factoid address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_fct_addr = "FA36DMDRdHVpu6hi3CRNT2jz3yhURzb2bZ7oncosgqJTtga6ZNzY";
/// assert!(is_valid_fct_address(pub_fct_addr));
/// 
/// let sec_fct_addr = "Fs3D7FNgvCV6P3bmFnJggAE3uQ9ykQufwzwSKgbrzQtjeiPnZW6g";
/// assert!(is_valid_fct_address(sec_fct_addr));
/// ```
pub fn is_valid_fct_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_FCT_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate an Entry Credit address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_ec_addr = "EC3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFq";
/// assert!(is_valid_ec_address(pub_ec_addr));
/// 
/// let sec_ec_addr = "Es4KG4EDfxT4bnNR7TUv45SpB8pbRnfq6hCv5dTr8hFAizV9isCr";
/// assert!(is_valid_ec_address(sec_ec_addr));
/// ```
pub fn is_valid_ec_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_EC_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate a public Entry Credit address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_ec_addr = "EC3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFq";
/// assert!(is_valid_pub_ec_address(pub_ec_addr));
/// ```
pub fn is_valid_pub_ec_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_EC_PREFIXES.contains(&prefix) || !VALID_PUB_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate a secret Entry Credit address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let sec_ec_addr = "Es4KG4EDfxT4bnNR7TUv45SpB8pbRnfq6hCv5dTr8hFAizV9isCr";
/// assert!(is_valid_sec_ec_address(sec_ec_addr));
/// ```
pub fn is_valid_sec_ec_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_EC_PREFIXES.contains(&prefix) || !VALID_SEC_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate a public Factoid address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let pub_fct_addr = "FA36DMDRdHVpu6hi3CRNT2jz3yhURzb2bZ7oncosgqJTtga6ZNzY";
/// assert!(is_valid_pub_fct_address(pub_fct_addr));
/// ```
pub fn is_valid_pub_fct_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_FCT_PREFIXES.contains(&prefix) || !VALID_PUB_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}

/// Validate a secret Factoid address.
/// 
/// # Example
/// ```rust
/// use factom::address::*;
/// 
/// let sec_fct_addr = "Fs3D7FNgvCV6P3bmFnJggAE3uQ9ykQufwzwSKgbrzQtjeiPnZW6g";
/// assert!(is_valid_sec_fct_address(sec_fct_addr));
/// ```
pub fn is_valid_sec_fct_address(addr: &str) -> bool {
  let prefix = &addr[..2];
  if !VALID_FCT_PREFIXES.contains(&prefix) || !VALID_SEC_PREFIXES.contains(&prefix) {
    return false;
  };
  is_valid_address(addr)
}


#[cfg(test)]
mod tests {
  use super::*;

  const PUB_KEY: &str = "d521851f839bd859dddf8f47583e45b625fcf69d3fc6081234cd889227d6edfd";
  const PUB_EC_ADDR: &str = "EC3P1v9UpTMGjng8qbMiDcrk3VpJZGqW8bEPshKvcqRr3GCyPQna";
  const PUB_FCT_ADDR: &str = "FA2UCJVA3Cpp665LkWWTPaRDEYLMraQsbqgbaa5muT7PYzVhWXMU";
  const RCD_HASH: &str = "421cb2fea1db8767b3d77e269e59de330178e5dd978ca942604152c33c9566b7";

  const SEC_KEY: &str = "0aa3bd0926fc64285abfdc1dab15861fd5c406cb6a6a966752555a738d8d99c3";
  const SEC_EC_ADDR: &str = "Es2WLtdNiTuBmA3Z7nUFPSK4AVzZFbU93LPmR7B1JK9Xjcfxt7YC";
  const SEC_FCT_ADDR: &str = "Fs1QC5mqxhwDYRGuG7J21X6HtmKwaDhyte8HfAK2A2o6fLYF4ite";

  const KEY_BAD_HEX: &str = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
  const KEY_BAD_LEN: &str = "cd89";

  const ADDR_BAD_BS58: &str = "0000000000000000000000000000000000000000000000000000";
  const ADDR_BAD_LEN: &str = "EC3e";
  const ADDR_BAD_PRE: &str = "EF3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFq";
  const ADDR_BAD_SUM: &str = "EC3ekiKnW3DpzqTzAEkSyrAvmC3mZ6zwrmEUmf7AfcVPJahnHQFs";

  #[test]
  fn key_to_pub_ec_addr() {
    assert_eq!(key_to_public_ec_address(PUB_KEY).unwrap(), PUB_EC_ADDR.to_string())
  }

  #[test]
  fn key_to_sec_ec_addr() {
    assert_eq!(key_to_secret_ec_address(SEC_KEY).unwrap(), SEC_EC_ADDR.to_string())
  }

  #[test]
  fn key_to_pub_fct_addr() {
    assert_eq!(key_to_public_fct_address(PUB_KEY).unwrap(), PUB_FCT_ADDR.to_string())
  }

  #[test]
  fn key_to_sec_fct_addr() {
    assert_eq!(key_to_secret_fct_address(SEC_KEY).unwrap(), SEC_FCT_ADDR.to_string())
  }

  #[test]
  fn key_to_fail_invalid_hex() -> std::result::Result<(), &'static str>{
    match key_to_secret_fct_address(KEY_BAD_HEX) {
      Ok(_) => Err("Function returned successfully"),
      Err(e) => match e {
        AddressError::FromHex(_) => Ok(()),
        _ => Err("Failed with incorrect error.")
      }
    }
  }

  #[test]
  fn key_to_fail_on_invalid_length() -> std::result::Result<(), &'static str>{
    match key_to_secret_fct_address(KEY_BAD_LEN) {
      Ok(_) =>  Err("Function returned successfully"),
      Err(e) => match e {
        AddressError::Length(_) => Ok(()),
        _ => Err("Failed with incorrect error.")
      }
    }
  }

  #[test]
  fn addr_to_key() {
    let sec_fct_key = address_to_key(SEC_FCT_ADDR);
    assert_eq!(sec_fct_key.unwrap(), SEC_KEY);
    
    let sec_ec_key = address_to_key(SEC_EC_ADDR);
    assert_eq!(sec_ec_key.unwrap(), SEC_KEY);

    let pub_ec_key = address_to_key(PUB_EC_ADDR);
    assert_eq!(pub_ec_key.unwrap(), PUB_KEY);
  }

  #[test]
  fn addr_to_key_fail() -> std::result::Result<(), &'static str> {
    match address_to_key(PUB_FCT_ADDR) {
      Ok(_) => Err("Should not convert public FCT address to key."),
      Err(_) => Ok(())
    }
  }

  #[test]
  fn addr_to_rcd_hash() {
    let rcd = address_to_rcd_hash(PUB_FCT_ADDR).unwrap();
    assert_eq!(rcd, RCD_HASH);
  }

  #[test]
  fn addr_to_rcd_hash_fail() -> std::result::Result<(), &'static str> {
    if let Ok(_) = address_to_rcd_hash(PUB_EC_ADDR) {
      return Err("Should not convert public EC address to RCD hash.");
    };

    if let Ok(_) = address_to_rcd_hash(SEC_EC_ADDR) {
      return Err("Should not convert sec EC address to RCD hash.");
    };

    if let Ok(_) = address_to_rcd_hash(SEC_FCT_ADDR) {
      return Err("Should not convert sec FCT address to RCD hash.");
    };

    Ok(())
  }

  #[test]
  fn sec_to_pub() {
    let pub_fct_addr = secret_to_public_address(SEC_FCT_ADDR);
    assert_eq!(pub_fct_addr.unwrap(), PUB_FCT_ADDR);

    let pub_fct_addr = secret_to_public_address(SEC_EC_ADDR);
    assert_eq!(pub_fct_addr.unwrap(), PUB_EC_ADDR);
  }


  #[test]
  fn validate_addresses() {
    assert!(is_valid_address(PUB_EC_ADDR));
    assert!(is_valid_address(SEC_EC_ADDR));
    assert!(is_valid_address(PUB_FCT_ADDR));
    assert!(is_valid_address(SEC_FCT_ADDR));
  }

  #[test]
  fn validate_addr_fail_on_bs58() {
    assert!(!is_valid_address(ADDR_BAD_BS58));
  }

  #[test]
  fn validate_addr_fail_on_len() {
    assert!(!is_valid_address(ADDR_BAD_LEN));
  }

  #[test]
  fn validate_addr_fail_on_prefix() {
    assert!(!is_valid_address(ADDR_BAD_PRE));
  }

  #[test]
  fn validate_addr_fail_on_sum() {
    assert!(!is_valid_address(ADDR_BAD_SUM));
  }

  #[test]
  fn validate_pub_addr() {
    // ok
    assert!(is_valid_public_address(PUB_EC_ADDR));
    assert!(is_valid_public_address(PUB_FCT_ADDR));
    //bad
    assert!(!is_valid_public_address(SEC_EC_ADDR));
    assert!(!is_valid_public_address(SEC_FCT_ADDR));
  }

  #[test]
  fn validate_sec_addr() {
    // Ok
    assert!(is_valid_secret_address(SEC_EC_ADDR));
    assert!(is_valid_secret_address(SEC_FCT_ADDR));
    // Bad
    assert!(!is_valid_secret_address(PUB_EC_ADDR));
    assert!(!is_valid_secret_address(PUB_FCT_ADDR));
  }

  #[test]
  fn validate_fct_addr() {
    // Ok
    assert!(is_valid_fct_address(SEC_FCT_ADDR));
    assert!(is_valid_fct_address(PUB_FCT_ADDR));
    // Bad
    assert!(!is_valid_fct_address(SEC_EC_ADDR));
    assert!(!is_valid_fct_address(PUB_EC_ADDR));
  }

  #[test]
  fn validate_ec_addr() {
    // Ok
    assert!(is_valid_ec_address(SEC_EC_ADDR));
    assert!(is_valid_ec_address(PUB_EC_ADDR));
    // Bad
    assert!(!is_valid_ec_address(SEC_FCT_ADDR));
    assert!(!is_valid_ec_address(PUB_FCT_ADDR));
  }

  #[test]
  fn validate_pub_ec_addr() {
    // Ok
    assert!(is_valid_pub_ec_address(PUB_EC_ADDR));
    // Bad
    assert!(!is_valid_pub_ec_address(SEC_EC_ADDR));
    assert!(!is_valid_pub_ec_address(PUB_FCT_ADDR));
    assert!(!is_valid_pub_ec_address(SEC_FCT_ADDR));
  }

  #[test]
  fn validate_sec_ec_addr() {
    // Ok
    assert!(is_valid_sec_ec_address(SEC_EC_ADDR));
    // Bad
    assert!(!is_valid_sec_ec_address(PUB_EC_ADDR));
    assert!(!is_valid_sec_ec_address(PUB_FCT_ADDR));
    assert!(!is_valid_sec_ec_address(SEC_FCT_ADDR));
  }

  #[test]
  fn validate_pub_fct_addr() {
    // Ok
    assert!(is_valid_pub_fct_address(PUB_FCT_ADDR));
    // Bad
    assert!(!is_valid_pub_fct_address(PUB_EC_ADDR));
    assert!(!is_valid_pub_fct_address(SEC_EC_ADDR));
    assert!(!is_valid_pub_fct_address(SEC_FCT_ADDR));
  }

  #[test]
  fn validate_sec_fct_addr() {
    // Ok
    assert!(is_valid_sec_fct_address(SEC_FCT_ADDR));
    // Bad
    assert!(!is_valid_sec_fct_address(PUB_EC_ADDR));
    assert!(!is_valid_sec_fct_address(PUB_FCT_ADDR));
    assert!(!is_valid_sec_fct_address(SEC_EC_ADDR));
  }
}