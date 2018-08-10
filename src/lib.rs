//! This is the documentation for the rust-abci crate.
#![feature(ptr_internals)]
extern crate bytes;
extern crate protobuf;
extern crate integer_encoding;

mod tcpserver;
pub mod server;
pub mod types;

pub use types::*;

pub trait Application : Sync {

    // Info/Query connection
    fn info(&mut self, req: &RequestInfo) -> ResponseInfo;

    fn set_option(&mut self, req: &RequestSetOption) -> ResponseSetOption;

    fn query(&mut self, p: &RequestQuery) -> ResponseQuery;

    // Mempool connection
    fn check_tx(&mut self, p: &RequestCheckTx) -> ResponseCheckTx;

    // Consensus connection
    fn init_chain(&mut self, p: &RequestInitChain) -> ResponseInitChain;

    fn begin_block(&mut self, p: &RequestBeginBlock) -> ResponseBeginBlock;

    fn deliver_tx(&mut self, p: &RequestDeliverTx) -> ResponseDeliverTx;

    fn end_block(&mut self, p: &RequestEndBlock) -> ResponseEndBlock;

    fn commit(&mut self, p: &RequestCommit) -> ResponseCommit;
}