# Factom client test environment

Initial setup of the wallet environment is required to run many of the tests.
Either run the test-setup script located in this folder or the below
commands with factom-walletd running
```bash
# Import funded testnet address FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q
factom-cli importaddress Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK

# Import an EC address EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK
factom-cli importaddress Es3LS7zYa9DSzZuUC14HDpMinehmzz61JG1XFY62rX5pVDenH8Pk

# Buy entry credits
factom-cli buyec FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK 100000

# Check balance
factom-cli balance EC2MJzCcHqYJyujnPzjitEaHhtEPVBhmEWUKkv4SVaaKeYcq3fqK
```

Some functions are multipart and are dependant on other calls being made, eg. 
Sending a transaction or creating a chain. 
For these full examples can be found in the examples folder. 

