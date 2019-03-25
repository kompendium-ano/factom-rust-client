# Factom client test environment

To run through tests a custom blockchain with prior entries and credits is needed, which is included in the tests/env folder. The .factom folder there containing the database needs to be moved to ~/.factom beforehand and the daemon needs additional parameters as specified below. The current blocktime is set at 100 seconds in factomd.conf and can be modified as needed.

#### Installing command line binaries
```bash
wget https://github.com/FactomProject/distribution/releases/download/v6.1.0/factom-amd64.deb 
dpkg -i factom-amd64.deb
rm factom-amd64.deb
```

#### Start custom factomd testnet and walletd
```bash
factomd -network=CUSTOM -customnet="cargo-test" -exclusive=true

# Open walletd in seperate shell
factom-walletd
```


#### CLI commands used in creating test environment
For replication purposes only, not necessary to call.
```bash

factom-cli importaddress Fs3E9gV6DXsYzf7Fqx1fVBQPQXV695eP3k5XbmHEZVRLkMdD9qCK

factom-cli importaddress Es2r45VdEdf34jBrA2zSeiQQKuH8sP9xzCsSBzLE68pB2KuhjTBn

factom-cli buyec FA2jK2HcLnRdS94dEcU27rF3meoJfpUcZPSinpb7AwQvPRY6RL1Q EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx 10000

echo "A test chain" | factom-cli addchain -n "rust api client testing" EC3EAsdwvihEN3DFhGJukpMS4aMPsZvxVvRSqyz5jeEqRVJMDDXx

```