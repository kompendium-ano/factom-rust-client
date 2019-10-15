# Client test environment

Initial setup of the wallet environment is required to run many of the tests.
With factom-walletd running locally use the apporopriate "test-setup" script located in this folder.

Otherwise the commands are listed below and will work on all platforms as long as you have factom-cli installed. It can be downloaded from [here](TODO: link github.com/factom/distribution/releases).


```bash
# Import funded testnet address
factom-cli importaddress Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK

# Import an EC address
factom-cli importaddress Es3LS7zYa9DSzZuUC14HDpMinehmzz61JG1XFY62rX5pVDenH8Pk

# Buy entry credits with the public addresses of above
factom-cli buyec FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK 100000

# Check balance
factom-cli balance EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK
```
Some functions are multipart and are dependant on other calls being made, *eg*. 
Sending a transaction or creating a chain or identity. 

These interactions can be found in the examples folder but in general the creation functions have a compose part whih returns the information to commit and reveal functions. Transactions require an input, output and fee, then it must be sent.

# Contributing

All contributions are welcome, please fork this repo, create your own branch, and submit a PR. 
By making contributions you agree to them being published under the Apache 2.0 license.




