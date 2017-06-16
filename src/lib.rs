//! This is the documentation for the rust-abci crate.
//! The crate is build with socket support by default.
//! In order to use grpc, please use `--feature "grpc_support".

extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;
extern crate websocket;

pub mod types;
#[cfg(feature="grpc_support")]
pub mod types_grpc;
#[cfg(feature="grpc_support")]
pub mod grpc_server;
#[cfg(feature="socket_support")]
pub mod socket_server;
