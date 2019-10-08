use super ::*;

impl Factom {
/**
Returns all of the identity key pairs that are currently stored in the wallet. 
If the wallet is encrypted, it must be unlocked prior to using this command.

TODO: Examples

*/
  pub fn all_id_keys(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("all-identity-keys", HashMap::new())
  }
/**
This command will return an identity’s set of public keys (in order of 
decreasing priority) that were active at a specific block, or at the most 
recent height if the "height" parameter is not included. This is useful for 
validating entries containing identity signatures (e.g. on identity attributes 
and endorsements), allowing you to tell if a given signature was created with 
a key that was valid at the time that the entry was published. Time is 
measured in directory blocks.

As an example, let’s say the identity at chain-id 
3b69dabe22c014af9a9bc9dfa7917ce4602a03579597ddf184d8de56702512ae signs an entry 
using their level-3 key idpub2GU1Pcax2PibH8hHZg58fKRiSJKQWQkWYkpmt7VH1jCXBgqp9w, 
and publishes it to the blockchain at height 163420 and then replaces that key 
one block later at height 163421. Even though the key is no longer valid at the 
highest block height, we can tell that it was valid at the time that the 
signature was created, so we can still trust that the entry is authentic. 
However, if someone then published another entry signed with the key that was 
just replaced, we will be able to tell that the signer key is no longer valid 
and that the entry shouldn’t be trusted.

If the wallet is encrypted, it must be unlocked prior to using this command.
*/
  pub fn active_id_keys(
    self,
    chain_id: &str,
    height: usize
  )-> impl Future<Item=Response, Error=FetchError>
  {
    let mut params = HashMap::new();
    params.insert("chainid".to_string(), json!(chain_id));
    params.insert("height".to_string(), json!(height));
    self.walletd_call("active-identity-keys", params)
  }

/**
 **Be careful using this function! Ensure that you have backups of important keys 
 before removing them.** Given an identity public key, this command deletes the 
 corresponding identity key pair from the wallet. Once executed, the user will 
 no longer be able to retrieve that key pair or sign attributes/endorsements 
 with the key pair from this wallet. If the wallet is encrypted, it must be 
 unlocked prior to using this command.
 */
  pub fn remove_id_key(
    self,
    public: &str
  )-> impl Future<Item=Response, Error=FetchError>
  {
    let mut params = HashMap::new();
    params.insert("public".to_string(), json!(public));
    self.walletd_call("remove-identity-key", params)
  }
/**
Given an identity public key as input, this command will respond with the 
corresponding public/private key pair from the wallet. If the desired identity 
key isn’t currently stored in the wallet, an error is returned to indicate this. 
If the wallet is encrypted, it must be unlocked prior to using this command.
 */
  pub fn id_key(
    self,
    public: &str
  )-> impl Future<Item=Response, Error=FetchError>
  {
    let mut params = HashMap::new();
    params.insert("public".to_string(), json!(public));
    self.walletd_call("identity-key", params)
  }
}