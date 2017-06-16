# Rust ABCI
A rust implementation of the ABCI protocol for Tendermint.

[![](https://tokei.rs/b1/github/tendermint/rust-abci)](https://github.com/tendermint/rust-abci)

### Join the chat!
[![](https://img.shields.io/badge/slack-join%20chat-brightgreen.svg)](http://forum.tendermint.com:3000/)

We have a friendly community of like-minded people which are always eager to help someone in need of advice or just
looking for casual banter.

### Code of Conduct
Please read, understand and adhere to our [code of conduct](https://github.com/tendermint/rust-abci/blob/develop/CODE_OF_CONDUCT.md).


----


## About rust-abci
This library implements the ABCI protocol and can be used to write ABCI applications for Tendermint in rust.
Here you can find more information about [Tendermint](https://github.com/tendermint/tendermint) and [ABCI application](https://github.com/tendermint/abci).

For a real life example of an ABCI application you can checkout [Ethermint](https://github.com/tendermint/ethermint) or [Basecoin](https://github.com/tendermint/basecoin).


----


## Installation
To test the examples, please clone this repository.
```bash
git clone git@github.com:tendermint/rust-abci.git
```
Please have a look at the dummy or counter app inside `src/bin` to get a feeling for how this server works.

To use this library to build your own ABCI apps in rust you have to include the following in your `Cargo.toml` file.
```bash
[dependencies]
rust-abci = { git = "git@github.com:tendermint/rust-abci.git" }
```

## Running the examples

### Tendermint
To run either of the example apps you have to have Tendermint installed and initialised. Please install it according to these [instructions](https://github.com/tendermint/tendermint). You can also check out [Ethermint](https://github.com/tendermint/ethermint/tree/develop) for an in-depth explanation of how ABCI apps work.
Rememeber to run tendermint with the `--abci grpc` flag like so.
```bash
tendermint node --abci grpc
```

### Examples
Once the Tendermint instance is up and running you can start either of the examples like this. Of course, please rememeber to
switch into the rust-abci folder.
```bash
cargo run --bin dummy

cargo run --bin counter
```

## Documentation
// TODO - publish to rust docs

// TODO - give quick examples

## Dependencies
The message types for the ABCI protocol are defined in a protobuf file, which can be copied from [here](https://github.com/tendermint/abci/blob/master/types/types.proto).

We use [rust-protobuf](https://github.com/stepancheg/rust-protobuf) to generate compatible rust files from the protobuf
source files. For convenience the generated rust files are checked into git and will be packaged with the library. If you
want to generate them yourself you can.
Install these two tools.
```bash
cargo install protobuf
cargo install grpc-compiler
```

```bash
protoc --rust_out src/ types.proto
protoc --rust-grpc_out src/ types.proto
```
