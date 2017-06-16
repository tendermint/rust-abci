# Rust ABCI
A rust implementation of the ABCI protocol for Tendermint.

## About rust-abci
This library implements the ABCI protocol and can be used to write ABCI applications for Tendermint in rust.
Here you can find more information about [Tendermint](https://github.com/tendermint/tendermint) and [ABCI application](https://github.com/tendermint/abci).

For a real life example of an ABCI application you can checkout [Ethermint](https://github.com/tendermint/ethermint) or [Basecoin](https://github.com/tendermint/basecoin).

## Installation
// TODO - explain how to use this library in writing your own ABCI app
// TODO - publish to crates.io

## Documentation
// TODO - publish to rust docs
// TODO - give quick examples

## Contributing
The message types for the ABCI protocol are defined in a proto buff file, which can be copied from [here](https://github.com/tendermint/abci/blob/master/types/types.proto).

We use [rust-protobuf](https://github.com/stepancheg/rust-protobuf) to generate compatible rust files from the protobuf
source files. For convenience the generated rust files are checked into git and will be packaged with the library. If you
want to generate them yourself you can.
```bash
protoc --rust_out src/ src/types.proto
```
