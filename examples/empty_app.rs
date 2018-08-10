extern crate abci;
use abci::*;

struct EmptyApp;

impl Application for EmptyApp {

    // Info/Query connection
    fn info(&mut self, req: &RequestInfo) -> ResponseInfo {
        ResponseInfo::new()
    }

    fn set_option(&mut self, req: &RequestSetOption) -> ResponseSetOption {
        ResponseSetOption::new()
    }

    fn query(&mut self, p: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }

    // Mempool connection
    fn check_tx(&mut self, p: &RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    // Consensus connection
    fn init_chain(&mut self, p: &RequestInitChain) -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn begin_block(&mut self, p: &RequestBeginBlock) -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    fn deliver_tx(&mut self, p: &RequestDeliverTx) -> ResponseDeliverTx {
        ResponseDeliverTx::new()
    }

    fn end_block(&mut self, p: &RequestEndBlock) -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    fn commit(&mut self, p: &RequestCommit) -> ResponseCommit {
        ResponseCommit::new()
    }
}

fn main() {
    static APP: EmptyApp = EmptyApp;
    let addr = "127.0.0.1:26658".parse().unwrap();
    server::new(addr, &APP);
}