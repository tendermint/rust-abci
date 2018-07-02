//! This is the documentation for the rust-abci crate.

extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate tls_api;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate integer_encoding;

pub mod server;
pub mod types;

use types::*;

pub trait Application {

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

    // Miscellaneous connection
    fn echo(&mut self, p: &RequestEcho) -> ResponseEcho;

    fn flush(&mut self, p: &RequestFlush) -> ResponseFlush;
}
