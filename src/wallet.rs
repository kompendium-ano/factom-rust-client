use super::*;

impl Factom {
  /**
Return the wallet seed and all addresses in the wallet for backup and offline 
storage.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .wallet_backup()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
    pub fn wallet_backup(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("wallet-backup", HashMap::new())
  } 

/**
The wallet-balances API is used to query the acknowledged and saved balances for 
all addresses in the currently running factom-walletd. The saved balance is the 
last saved to the database and the acknowledged or “ack” balance is the balance 
after processing any in-flight transactions known to the Factom node responding 
to the API call. The factoid address balance will be returned in factoshis 
(a factoshi is 10^8 factoids) not factoids(FCT) and the entry credit balance 
will be returned in entry credits.

* If walletd and factomd are not both running this call will not work.

* If factomd is not loaded up all the way to last saved block it will 
return: “result”:{“Factomd Error”:“Factomd is not fully booted, please 
wait and try again.”}

* If an address is not in the correct format the call will return: 
“result”:{“Factomd Error”:”There was an error decoding an address”}

* If an address does not have a public and private address known to the wallet 
it will not be included in the balance.

* "fctaccountbalances" are the total of all factoid account balances returned 
in factoshis.

* "ecaccountbalances" are the total of all entry credit account balances 
returned in entry credits.
# Example
```
use factom::*;

let factom = Factom::new();
let query = factom
            .wallet_balances()
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn wallet_balances(self)-> impl Future<Item=Response, Error=FetchError>{
    self.walletd_call("wallet-balances", HashMap::new())
  } 
}