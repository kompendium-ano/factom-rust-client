# Fuzzing suite 

This fuzzing suite uses a rust implementation of American Fuzzy Lop, you can find more information about the underlying system here: http://lcamtuf.coredump.cx/afl/

The library itself is located here: https://github.com/rust-fuzz/afl.rs

Note: It doesn't build on windows due to xdg. You might have luck with Windows Subsystem for Linux.

You can fuzz both the api client library and the factomd/factom-walletd binaries 
with this tool. Setting up factomd for debug logging is highly recommended as below. 

To run a simulation of the factom network locally: 

```bash
factomd --count=32 --net=alot+ --enablenet=false --network=LOCAL  -journaling=true
```

To get started: 

```bash
cargo install afl
```

Currently the only input is the send-raw-message api function which puts various messages into factomd for debugging purposes. You can find the documentation for send-raw-message
[here](https://docs.factom.com/api#send-raw-message) and [here](https://docs.rs/factom/2.0.0./factom)

