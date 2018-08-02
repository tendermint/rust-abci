# Rust ABCI
A rust implementation of the ABCI protocol for Tendermint. This has been heavily modified to be consistent and compliant with
Tendermint `0.22`. 

[![Build Status](https://travis-ci.org/tendermint/rust-abci.svg?branch=develop)](https://travis-ci.org/tendermint/rust-abci) [![](https://tokei.rs/b1/github/tendermint/rust-abci)](https://github.com/tendermint/rust-abci) [![](https://docs.rs/rust-abci/badge.svg)](https://docs.rs/rust-abci/0.1.0/rust_abci/)

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


## Dependencies
Make sure that you have Rust and Cargo installed. The easiest way is to follow the instructions on [rustup](https://rustup.rs/).


## Installation
To test the examples, please clone this repository.
```bash
git clone git@github.com:tendermint/rust-abci.git
```
The `echo_app` example, found under the `examples` folder, is a good demonstration/bare minimum foundation for a Rust ABCI app.

To use this library to build your own ABCI apps in rust you have to include the following in your `Cargo.toml` file.
```toml
[dependencies]
abci = "0.3.0"
```

## Running the examples

### Tendermint
To run either of the example apps you have to have Tendermint installed and initialised. Please install it according to these [instructions](https://github.com/tendermint/tendermint). After initializing and configuring the node, Tendermint can be run with:
```bash
tendermint node
```

If you wish to not create new blocks each second, make sure to add the `--consensus.create_empty_blocks=0` flag.

After the node is online, you can run the `echo_app` example using `cargo run --example echo_app`.


## Documentation
Documentation is coming soon.


----

## Credits
Original `rust-tsp` made by Adrian Brink. New `abci` package and fixes based on `rust-tsp` made by Jackson Lewis.
