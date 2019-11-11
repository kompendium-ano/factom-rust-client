use super::{*, address::*};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use sha2::Sha512;
use hex;

impl Factom {
  /**
Create a new Entry Credit Address and store it in the wallet.
# Example
```
use factom::*;
let factom = Factom::new();
let query = factom
            .generate_ec_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub async fn generate_ec_address(self)
    -> Result<ApiResponse<Generate>>
  {
    let req =  ApiRequest::new("generate-ec-address");
    let response = self.walletd_call(req).await;
    parse(response).await
  }

/**
Create a new Entry Credit Address and store it in the wallet.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .generate_factoid_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub async fn generate_factoid_address(self)
    -> Result<ApiResponse<Generate>>
  {
    let req =  ApiRequest::new("generate-factoid-address");
    let response = self.walletd_call(req).await;
    parse(response).await
  }

/**
Creates a new identity key and adds it to the wallet. New keys are generated 
from the same mnemonic seed used for FCT and EC addresses. If the wallet is 
encrypted, it must be unlocked prior to using this command.

# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .generate_factoid_address()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub async fn generate_identity_key(self)
    -> Result<ApiResponse<Generate>>
    {
    let req =  ApiRequest::new("generate-identity-key");
    let response = self.walletd_call(req).await;
    parse(response).await
  }

}

/// Deserialises from generate-ec-address, generate-fct-address and 
/// generate-identity-key
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Generate {
  pub public: String,
  pub secret: String,
}

impl Generate {
  fn generate_local_keys() -> Keypair {
    let mut csprng: OsRng = OsRng::new().unwrap();
    Keypair::generate::<Sha512, _>(&mut csprng)
  }

  /// Generate a local public/secet FCT keypair.
  /// 
  /// # Warning
  /// Keys are NOT persisted to walletd. To generate and persist FCT keypair to walletd, 
  /// please use the `generate_factoid_address()` method on `Factom`.
  /// 
  /// # Example
  /// ```
  /// use factom::{generate::*, address::*};
  /// 
  /// let addresses = Generate::generate_local_factoid_addresses();
  /// assert!(is_valid_pub_fct_address(&addresses.public));
  /// assert!(is_valid_sec_fct_address(&addresses.secret));
  /// 
  /// let public_addr = secret_to_public_address(&addresses.secret).unwrap();
  /// assert_eq!(addresses.public, public_addr);
  /// ```
  pub fn generate_local_factoid_addresses() -> Self {
    let keys = Self::generate_local_keys();

    Self {
        public: key_to_public_fct_address(&hex::encode(keys.public.to_bytes())).unwrap(),
        secret: key_to_secret_fct_address(&hex::encode(keys.secret.to_bytes())).unwrap(),
    }
  }

  /// Generate a local public/secet EC keypair.
  /// 
  /// # Warning
  /// Keys are NOT persisted to walletd. To generate and persist EC keypair to walletd, 
  /// please use the `generate_ec_address()` method on `Factom`.
  /// 
  /// # Example
  /// ```
  /// use factom::{generate::*, address::*};
  /// 
  /// let addresses = Generate::generate_local_ec_addresses();
  /// assert!(is_valid_pub_ec_address(&addresses.public));
  /// assert!(is_valid_sec_ec_address(&addresses.secret));
  /// 
  /// let public_addr = secret_to_public_address(&addresses.secret).unwrap();
  /// assert_eq!(addresses.public, public_addr);
  /// ```
  pub fn generate_local_ec_addresses() -> Self {
    let keys = Self::generate_local_keys();

    Self {
        public: key_to_public_ec_address(&hex::encode(keys.public.to_bytes())).unwrap(),
        secret: key_to_secret_ec_address(&hex::encode(keys.secret.to_bytes())).unwrap(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gen_local_fct_addr() {
    let addresses = Generate::generate_local_factoid_addresses();
    assert!(is_valid_pub_fct_address(&addresses.public));
    assert!(is_valid_sec_fct_address(&addresses.secret));

    let public_addr = secret_to_public_address(&addresses.secret).unwrap();
    assert_eq!(addresses.public, public_addr);
  }

  #[test]
  fn gen_local_ec_addr() {
    let addresses = Generate::generate_local_ec_addresses();
    assert!(is_valid_pub_ec_address(&addresses.public));
    assert!(is_valid_sec_ec_address(&addresses.secret));

    let public_addr = secret_to_public_address(&addresses.secret).unwrap();
    assert_eq!(addresses.public, public_addr);
  }
}