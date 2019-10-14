use super::*;

impl Factom{
  /**
Return its current balance for a specific entry credit address.
# Example
```
use factom::*;

let address = "EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx";
let factom = Factom::new();
let query = factom
            .entry_credit_balance(address)
            .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn entry_credit_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("address".to_string(), json!(address));
    self.call("entry-credit-balance", params)
  }

/**
This call returns the number of Factoshis (Factoids *10^-8) that are 
currently available at the address specified.
# Example
```
use factom::*;

let address = "FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q";
let factom = Factom::new();
let query = factom
      .factoid_balance(address)
      .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
*/
  pub fn factoid_balance(self, address: &str)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("address".to_string(), json!(address));
    self.call("factoid-balance", params)
  }

/**
The multiple-ec-balances API is used to query the acknowledged and saved 
balances for a list of entry credit addresses.

* currentheight is the current height that factomd was loading.
* lastsavedheight is the height last saved to the database.

* In balances it returns "ack", "saved" and "err".
  * ack is the balance after processing any in-flight transactions known to 
  the Factom node responding to the API call
  * saved is the last saved to the database
  * err is just used to display any error that might have happened during the 
  request. If it is empty that means there was no error.

* If the syntax of the parameters is off e.g. missing a quote, a comma, or a 
square bracket, it will return: `{“jsonrpc”:“2.0”,“id”:null,“error”:
{“code”:-32600,“message”:“Invalid Request”}}`

* If the parameters are labeled incorrectly the call will return: 
`{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed 
in, expected addresses”}`

* If factomd is not loaded up all the way to the last saved block it will 
return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,“saved”:0,
“err”:“Not fully booted”}]}`

* If the list of addresses contains an incorrectly formatted address the call 
will return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:[{“ack”:0,
“saved”:0,“err”:“Error decoding address”}]}`

* If an address in the list is valid but has never been part of a transaction 
the call will return: `“balances”:[{“ack”:0,“saved”:0,“err”:“Address has not 
had a transaction”}]`
# Example
```
use factom::*;

let addresses: Vec<&str> = 
vec!["EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx"];
let factom = Factom::new();
let query = factom.multiple_ec_balances(addresses)
            .map(|response| response).map_err(|err| err);
let result = fetch(query);
let response = result.unwrap();
assert!(response.success());   
```
*/
  pub fn multiple_ec_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("addresses".to_string(), json!(addresses));
    self.call("multiple-ec-balances", params)
  }

/**
The multiple-fct-balances API is used to query the acknowledged and saved 
balances in factoshis (a factoshi is 10^8 factoids) not factoids(FCT) for a 
list of FCT addresses.

* currentheight is the current height that factomd was loading.
* lastsavedheight is the height last saved to the database.

* In balances it returns "ack", "saved" and "err".
  * ack is the balance after processing any in-flight transactions known to 
  the Factom node responding to the API call
  * saved is the last saved to the database
  * err is just used to display any error that might have happened during the 
  request. If it is "" that means there was no error.

* If the syntax of the parameters is off e.g. missing a quote, a comma, or a 
square bracket, it will return: `{”jsonrpc”:“2.0”,“id”:null,“error”:
{“code”:-32600,“message”:“Invalid Request”}}`

* If the parameters are labeled incorrectly the call will return: `
{“code”:-32602,“message”:“Invalid params”,“data”:“ERROR! Invalid params passed in, expected 'addresses’”}`

* If factomd is not loaded up all the way to the last saved block it will 
return: `{“currentheight”:0,“lastsavedheight”:0,“balances”:
[{“ack”:0,“saved”:0,“err”:“Not fully booted”}]}`

* If the list of addresses contains an incorrectly formatted address the call 
will return: `{“currentheight”:0,“lastsavedheight”:0,
“balances”:[{“ack”:0,“saved”:0,“err”:“Error decoding address”}]}`

* If an address in the list is valid but has never been part of a transaction 
it will return: `“balances”:[{“ack”:0,“saved”:0,“err”:“Address has not had a 
transaction”}]`
# Example
```
use factom::*;

let addresses: Vec<&str> = vec!["FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q"];
let factom = Factom::new();
let query = factom.multiple_fct_balances(addresses)
            .map(|response| response).map_err(|err| err);
let result = fetch(query);
let response = result.unwrap();
assert!(response.success());   
```
*/
  pub fn multiple_fct_balances(self, addresses: Vec<&str>)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("addresses".to_string(), json!(addresses));
    self.call("multiple-fct-balances", params)
  }
}
