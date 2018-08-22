use std::net::SocketAddr;

extern crate bytes;
extern crate integer_encoding;
extern crate protobuf;

mod server;
pub mod types;

use server::TCPServer;
pub use types::*;

pub trait Application {
    fn info(&mut self, _req: &RequestInfo) -> ResponseInfo {
        ResponseInfo::new()
    }

    fn set_option(&mut self, _req: &RequestSetOption) -> ResponseSetOption {
        ResponseSetOption::new()
    }

    fn query(&mut self, _req: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }

    // Mempool connection
    fn check_tx(&mut self, _req: &RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    // Consensus connection
    fn init_chain(&mut self, _req: &RequestInitChain) -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn begin_block(&mut self, _req: &RequestBeginBlock) -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    fn end_block(&mut self, _req: &RequestEndBlock) -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    fn commit(&mut self, _req: &RequestCommit) -> ResponseCommit {
        ResponseCommit::new()
    }

    fn deliver_tx(&mut self, p: &RequestDeliverTx) -> ResponseDeliverTx;
}

pub fn run<A>(listen_addr: SocketAddr, app: A)
where
    A: Application + 'static + Send + Sync,
{
    let server = TCPServer::new(app, listen_addr);
    server.serve().unwrap();
}
