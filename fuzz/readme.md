# Fuzzing suite 

You can fuzz both the api client library and the factomd/factom-walletd binaries 
with this tool. Setting up factomd for debug logging is highly recommended. 

To run a simulation of the factom network locally: 

```bash
factomd --count=32 --net=alot+ --enablenet=false --network=LOCAL  -journaling=true
```



