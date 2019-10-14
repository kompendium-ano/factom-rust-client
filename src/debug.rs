use super::*;

impl Factom {

/**
Show current holding messages in the queue.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .holding_queue()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn holding_queue(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("holding-queue", HashMap::new())
  }

/**
Get information on the current network factomd is connected to (TEST, MAIN, etc)


#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .network_info()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn network_info(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("network-info", HashMap::new())
  }

/**
Get the predicted future entry credit rate.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .predictive_fer()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn predictive_fer(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("predictive-fer", HashMap::new())
  }

/**
Get a list of the current network audit servers along with their information.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .audit_servers()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn audit_servers(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("audit-servers", HashMap::new())
  }

/**
Get a list of the current network federated servers along with their information.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .federated_servers()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn federated_servers(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("federated-servers", HashMap::new())
  }

/**
Get the current configuration state from factomd.conf.

NOTE: If a tag is commented out, this call will return the default value for it.
E.g: In the Example Response “ExchangeRate” is set to “0”. factomd.config default
 does not have an “ExchangeRate” tag. That is why it is set to “0”.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .configuration()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn configuration(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("configuration", HashMap::new())
  }

/**

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .process_list()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn process_list(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("process-list", HashMap::new())
  }

/**
List of authority servers in the management chain.
Get the process list known to the current factomd instance.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .authorities()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn authorities(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("authorities", HashMap::new())
  }

/**
Causes factomd to re-read the configuration from the config file. Note: This 
may cause consensus problems and could be impractical even in testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .reload_configuration()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn reload_configuration(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("reload-configuration", HashMap::new())
  }

/**
Get the current package drop rate for network testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .drop_rate()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn drop_rate(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("drop-rate", HashMap::new())
  }

/**
Change the network drop rate for testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .set_drop_rate(10)
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn set_drop_rate(self, drop_rate: usize)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("DropRate".to_string(), json!(drop_rate)); 
    self.debug_call("set-drop-rate", params)
  }

/**
Get the current msg delay time for network testing.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .delay()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn delay(self)-> impl Future<Item=Response, Error=FetchError>{
    self.debug_call("delay", HashMap::new())
  }

/**
Set the current msg delay time for network testing.
#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .set_delay(10)
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn set_delay(self, delay: usize)-> impl Future<Item=Response, Error=FetchError>{
    let mut params = HashMap::new();
    params.insert("Delay".to_string(), json!(delay)); 
    self.debug_call("set-delay", params)
  }

  /**
Get the nodes summary string.

#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .summary()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn summary(self)-> impl Future<Item=Response, Error=FetchError>{ 
    self.debug_call("summary", HashMap::new())
  }

  /**
Show current holding messages in the queue.
#Example
```
use factom::*;

let factom = Factom::new();
let query = factom
    .messages()
    .map(|response| response).map_err(|err| err);
let response = fetch(query).unwrap();
assert!(response.success());  
```
 */
  pub fn messages(self)-> impl Future<Item=Response, Error=FetchError>{ 
    self.debug_call("messages", HashMap::new())
  }
}

