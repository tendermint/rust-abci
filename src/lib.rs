extern crate bytes;
extern crate integer_encoding;
extern crate protobuf;

pub mod server;
mod tcpserver;
pub mod types;

pub use types::*;

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
}
